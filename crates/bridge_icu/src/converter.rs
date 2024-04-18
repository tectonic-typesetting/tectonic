use crate::{sys, IcuErr};
use std::ffi::CStr;
use std::ptr::NonNull;

pub struct Converter(NonNull<sys::UConverter>);

impl Converter {
    pub fn new(name: &CStr) -> Result<Converter, IcuErr> {
        let mut err = sys::U_ZERO_ERROR;
        let ptr = unsafe { sys::ucnv_open(name.as_ptr(), &mut err) };
        if sys::U_SUCCESS(err) {
            Ok(Converter(NonNull::new(ptr).unwrap()))
        } else {
            Err(IcuErr::from_raw(err))
        }
    }

    fn as_ptr(&self) -> *mut sys::UConverter {
        self.0.as_ptr()
    }

    pub fn max_char_size(&self) -> u8 {
        unsafe { sys::ucnv_getMaxCharSize(self.as_ptr()) as u8 }
    }

    /// Takes a slice encoded using this converter, and writes it to a buffer as UTF16 codepoints.
    pub fn to_uchars(&self, encoded: &[u8]) -> Result<Vec<u16>, IcuErr> {
        let mut buffer = vec![0; encoded.len() * 2];
        let mut err = sys::U_ZERO_ERROR;
        let len = unsafe {
            sys::ucnv_toUChars(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len() as i32,
                encoded.as_ptr().cast(),
                encoded.len() as i32,
                &mut err,
            )
        };
        if sys::U_SUCCESS(err) {
            buffer.truncate(len as usize);
            Ok(buffer)
        } else {
            Err(IcuErr::from_raw(err))
        }
    }

    /// Takes a slice of UTF16 codepoints, and encodes it using this converter.
    pub fn from_uchars(&self, chars: &[u16]) -> Result<Vec<u8>, IcuErr> {
        let mut buffer = vec![0u8; (chars.len() + 10) * self.max_char_size() as usize];
        let mut err = sys::U_ZERO_ERROR;
        let len = unsafe {
            sys::ucnv_fromUChars(
                self.as_ptr(),
                buffer.as_mut_ptr().cast(),
                buffer.len() as i32,
                chars.as_ptr().cast(),
                chars.len() as i32,
                &mut err,
            )
        };
        if sys::U_SUCCESS(err) {
            buffer.truncate(len as usize);
            Ok(buffer)
        } else {
            Err(IcuErr::from_raw(err))
        }
    }
}

impl Drop for Converter {
    fn drop(&mut self) {
        unsafe { sys::ucnv_close(self.0.as_ptr()) }
    }
}
