use super::{sys, FcErr};
use std::convert::TryInto;
use std::ffi::CStr;
use std::ptr;
use std::ptr::NonNull;

unsafe fn get_string<'a>(
    pat: *mut sys::FcPattern,
    ty: *const libc::c_char,
    idx: libc::c_int,
) -> Result<&'a CStr, FcErr> {
    let mut str = ptr::null();
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
    let mut int = 0;
    match unsafe { sys::FcPatternGetInteger(pat, ty, idx, &mut int) }.try_into() {
        Ok(err) => Err(err),
        Err(_) => Ok(int as i32),
    }
}

pub trait PatParam {
    type Output<'a>;
    fn get(pat: &Pattern, idx: usize) -> Result<Self::Output<'_>, FcErr>;
}

pub struct File(());

impl PatParam for File {
    type Output<'a> = &'a CStr;

    fn get(pat: &Pattern, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        unsafe { get_string(pat.0.as_ptr(), sys::FC_FILE, idx as libc::c_int) }
    }
}

pub struct Family(());

impl PatParam for Family {
    type Output<'a> = &'a CStr;

    fn get(pat: &Pattern, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        unsafe { get_string(pat.0.as_ptr(), sys::FC_FAMILY, idx as libc::c_int) }
    }
}

pub struct FullName(());

impl PatParam for FullName {
    type Output<'a> = &'a CStr;

    fn get(pat: &Pattern, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        unsafe { get_string(pat.0.as_ptr(), sys::FC_FULLNAME, idx as libc::c_int) }
    }
}

pub struct Style(());

impl PatParam for Style {
    type Output<'a> = &'a CStr;

    fn get(pat: &Pattern, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        unsafe { get_string(pat.0.as_ptr(), sys::FC_STYLE, idx as libc::c_int) }
    }
}

pub struct Index(());

impl PatParam for Index {
    type Output<'a> = i32;

    fn get(pat: &Pattern, idx: usize) -> Result<Self::Output<'_>, FcErr> {
        unsafe { get_int(pat.0.as_ptr(), sys::FC_INDEX, idx as libc::c_int) }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Pattern(NonNull<sys::FcPattern>);

impl Pattern {
    pub unsafe fn from_raw(ptr: *mut sys::FcPattern) -> Option<Pattern> {
        NonNull::new(ptr).map(Pattern)
    }

    pub fn into_raw(self) -> *mut sys::FcPattern {
        self.0.as_ptr()
    }

    pub fn from_name(name: &CStr) -> Option<Pattern> {
        let raw = unsafe { sys::FcNameParse(name.as_ptr()) };
        NonNull::new(raw).map(Pattern)
    }

    pub fn get<T: PatParam>(&self, idx: usize) -> Result<T::Output<'_>, FcErr> {
        T::get(self, idx)
    }
}
