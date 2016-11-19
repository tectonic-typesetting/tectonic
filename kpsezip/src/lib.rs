#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate zip;

use std::ffi;
use std::ptr;

mod find;

/* Emulating the kpathsea C API. You can see that these are virtually all
 * boring noops, so with time these will hopefully disappear.
 */

#[no_mangle]
pub extern fn kpse_find_file(name: *const i8, format: libc::c_int, must_exist: libc::c_int) -> *const i8 {
    let rname = unsafe { ffi::CStr::from_ptr (name) }.to_bytes ();
    let rformat = find::c_format_to_rust (format);
    let rmust_exist = must_exist != 0;

    println!("kpsezip find_file: {:?}, {:?} ({}), {}", rname, rformat, format, rmust_exist);

    let rv = match rformat {
        Some(fmt) => find::find_file (rname, fmt, rmust_exist),
        None => None
    };

    match rv {
        Some(path) => unsafe {
            let ours = ffi::CStr::from_bytes_with_nul_unchecked (path);
            let theirs = libc::malloc (path.len () + 1) as *mut i8;
            ptr::copy_nonoverlapping (ours.as_ptr (), theirs, path.len () + 1);
            theirs
        },
        None => ptr::null()
    }
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static kpathsea_version_string: &'static str = "kpsezip 0.1";


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
