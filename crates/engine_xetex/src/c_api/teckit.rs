#![allow(nonstandard_style)]

#[repr(C)]
pub struct TECkit_Converter_Private(());

pub type TECkit_Converter = *mut TECkit_Converter_Private;
pub type TECkit_Status = libc::c_long;

extern "C" {
    pub fn TECkit_DisposeConverter(converter: TECkit_Converter) -> TECkit_Status;
}
