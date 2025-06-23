use super::{sys, FcErr};
use std::convert::TryInto;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

unsafe fn get_string<'a>(
    pat: *mut sys::FcPattern,
    ty: *const libc::c_char,
    idx: libc::c_int,
) -> Result<&'a CStr, FcErr> {
    let mut str = ptr::null();
    // SAFETY: Provided values valid is a precondition.
    match unsafe { sys::FcPatternGetString(pat, ty, idx, &mut str) }.try_into() {
        Ok(err) => Err(err),
        Err(_) => Ok(CStr::from_ptr(str)),
    }
}

unsafe fn get_int(
    pat: *mut sys::FcPattern,
    ty: *const libc::c_char,
    idx: libc::c_int,
) -> Result<i32, FcErr> {
    let mut int: libc::c_int = 0;
    // SAFETY: Provided values valid is a precondition.
    match unsafe { sys::FcPatternGetInteger(pat, ty, idx, &mut int) }.try_into() {
        Ok(err) => Err(err),
        Err(_) => Ok(int as i32),
    }
}

pub trait PatParam {
    type Output<'a>;
    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr>;
}

pub struct File(());

impl PatParam for File {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_FILE is a string property.
        unsafe { get_string(pat.0.as_ptr(), sys::FC_FILE, idx as libc::c_int) }
    }
}

pub struct Family(());

impl PatParam for Family {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_FAMILY is a string property.
        unsafe { get_string(pat.0.as_ptr(), sys::FC_FAMILY, idx as libc::c_int) }
    }
}

pub struct FullName(());

impl PatParam for FullName {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_FULLNAME is a string property.
        unsafe { get_string(pat.0.as_ptr(), sys::FC_FULLNAME, idx as libc::c_int) }
    }
}

pub struct Style(());

impl PatParam for Style {
    type Output<'a> = &'a CStr;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_STYLE is a string property.
        unsafe { get_string(pat.0.as_ptr(), sys::FC_STYLE, idx as libc::c_int) }
    }
}

pub struct Index(());

impl PatParam for Index {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_INDEX is an int property.
        unsafe { get_int(pat.0.as_ptr(), sys::FC_INDEX, idx as libc::c_int) }
    }
}

pub struct Weight(());

impl PatParam for Weight {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_WEIGHT is an int property.
        unsafe { get_int(pat.0.as_ptr(), sys::FC_WEIGHT, idx as libc::c_int) }
    }
}

pub struct Width(());

impl PatParam for Width {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_WIDTH is an int property.
        unsafe { get_int(pat.0.as_ptr(), sys::FC_WIDTH, idx as libc::c_int) }
    }
}

pub struct Slant(());

impl PatParam for Slant {
    type Output<'a> = i32;

    fn get(pat: PatternRef<'_>, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        // SAFETY: Pat pointer guaranteed valid. FC_SLANT is an int property.
        unsafe { get_int(pat.0.as_ptr(), sys::FC_SLANT, idx as libc::c_int) }
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
