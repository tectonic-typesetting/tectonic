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
    /// A string. Note this is not necessarily like [`String`], as the contained bytes may not
    /// be in UTF-8.
    CFString : CFStringGetTypeID
}

impl CFString {
    /// Create a new string from a value. The value may be any of multiple Rust string types, such
    /// as `str` or `CStr`.
    pub fn new<S: ?Sized + StrLike>(val: &S) -> CFString {
        // SAFETY: Length, pointer, and encoding are derived from same value.
        let ptr = unsafe {
            sys::CFStringCreateWithBytes(
                ptr::null(),
                val.as_ptr(),
                val.len() as sys::CFIndex,
                val.encoding(),
                false,
            )
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer returned by CFStringCreateWithBytes is guaranteed to be a
        //         valid CFString.
        unsafe { CFString::new_owned(ptr) }
    }

    /// Get the length of this string
    pub fn len(&self) -> usize {
        // SAFETY: Self is guaranteed to be valid
        unsafe { sys::CFStringGetLength(self.as_type_ref()) as usize }
    }

    /// Check whether this string is empty (has a length of 0)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get this value as a Rust [`String`]. The value will be re-encoded into UTF-8 if in another
    /// encoding.
    pub fn get_string(&self) -> String {
        let mut buf_len = 0;
        // SAFETY: Self is guaranteed valid. No buffer is passed.
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
        // SAFETY: Self is guaranteed valid. Buffer is allocated to the correct length.
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

    /// Attempt to get a reference to this value as a `str`, if the value is natively UTF-8,
    /// otherwise re-encode into UTF-8.
    pub fn as_str(&self) -> Cow<'_, str> {
        // SAFETY: Self is guaranteed valid
        let cstr =
            unsafe { sys::CFStringGetCStringPtr(self.as_type_ref(), sys::kCFStringEncodingUTF8) };
        if cstr.is_null() {
            Cow::Owned(self.get_string())
        } else {
            // SAFETY: If non-null, the return value of CFStringGetCStringPtr is guaranteed to be a
            //         valid C-string
            Cow::Borrowed(unsafe { CStr::from_ptr(cstr) }.to_str().unwrap())
        }
    }

    /// Get this value as a null-terminated C-string
    pub fn get_cstring(&self) -> CString {
        let len = self.len() * 4 + 1;
        let mut buf = vec![0; len];
        // SAFETY: Self is guaranteed valid. Buffer is definitely of sufficient length to hold the
        //         bytes of self.
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

    /// Attempt to get a reference to this value as a `CStr`, if the value is natively UTF-8,
    /// otherwise allocate a CString in that encoding.
    pub fn as_cstr(&self) -> Cow<'_, CStr> {
        // SAFETY: Self is guaranteed valid
        let cstr =
            unsafe { sys::CFStringGetCStringPtr(self.as_type_ref(), sys::kCFStringEncodingUTF8) };
        if cstr.is_null() {
            Cow::Owned(self.get_cstring())
        } else {
            // SAFETY: If non-null, the return value of CFStringGetCStringPtr is guaranteed to be a
            //         valid C-string
            Cow::Borrowed(unsafe { CStr::from_ptr(cstr) })
        }
    }
}
