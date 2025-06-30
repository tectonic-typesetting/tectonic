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

pub trait PatParam {
    type Output<'a>;
    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr>;
}

pub struct File(());

impl PatParam for File {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_FILE, idx as libc::c_int)
    }
}

pub struct Family(());

impl PatParam for Family {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_FAMILY, idx as libc::c_int)
    }
}

pub struct FullName(());

impl PatParam for FullName {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_FULLNAME, idx as libc::c_int)
    }
}

pub struct Style(());

impl PatParam for Style {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_string(pat, sys::FC_STYLE, idx as libc::c_int)
    }
}

pub struct Index(());

impl PatParam for Index {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_INDEX, idx as libc::c_int)
    }
}

pub struct Weight(());

impl PatParam for Weight {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_WEIGHT, idx as libc::c_int)
    }
}

pub struct Width(());

impl PatParam for Width {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_WIDTH, idx as libc::c_int)
    }
}

pub struct Slant(());

impl PatParam for Slant {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        get_int(pat, sys::FC_SLANT, idx as libc::c_int)
    }
}

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

    pub fn upgrade(self) -> Pattern {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { Pattern::from_raw_borrowed(self.0) }
    }

    pub fn as_ptr(self) -> *mut sys::FcPattern {
        self.0.as_ptr()
    }

    pub fn get<T: PatParam>(self, idx: usize) -> Result<T::Output<'a>, FcErr> {
        T::get(self, idx)
    }
}

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

    pub fn from_name(name: &CStr) -> Option<Pattern> {
        super::init();
        // SAFETY: Name is guaranteed a valid C-string, and not held past the duration of this call.
        let raw = unsafe { sys::FcNameParse(name.as_ptr()) };
        NonNull::new(raw).map(Pattern)
    }

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
