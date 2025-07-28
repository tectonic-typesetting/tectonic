use super::{sys, CoreType};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr;
use std::ptr::NonNull;

pub trait StrLike {
    fn encoding(&self) -> sys::CFStringEncoding;
    fn len(&self) -> usize;
    fn as_ptr(&self) -> *const u8;
}

impl StrLike for str {
    fn encoding(&self) -> sys::CFStringEncoding {
        sys::kCFStringEncodingUTF8
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

impl StrLike for CStr {
    fn encoding(&self) -> sys::CFStringEncoding {
        if self.to_str().is_ok() {
            sys::kCFStringEncodingUTF8
        } else {
            sys::kCFStringEncodingNonLossyASCII
        }
    }

    fn len(&self) -> usize {
        self.to_bytes().len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr().cast()
    }
}

cfty! {
    CFString : CFStringGetTypeID
}

impl CFString {
    pub fn new<S: ?Sized + StrLike>(val: &S) -> CFString {
        let ptr = unsafe {
            sys::CFStringCreateWithBytes(
                ptr::null(),
                val.as_ptr(),
                val.len() as sys::CFIndex,
                val.encoding(),
                false,
            )
        };
        CFString::new_owned(NonNull::new(ptr.cast_mut()).unwrap())
    }

    pub fn len(&self) -> usize {
        unsafe { sys::CFStringGetLength(self.as_type_ref()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_string(&self) -> String {
        let mut buf_len = 0;
        unsafe {
            sys::CFStringGetBytes(
                self.as_type_ref(),
                sys::CFRange {
                    location: 0,
                    length: self.len() as sys::CFIndex,
                },
                sys::kCFStringEncodingUTF8,
                0,
                false,
                ptr::null_mut(),
                0,
                &mut buf_len,
            )
        };
        let mut buf = vec![0u8; buf_len as usize];
        let written = unsafe {
            sys::CFStringGetBytes(
                self.as_type_ref(),
                sys::CFRange {
                    location: 0,
                    length: self.len() as sys::CFIndex,
                },
                sys::kCFStringEncodingUTF8,
                0,
                false,
                buf.as_mut_ptr(),
                buf_len,
                &mut buf_len,
            )
        };
        if written as usize == self.len() {
            String::from_utf8(buf).unwrap()
        } else {
            panic!("Failed to convert CFString");
        }
    }

    pub fn as_str(&self) -> Cow<'_, str> {
        let cstr =
            unsafe { sys::CFStringGetCStringPtr(self.as_type_ref(), sys::kCFStringEncodingUTF8) };
        if cstr.is_null() {
            Cow::Owned(self.get_string())
        } else {
            Cow::Borrowed(unsafe { CStr::from_ptr(cstr) }.to_str().unwrap())
        }
    }

    pub fn get_cstring(&self) -> CString {
        let len = self.len() * 4 + 1;
        let mut buf = vec![0; len];
        let res = unsafe {
            sys::CFStringGetCString(
                self.as_type_ref(),
                buf.as_mut_ptr().cast(),
                len as sys::CFIndex,
                sys::kCFStringEncodingUTF8,
            )
        };
        if res {
            let buf = buf.into_iter().take_while(|&c| c != 0).collect::<Vec<_>>();
            CString::new(buf).unwrap()
        } else {
            panic!("Invalid C String")
        }
    }

    pub fn as_cstr(&self) -> Cow<'_, CStr> {
        let cstr =
            unsafe { sys::CFStringGetCStringPtr(self.as_type_ref(), sys::kCFStringEncodingUTF8) };
        if cstr.is_null() {
            Cow::Owned(self.get_cstring())
        } else {
            Cow::Borrowed(unsafe { CStr::from_ptr(cstr) })
        }
    }
}
