// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the graphite2 *C* API into the Cargo build framework used by [Tectonic],
//! as well as provide bindings to other tectonic crates.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::unnecessary_cast)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

pub mod sys;

pub use sys::gr_encform as EncForm;

pub const BREAK_NONE: i32 = sys::gr_breakNone as i32;
pub const BREAK_BEFORE_WORD: i32 = sys::gr_breakBeforeWord as i32;
pub const BREAK_WORD: i32 = sys::gr_breakWord as i32;

pub struct FeatErr(());

pub struct Label(usize, NonNull<u8>);

impl Label {
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: Pointer in `self.1` is guaranteed to be a valid array of length `self.0`
        unsafe { std::slice::from_raw_parts(self.1.as_ptr(), self.0) }
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: Array returned by `as_bytes` is guaranteed a valid UTF-8 array on Label creation.
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn into_raw(self) -> *mut u8 {
        let this = ManuallyDrop::new(self);
        this.1.as_ptr()
    }
}

impl Drop for Label {
    fn drop(&mut self) {
        // SAFETY: Pointer guaranteed valid and owned by us.
        unsafe { sys::gr_label_destroy(self.1.cast().as_ptr()) }
    }
}

#[derive(Copy, Clone)]
pub struct FeatureRef<'a>(
    NonNull<sys::gr_feature_ref>,
    PhantomData<&'a sys::gr_feature_ref>,
);

impl FeatureRef<'_> {
    fn as_ptr(self) -> *const sys::gr_feature_ref {
        self.0.as_ptr().cast_const()
    }

    pub fn id(self) -> u32 {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_fref_id(self.as_ptr()) }
    }

    pub fn num_values(self) -> usize {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_fref_n_values(self.as_ptr()) as usize }
    }

    pub fn value(self, idx: usize) -> i16 {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_fref_value(self.as_ptr(), idx as u16) }
    }

    pub fn feat_value(self, feat: FeatureValRef<'_>) -> u16 {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_fref_feature_value(self.as_ptr(), feat.as_ptr()) }
    }

    pub fn set_feat_value(self, mut feat: FeatureValMut<'_>, value: u16) -> Result<(), FeatErr> {
        // SAFETY: Contained pointer guaranteed valid
        //         Feat pointer guaranteed valid
        let res =
            unsafe { sys::gr_fref_set_feature_value(self.as_ptr(), value, feat.as_ptr_mut()) };
        if res != 0 {
            Ok(())
        } else {
            Err(FeatErr(()))
        }
    }

    pub fn label(self, lang_id: u16) -> Option<Label> {
        let mut actual_id = lang_id;
        let mut len = 0;
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe {
            sys::gr_fref_label(
                self.as_ptr(),
                &mut actual_id,
                sys::gr_encform::utf8,
                &mut len,
            )
        };
        NonNull::new(ptr.cast()).map(|ptr| Label(len as usize, ptr))
    }

    pub fn value_label(self, idx: usize, lang_id: u16) -> Option<Label> {
        let mut actual_id = lang_id;
        let mut len = 0;
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe {
            sys::gr_fref_value_label(
                self.as_ptr(),
                idx as u16,
                &mut actual_id,
                sys::gr_encform::utf8,
                &mut len,
            )
        };
        NonNull::new(ptr.cast()).map(|ptr| Label(len as usize, ptr))
    }
}

#[derive(Copy, Clone)]
pub struct FeatureValRef<'a>(
    NonNull<sys::gr_feature_val>,
    PhantomData<&'a sys::gr_feature_val>,
);

impl FeatureValRef<'_> {
    fn as_ptr(self) -> *const sys::gr_feature_val {
        self.0.as_ptr().cast_const()
    }
}

pub struct FeatureValMut<'a>(
    NonNull<sys::gr_feature_val>,
    PhantomData<&'a mut sys::gr_feature_val>,
);

impl FeatureValMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::gr_feature_val {
        self.0.as_ptr()
    }
}

pub struct FeatureVal(NonNull<sys::gr_feature_val>);

impl FeatureVal {
    pub fn as_ref(&self) -> FeatureValRef<'_> {
        FeatureValRef(self.0, PhantomData)
    }

    pub fn as_mut(&mut self) -> FeatureValMut<'_> {
        FeatureValMut(self.0, PhantomData)
    }
}

impl Drop for FeatureVal {
    fn drop(&mut self) {
        // SAFETY: Contained pointer guaranteed valid, and owned by us
        unsafe { sys::gr_featureval_destroy(self.0.as_ptr()) }
    }
}

#[derive(Copy, Clone)]
pub struct FaceRef<'a>(NonNull<sys::gr_face>, PhantomData<&'a sys::gr_face>);

impl<'a> FaceRef<'a> {
    /// Create a new instace of `FaceRef` from a pointer to a Graphite `gr_face` structure.
    ///
    /// # Safety
    ///
    /// The provided pointer must be valid to pass to graphite functions for the chosen lifetime
    /// of `'a`. This means not calling `gr_face_destroy` on it such that its refcount is reduced to
    /// zero.
    pub unsafe fn from_raw(ptr: NonNull<()>) -> Self {
        FaceRef(ptr.cast(), PhantomData)
    }

    fn as_ptr(self) -> *const sys::gr_face {
        self.0.as_ptr().cast_const()
    }

    pub fn num_feature_refs(self) -> usize {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_face_n_fref(self.as_ptr()) as usize }
    }

    pub fn feature_ref(self, idx: usize) -> Option<FeatureRef<'a>> {
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe { sys::gr_face_fref(self.as_ptr(), idx as u16) };
        NonNull::new(ptr.cast_mut()).map(|p| FeatureRef(p, PhantomData))
    }

    pub fn find_feature_ref(self, feat_id: u32) -> Option<FeatureRef<'a>> {
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe { sys::gr_face_find_fref(self.as_ptr(), feat_id) };
        NonNull::new(ptr.cast_mut()).map(|p| FeatureRef(p, PhantomData))
    }

    pub fn feature_val_for_lang(self, lang: u32) -> FeatureVal {
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe { sys::gr_face_featureval_for_lang(self.as_ptr(), lang) };
        FeatureVal(NonNull::new(ptr.cast()).unwrap())
    }
}

#[derive(Copy, Clone)]
pub struct FontRef<'a>(NonNull<sys::gr_font>, PhantomData<&'a sys::gr_font>);

impl FontRef<'_> {
    fn as_ptr(self) -> *const sys::gr_font {
        self.0.as_ptr().cast_const()
    }
}

pub struct Font(NonNull<sys::gr_font>);

impl Font {
    pub fn new(pt_size: f32, face: FaceRef<'_>) -> Option<Font> {
        // SAFETY: Face pointer guaranteed valid
        let ptr = unsafe { sys::gr_make_font(pt_size, face.as_ptr()) };
        NonNull::new(ptr.cast()).map(Font)
    }

    pub fn as_ref(&self) -> FontRef<'_> {
        FontRef(self.0, PhantomData)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        // SAFETY: Contained pointer guaranteed valid, and owned by us.
        unsafe { sys::gr_font_destroy(self.0.cast().as_ptr()) }
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for str {}
    impl Sealed for (*const u16, usize) {}
}

#[allow(clippy::len_without_is_empty)]
pub trait StrEnc: sealed::Sealed {
    fn enc(&self) -> EncForm;
    fn as_ptr(&self) -> *const ();
    fn len(&self) -> usize;
}

impl StrEnc for str {
    fn enc(&self) -> EncForm {
        EncForm::utf8
    }

    fn as_ptr(&self) -> *const () {
        self.as_ptr().cast()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl StrEnc for (*const u16, usize) {
    fn enc(&self) -> EncForm {
        EncForm::utf16
    }

    fn as_ptr(&self) -> *const () {
        self.0.cast()
    }

    fn len(&self) -> usize {
        self.1
    }
}

#[derive(Copy, Clone)]
pub struct SegmentRef<'a>(NonNull<sys::gr_segment>, PhantomData<&'a sys::gr_segment>);

impl<'a> SegmentRef<'a> {
    fn as_ptr(self) -> *const sys::gr_segment {
        self.0.as_ptr().cast_const()
    }

    pub fn first_slot(self) -> Slot {
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe { sys::gr_seg_first_slot(self.as_ptr().cast_mut()) };
        Slot(self.0, NonNull::new(ptr.cast_mut()).unwrap())
    }

    pub fn last_slot(self) -> Slot {
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe { sys::gr_seg_last_slot(self.as_ptr().cast_mut()) };
        Slot(self.0, NonNull::new(ptr.cast_mut()).unwrap())
    }

    pub fn cinfo(self, idx: usize) -> CharInfoRef<'a> {
        // SAFETY: Contained pointer guaranteed valid
        let ptr = unsafe { sys::gr_seg_cinfo(self.as_ptr(), idx as u32) };
        CharInfoRef(NonNull::new(ptr.cast_mut()).unwrap(), PhantomData)
    }

    pub fn next(self, slot: &Slot) -> Option<Slot> {
        assert_eq!(self.as_ptr().cast_mut(), slot.0.as_ptr().cast());
        // SAFETY: Slot pointer guaranteed valid
        let ptr = unsafe { sys::gr_slot_next_in_segment(slot.as_ptr()) };
        NonNull::new(ptr.cast_mut()).map(|ptr| Slot(self.0, ptr))
    }

    pub fn index(self, slot: &Slot) -> usize {
        assert_eq!(self.as_ptr().cast_mut(), slot.0.as_ptr().cast());
        // SAFETY: Slot pointer guaranteed valid
        unsafe { sys::gr_slot_index(slot.as_ptr()) as usize }
    }
}

pub struct Segment(NonNull<sys::gr_segment>);

impl Segment {
    pub fn new<S: ?Sized + StrEnc>(
        font: FontRef<'_>,
        face: FaceRef<'_>,
        script: u32,
        features: FeatureValRef<'_>,
        s: &S,
    ) -> Option<Segment> {
        // SAFETY: Font pointer guaranteed valid
        //         Face pointer guaranteed valid
        //         Features pointer guaranteed valid
        //         S values must all match for this call, guaranteed by StrEnc.
        let ptr = unsafe {
            sys::gr_make_seg(
                font.as_ptr(),
                face.as_ptr(),
                script,
                features.as_ptr(),
                s.enc(),
                s.as_ptr().cast(),
                s.len(),
                0,
            )
        };
        NonNull::new(ptr.cast()).map(Segment)
    }

    pub fn as_ref(&self) -> SegmentRef<'_> {
        SegmentRef(self.0, PhantomData)
    }
}

impl Drop for Segment {
    fn drop(&mut self) {
        // SAFETY: Contained pointer guaranteed valid, and owned by us.
        unsafe { sys::gr_seg_destroy(self.0.cast().as_ptr()) }
    }
}

#[derive(Clone, PartialEq)]
pub struct Slot(NonNull<sys::gr_segment>, NonNull<sys::gr_slot>);

impl Slot {
    fn as_ptr(&self) -> *const sys::gr_slot {
        self.1.as_ptr()
    }
}

#[derive(Copy, Clone)]
pub struct CharInfoRef<'a>(
    NonNull<sys::gr_char_info>,
    PhantomData<&'a sys::gr_char_info>,
);

impl CharInfoRef<'_> {
    fn as_ptr(self) -> *const sys::gr_char_info {
        self.0.as_ptr().cast_const()
    }

    pub fn break_weight(self) -> i32 {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_cinfo_break_weight(self.as_ptr()) as i32 }
    }

    pub fn base(self) -> usize {
        // SAFETY: Contained pointer guaranteed valid
        unsafe { sys::gr_cinfo_base(self.as_ptr()) }
    }
}

#[test]
fn linkage() {}
