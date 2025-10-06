//! Font patterns - used to select fonts matching certain properties, or as a unique identifier
//! of a resolved font.

use super::{sys, FcErr};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

fn get_string<'a>(pat: PatternRef<'a>, ty: &CStr, idx: libc::c_int) -> Result<&'a CStr, FcErr> {
    let mut str = ptr::null();
    // SAFETY: PatternRef is guaranteed valid for its lifetime, as is CStr
    unsafe { sys::FcPatternGetString(pat.as_ptr(), ty.as_ptr(), idx, &mut str) }
        .res()
        // SAFETY: Assuming no error, `str` will have been filled with a valid C-string pointer
        .map(|_| unsafe { CStr::from_ptr(str) })
}

fn get_int(pat: PatternRef<'_>, ty: &CStr, idx: libc::c_int) -> Result<i32, FcErr> {
    let mut int: libc::c_int = 0;
    // SAFETY: Provided values valid is a precondition.
    unsafe { sys::FcPatternGetInteger(pat.as_ptr(), ty.as_ptr(), idx, &mut int) }
        .res()
        .map(|_| int as i32)
}

/// Type that may be a parameter of a loaded font
pub trait PatParam {
    /// The type of this parameter
    type Output<'a>;

    /// Given a pattern, attempt to get the value of the parameter for the N-th font that matches
    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr>;
}

/// File parameter
pub struct File(());

impl PatParam for File {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_FILE, idx as libc::c_int)
    }
}

/// Family parameter
pub struct Family(());

impl PatParam for Family {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_FAMILY, idx as libc::c_int)
    }
}

/// Full name parameter
pub struct FullName(());

impl PatParam for FullName {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_FULLNAME, idx as libc::c_int)
    }
}

/// Style parameter
pub struct Style(());

impl PatParam for Style {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_STYLE, idx as libc::c_int)
    }
}

/// Index parameter
pub struct Index(());

impl PatParam for Index {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_INDEX, idx as libc::c_int)
    }
}

/// Weight parameter
pub struct Weight(());

impl PatParam for Weight {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_WEIGHT, idx as libc::c_int)
    }
}

/// Width parameter
pub struct Width(());

impl PatParam for Width {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_WIDTH, idx as libc::c_int)
    }
}

/// Slant parameter
pub struct Slant(());

impl PatParam for Slant {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_SLANT, idx as libc::c_int)
    }
}

/// A borrowed reference to a [`Pattern`]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PatternRef<'a>(NonNull<sys::FcPattern>, PhantomData<&'a sys::FcPattern>);

impl<'a> PatternRef<'a> {
    /// # Safety
    ///
    /// The pointer must be valid for the lifetime of this reference, pointing to a valid
    /// [`sys::FcPattern`] that is not destroyed before the .
    pub unsafe fn from_raw(ptr: NonNull<sys::FcPattern>) -> PatternRef<'a> {
        PatternRef(ptr, PhantomData)
    }

    /// Upgrade this borrow into an owned reference
    pub fn upgrade(self) -> Pattern {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { Pattern::from_raw_borrowed(self.0) }
    }

    /// Convert into a raw pointer to the inner [`sys::FcPattern`] struct
    pub fn as_ptr(self) -> *mut sys::FcPattern {
        self.0.as_ptr()
    }

    /// Get a parameter from the font matching this pattern at the provided index. May fail if
    /// the provided index is out-of-range, or if the property wasn't loaded for this pattern.
    pub fn get<T: PatParam>(self, idx: usize) -> Result<T::Output<'a>, FcErr> {
        T::get(self, idx)
    }
}

/// An owned font pattern. This is a semi-opaque value used to select fonts matching certain
/// properties, or as a unique identifier of a resolved font.
#[derive(PartialEq, Eq, Hash)]
pub struct Pattern(NonNull<sys::FcPattern>);

impl Pattern {
    /// # Safety
    ///
    /// The pointer must point to a valid [`sys::FcPattern`].
    pub unsafe fn from_raw_borrowed(ptr: NonNull<sys::FcPattern>) -> Pattern {
        // SAFETY: Provided pointer guaranteed valid by precondition
        unsafe { sys::FcPatternReference(ptr.as_ptr()) };
        Pattern(ptr)
    }

    /// Convert a name string, of the syntax `<families>-<point sizes>:<name1>=<values1>:<name2>=<values2>...`,
    /// into a pattern. All sections are optional, and the empty string will match all fonts.
    pub fn from_name(name: &CStr) -> Option<Pattern> {
        super::init();
        // SAFETY: Name is guaranteed a valid C-string, and not held past the duration of this call.
        let raw = unsafe { sys::FcNameParse(name.as_ptr()) };
        NonNull::new(raw).map(Pattern)
    }

    /// Convert into a borrowed reference.
    pub fn as_ref(&self) -> PatternRef<'_> {
        PatternRef(self.0, PhantomData)
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::FcPatternReference(self.0.as_ptr()) };
        Pattern(self.0)
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer
        unsafe { sys::FcPatternDestroy(self.0.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FontSet;

    #[test]
    fn test_pattern_from_name() {
        let pat = Pattern::from_name(c":outline=true").unwrap();
        let pat_ref = pat.as_ref();
        let pat2 = pat_ref.upgrade();
        let pat_ref_2 = pat2.as_ref();

        assert!(pat_ref == pat_ref_2);
    }

    #[test]
    fn test_pattern_get() {
        let fs = FontSet::all();
        let fonts = fs.as_ref().fonts();
        assert!(!fonts.is_empty());

        let mut has_file = false;
        let mut has_index = false;
        for pat in fs.as_ref().fonts() {
            let file = pat.get::<File>(0);
            if !has_file {
                if let Ok(f) = file {
                    // Ensure we don't fault on reading the bytes of the file
                    let _ = f.to_str();
                    has_file = true;
                }
            }
            if !has_index && pat.get::<Index>(0).is_ok() {
                has_index = true;
            }
        }

        assert!(has_file && has_index)
    }
}
