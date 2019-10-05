/* When cross compiling, build.rs runs on the host, thus when compiling it
 * target = host. Use env vars listed in the links below for the actual target.
 *
 * https://doc.rust-lang.org/cargo/reference/environment-variables.html
 * https://doc.rust-lang.org/reference/conditional-compilation.html
 */

#[derive(Debug)]
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
pub struct Cfg<'a> {
    arch: Option<&'a str>,
    // Not including target_feature here,
    // it doesn't seem useful for an equality test.
    //
    // Usually you'll want to CfgTarget::new().target_feature.contains("....")
    os: Option<&'a str>,
    family: Option<&'a str>,
    env: Option<&'a str>,
    endian: Option<&'a str>,
    pointer_width: Option<&'a str>,
    vendor: Option<&'a str>,
}

#[derive(Copy, Clone)]
pub enum CfgBuilder<'a> {
    Uninitialized,
    Cond(Cfg<'a>),
}

fn check_same(target: String, other: Option<&'_ str>) -> bool {
    Some(target) == other.map(|x: &str| x.to_uppercase())
}
fn check_conflict(target: String, other: Option<&'_ str>) -> bool {
    None == other || check_same(target, other)
}

impl<'a> Cfg<'a> {
    pub fn arch_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_arch).into(), self.arch)
    }

    pub fn os_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_os).into(), self.os)
    }

    pub fn family_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_family).into(), self.family)
    }

    pub fn env_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_env).into(), self.env)
    }

    pub fn endian_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_endian).into(), self.endian)
    }

    pub fn pointer_width_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_pointer_width).into(), self.pointer_width)
    }

    pub fn vendor_conflict(&'a self, arg: &CfgTarget) -> bool {
        check_conflict((&*arg.target_vendor).into(), self.vendor)
    }

    pub fn check(&'a self, arg: &CfgTarget) -> bool {
        self.arch_conflict(arg)
            && self.os_conflict(arg)
            && self.family_conflict(arg)
            && self.env_conflict(arg)
            && self.endian_conflict(arg)
            && self.pointer_width_conflict(arg)
            && self.vendor_conflict(arg)
    }
}

impl<'a> CfgTarget {
    pub fn new() -> CfgTarget {
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
    // Same and conflict should differ by the None case.

    pub fn any(&self, args: &'a [Cfg<'a>]) -> bool {
        assert_eq!(args.is_empty(), false);
        args.iter().any(|other| other.check(self))
    }

    pub fn all(&self, args: &'a [Cfg<'a>]) -> bool {
        assert_eq!(args.is_empty(), false);
        args.iter().all(|other| other.check(self))
    }
}

impl Default for CfgTarget {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CfgBuilder<'a> {
    pub fn new() -> CfgBuilder<'a> {
        CfgBuilder::Uninitialized
    }

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
            CfgBuilder::Uninitialized => panic!("CfgBuilder uninitialized"), // reachable
            CfgBuilder::Cond(inner) => *inner,
        }
    }
}

impl<'a> Default for CfgBuilder<'a> {
    fn default() -> Self {
        Self::new()
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

        let target_info: CfgTarget = CfgTarget::new();
        let mut cond_a = CfgBuilder::new();
        let mut cond_b = CfgBuilder::new();
        let mut cond_c = CfgBuilder::new();
        let mut cond_d = CfgBuilder::new();

        let cond_ff_1 = cond_a
            .target_arch("gothic")
            .target_os("cathederal")
            .target_endian("big")
            .build();
        let cond_ff_2 = cond_b.target_arch("victorian").target_os("home").build();
        let cond_tt = cond_c.target_arch("i686").target_os("linux").build();
        let cond_ff_3 = cond_d
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
        CfgBuilder::new().build();
    }

    #[test]
    #[should_panic]
    fn test_any_invalid_input() {
        setup_test_env();
        let target_info: CfgTarget = CfgTarget::new();
        target_info.any(&[]); // I guess?
    }

    #[test]
    fn test_all() {
        setup_test_env();
        let target_info: CfgTarget = CfgTarget::new();
        let mut cond_a = CfgBuilder::new();
        let mut cond_b = CfgBuilder::new();
        let mut cond_c = CfgBuilder::new();
        let mut cond_d = CfgBuilder::new();

        let cond_ff_1 = cond_a
            .target_arch("gothic")
            .target_os("cathederal")
            .target_endian("big")
            .build();
        let cond_ff_2 = cond_b.target_arch("victorian").target_os("home").build();
        let cond_tt = cond_c.target_arch("i686").target_os("linux").build();
        let cond_ff_3 = cond_d
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
        let target_info: CfgTarget = CfgTarget::new();
        target_info.all(&[]); // I guess?
    }
}
