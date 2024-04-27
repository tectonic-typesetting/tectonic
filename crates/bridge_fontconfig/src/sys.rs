#![allow(nonstandard_style, unused)]

pub const FC_FAMILY: *const libc::c_char = c!("family");
pub const FC_STYLE: *const libc::c_char = c!("style");
pub const FC_SLANT: *const libc::c_char = c!("slant");
pub const FC_WEIGHT: *const libc::c_char = c!("weight");
pub const FC_WIDTH: *const libc::c_char = c!("width");
pub const FC_FILE: *const libc::c_char = c!("file");
pub const FC_INDEX: *const libc::c_char = c!("index");
pub const FC_FULLNAME: *const libc::c_char = c!("fullname");
pub const FC_FONTFORMAT: *const libc::c_char = c!("fontformat");

pub type FcBool = libc::c_int;

pub const FcTrue: FcBool = 1;
pub const FcFalse: FcBool = 0;

#[repr(C)]
pub struct FcPattern(());

#[repr(C)]
pub struct FcFontSet {
    pub nfont: libc::c_int,
    sfont: libc::c_int,
    pub fonts: *const *mut FcPattern,
}

#[repr(C)]
pub struct FcObjectSet(());

#[repr(C)]
pub struct FcConfig(());

#[derive(PartialEq)]
#[repr(C)]
pub enum FcResult {
    Match,
    NoMatch,
    TypeMismatch,
    ResultNoId,
    OutOfMemory,
}

extern "C" {
    pub fn FcPatternGetString(
        p: *mut FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        s: *mut *const libc::c_char,
    ) -> FcResult;
    pub fn FcPatternGetInteger(
        p: *mut FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        i: *mut libc::c_int,
    ) -> FcResult;
    pub fn FcInit() -> FcBool;
    pub fn FcNameParse(name: *const libc::c_char) -> *mut FcPattern;
    pub fn FcObjectSetBuild(first: *const libc::c_char, ...) -> *mut FcObjectSet;
    pub fn FcFontList(
        config: *mut FcConfig,
        p: *mut FcPattern,
        os: *mut FcObjectSet,
    ) -> *mut FcFontSet;
    pub fn FcConfigGetCurrent() -> *mut FcConfig;
    pub fn FcObjectSetDestroy(os: *mut FcObjectSet);
    pub fn FcPatternReference(pat: *mut FcPattern);
    pub fn FcPatternDestroy(pat: *mut FcPattern);
    pub fn FcFontSetDestroy(fs: *mut FcFontSet);
    pub fn FcObjectSetCreate() -> *mut FcObjectSet;
    pub fn FcObjectSetAdd(os: *mut FcObjectSet, object: *const libc::c_char) -> FcBool;
}
