//! When cross compiling, build.rs runs on the host, thus when compiling it
//! target = host. Use env vars described in the links below for the actual target.
//!
//! * [cargo environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
//! * [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)

/// This is not even close to an exact replica of the cfg! macro.
/// None of the use cases involving nesting is supported.
/// Help supporting these cases is welcome if possible.
/// * `cfg!(all(any(...), ...))`
/// * `cfg!(not(any(...)))`
///
/// Supported cases are:
/// * `build_cfg!(any(...))
/// * `build_cfg!(not(target_* = "zzz", ...)`
/// * `build_cfg!(all(...))`
#[macro_export]
macro_rules! build_cfg {
    () => {};
    ( $e1:tt = $v1:tt $( , $e2:tt = $v2 : tt )* ) => {
        CfgTarget::default().check(&CfgBuilder::default().$e1($v1)$(. $e2($v2))*.build())
    };
    (any($e1:tt = $v1:tt $( , $e2:tt = $v2:tt )* ) ) => {
        [bar_cfg!($e1 = $v1)
          $(, bar_cfg!($e2 = $v2) )* ].iter().any(|x| *x)
    };
    (all($e1:tt = $v1:tt $( , $e2:tt = $v2:tt )* ) ) => {
        [bar_cfg!($e1 = $v1)
          $(, bar_cfg!($e2 = $v2) )* ].iter().all(|x| *x)
    };
    (not($e1:tt = $v1:tt $(, $e2:tt = $v2:tt )* ) ) => {
        ! build_cfg!($e1 = $v1 $(, $e2 = $v2)* )
    }
}

#[derive(Debug)]
/// Representation of the current compilation target
/// containing the conditional compilation target_* variables
/// as derived from CARGO_TARGET_CFG_* environment variables.
pub struct CfgTarget {
    pub target_arch: String,
    pub target_feature: String,
    pub target_os: String,
    pub target_family: String,
    pub target_env: String,
    pub target_endian: String,
    pub target_pointer_width: String,
    pub target_vendor: String,
}

#[derive(Copy, Clone)]
/// A set of target_* variables, to be checked against `CfgTaget`.
pub struct Cfg<'a> {
    arch: Option<&'a str>,
    // Not including target_feature here,
    // it doesn't seem useful for an equality test.
    //
    // Usually you'll want to CfgTarget::default().target_feature.contains("....")
    os: Option<&'a str>,
    family: Option<&'a str>,
    env: Option<&'a str>,
    endian: Option<&'a str>,
    pointer_width: Option<&'a str>,
    vendor: Option<&'a str>,
}

#[derive(Copy, Clone)]
/// Builder for Cfg.
pub enum CfgBuilder<'a> {
    // If Uninitialized still matches by the time we get to build() panic!
    // otherwise CfgBuilder::default().build() matches all targets.
    // The normal cfg! macros preclude this when called like `cfg!()` with the following:
    // error: macro requires a cfg-pattern as an argument
    Uninitialized,
    Cond(Cfg<'a>),
}

fn check_same(target: String, other: Option<&'_ str>) -> bool {
    Some(target) == other.map(|x: &str| x.to_uppercase())
}

fn check_no_conflict(target: String, other: Option<&'_ str>) -> bool {
    None == other || check_same(target, other)
}

impl<'a> Cfg<'a> {
    /// Return true if `CfgTarget` is not in conflict with `self`.
    pub fn check(&'a self, arg: &CfgTarget) -> bool {
        check_no_conflict((&*arg.target_arch).into(), self.arch)
            && check_no_conflict((&*arg.target_os).into(), self.os)
            && check_no_conflict((&*arg.target_family).into(), self.family)
            && check_no_conflict((&*arg.target_env).into(), self.env)
            && check_no_conflict((&*arg.target_endian).into(), self.endian)
            && check_no_conflict((&*arg.target_pointer_width).into(), self.pointer_width)
            && check_no_conflict((&*arg.target_vendor).into(), self.vendor)
    }
}

impl<'a> CfgTarget {
    /// Return true if Any of the `args` check true.
    pub fn any(&self, args: &'a [&Cfg<'a>]) -> bool {
        // precluded by the cfg! macro from being a question so we panic.
        assert_eq!(args.is_empty(), false);

        args.iter().any(|other| other.check(self))
    }

    /// Return `true` if All of the `args` check true.
    pub fn all(&self, args: &'a [&Cfg<'a>]) -> bool {
        // precluded by the cfg! macro from being a question so we panic.
        assert_eq!(args.is_empty(), false);

        args.iter().all(|other| other.check(self))
    }

    /// Return true if `target` is not in conflict with `self`.
    pub fn check(&self, target: &'a Cfg<'a>) -> bool {
        target.check(self)
    }
}

impl Default for CfgTarget {
    /// Builds a CfgTarget from the `CARGO_CFG_TARGET_*`
    /// [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
    fn default() -> Self {
        fn getenv(var: &'static str) -> String {
            std::env::var(var)
                .unwrap_or_else(|_| String::new())
                .to_uppercase()
        }
        CfgTarget {
            target_arch: getenv("CARGO_CFG_TARGET_ARCH"),
            target_feature: getenv("CARGO_CFG_TARGET_FEATURE"),
            target_os: getenv("CARGO_CFG_TARGET_OS"),
            target_family: getenv("CARGO_CFG_TARGET_FAMILY"),
            target_env: getenv("CARGO_CFG_TARGET_ENV"),
            target_endian: getenv("CARGO_CFG_TARGET_ENDIAN"),
            target_pointer_width: getenv("CARGO_CFG_TARGET_POINTER_WIDTH"),
            target_vendor: getenv("CARGO_CFG_TARGET_VENDOR"),
        }
    }
}

impl<'a> CfgBuilder<'a> {
    fn initialized(&'a mut self) -> &'a mut CfgBuilder {
        match *self {
            CfgBuilder::Uninitialized => {
                *self = CfgBuilder::Cond(Cfg {
                    arch: None,
                    os: None,
                    family: None,
                    env: None,
                    endian: None,
                    pointer_width: None,
                    vendor: None,
                });
                self
            }
            _ => self,
        }
    }

    pub fn target_arch(&'a mut self, arch: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => Self::target_arch(Self::initialized(self), arch),
            CfgBuilder::Cond(it) => {
                it.arch = Some(arch);
                self
            }
        }
    }

    pub fn target_os(&'a mut self, os: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => Self::target_os(Self::initialized(self), os),
            CfgBuilder::Cond(it) => {
                it.os = Some(os);
                self
            }
        }
    }

    pub fn target_family(&'a mut self, family: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => Self::target_family(Self::initialized(self), family),
            CfgBuilder::Cond(it) => {
                it.family = Some(family);
                self
            }
        }
    }

    pub fn target_env(&'a mut self, env: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => Self::target_env(Self::initialized(self), env),
            CfgBuilder::Cond(it) => {
                it.env = Some(env);
                self
            }
        }
    }

    pub fn target_endian(&'a mut self, endianess: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => Self::target_endian(Self::initialized(self), endianess),
            CfgBuilder::Cond(it) => {
                it.endian = Some(endianess);
                self
            }
        }
    }

    pub fn target_pointer_width(&'a mut self, pointer_width: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => {
                Self::target_pointer_width(Self::initialized(self), pointer_width)
            }
            CfgBuilder::Cond(it) => {
                it.pointer_width = Some(pointer_width);
                self
            }
        }
    }

    pub fn target_vendor(&'a mut self, vendor: &'a str) -> &mut CfgBuilder<'a> {
        match self {
            CfgBuilder::Uninitialized => Self::target_vendor(Self::initialized(self), vendor),
            CfgBuilder::Cond(it) => {
                it.vendor = Some(vendor);
                self
            }
        }
    }

    pub fn build(&self) -> Cfg {
        match self {
            // precluded by the cfg! macro from being a question
            // reachable here via CfgBuilder::default().build() so we want to panic.
            CfgBuilder::Uninitialized => panic!("CfgBuilder uninitialized"),

            CfgBuilder::Cond(inner) => *inner,
        }
    }
}

impl<'a> Default for CfgBuilder<'a> {
    fn default() -> Self {
        CfgBuilder::Uninitialized
    }
}

#[cfg(test)]
mod tests {
    use crate::{CfgBuilder, CfgTarget};

    fn setup_test_env() {
        /* Dummy values for running the tests outside of build.rs/cargo */
        std::env::set_var("CARGO_CFG_TARGET_ARCH", "i686");
        std::env::set_var("CARGO_CFG_TARGET_FEATURE", "avx");
        std::env::set_var("CARGO_CFG_TARGET_OS", "linux");
        std::env::set_var("CARGO_CFG_TARGET_FAMILY", "unix");
        std::env::set_var("CARGO_CFG_TARGET_ENV", "gnu");
        std::env::set_var("CARGO_CFG_TARGET_ENDIAN", "little");
        std::env::set_var("CARGO_CFG_TARGET_POINTER_WIDTH", "32");
        std::env::set_var("CARGO_CFG_TARGET_VENDOR", "unknown");
    }

    #[test]
    fn test_any() {
        setup_test_env();

        let target_info: CfgTarget = CfgTarget::default();
        let mut cond_a = CfgBuilder::default();
        let mut cond_b = CfgBuilder::default();
        let mut cond_c = CfgBuilder::default();
        let mut cond_d = CfgBuilder::default();

        let cond_ff_1 = &cond_a
            .target_arch("gothic")
            .target_os("cathederal")
            .target_endian("big")
            .build();
        let cond_ff_2 = &cond_b.target_arch("victorian").target_os("home").build();
        let cond_tt = &cond_c.target_arch("i686").target_os("linux").build();
        let cond_ff_3 = &cond_d
            .target_arch("i686")
            .target_os("linux")
            .target_endian("big")
            .build();

        assert_eq!(target_info.any(&[cond_ff_1]), false);
        assert_eq!(target_info.any(&[cond_ff_3]), false);
        assert_eq!(target_info.any(&[cond_tt]), true);
        assert_eq!(target_info.any(&[cond_ff_1, cond_tt]), true);
        assert_eq!(target_info.any(&[cond_ff_1, cond_tt]), true);
        assert_eq!(target_info.any(&[cond_ff_1, cond_tt, cond_ff_2]), true);
    }

    #[test]
    #[should_panic]
    fn test_invalid_build() {
        setup_test_env();
        CfgBuilder::default().build();
    }

    #[test]
    #[should_panic]
    fn test_any_invalid_input() {
        setup_test_env();
        let target_info: CfgTarget = CfgTarget::default();
        target_info.any(&[]); // I guess?
    }

    #[test]
    fn test_all() {
        setup_test_env();
        let target_info: CfgTarget = CfgTarget::default();
        let mut cond_a = CfgBuilder::default();
        let mut cond_b = CfgBuilder::default();
        let mut cond_c = CfgBuilder::default();
        let mut cond_d = CfgBuilder::default();

        let cond_ff_1 = &cond_a
            .target_arch("gothic")
            .target_os("cathederal")
            .target_endian("big")
            .build();
        let cond_ff_2 = &cond_b.target_arch("victorian").target_os("home").build();
        let cond_tt = &cond_c.target_arch("i686").target_os("linux").build();
        let cond_ff_3 = &cond_d
            .target_arch("i686")
            .target_os("linux")
            .target_endian("big")
            .build();

        assert_eq!(target_info.all(&[cond_ff_1]), false);
        assert_eq!(target_info.all(&[cond_tt]), true);
        assert_eq!(target_info.all(&[cond_ff_1]), false);
        assert_eq!(target_info.all(&[cond_ff_3]), false);
        assert_eq!(target_info.all(&[cond_ff_1, cond_tt]), false);
        assert_eq!(target_info.all(&[cond_tt, cond_ff_1]), false);
        assert_eq!(target_info.all(&[cond_ff_1, cond_tt]), false);
        assert_eq!(target_info.all(&[cond_ff_1, cond_tt, cond_ff_2]), false);
    }

    #[test]
    #[should_panic]
    fn test_all_invalid_input() {
        setup_test_env();
        let target_info: CfgTarget = CfgTarget::default();
        target_info.all(&[]); // I guess?
    }
}
