use crate::c_api::fc::sys;
use std::ops::Deref;
use std::ptr;
use std::ptr::NonNull;

pub struct ObjectSet(NonNull<sys::FcObjectSet>);

impl ObjectSet {
    fn from_raw(ptr: *mut sys::FcObjectSet) -> Option<ObjectSet> {
        NonNull::new(ptr).map(ObjectSet)
    }

    pub fn as_raw(&self) -> *mut sys::FcObjectSet {
        self.0.as_ptr()
    }
}

pub struct OwnObjectSet(ObjectSet);

impl OwnObjectSet {
    pub fn new() -> OwnObjectSet {
        super::init();
        // TODO: Allow configuring these. Annoying because it's a VaList.
        let ptr = unsafe {
            sys::FcObjectSetBuild(
                sys::FC_FAMILY,
                sys::FC_STYLE,
                sys::FC_FILE,
                sys::FC_INDEX,
                sys::FC_FULLNAME,
                sys::FC_WEIGHT,
                sys::FC_WIDTH,
                sys::FC_SLANT,
                sys::FC_FONTFORMAT,
                ptr::null::<libc::c_char>(),
            )
        };
        OwnObjectSet(ObjectSet::from_raw(ptr).unwrap())
    }
}

impl Deref for OwnObjectSet {
    type Target = ObjectSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for OwnObjectSet {
    fn drop(&mut self) {
        unsafe { sys::FcObjectSetDestroy(self.0.as_raw()) }
    }
}
