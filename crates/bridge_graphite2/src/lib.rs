// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the graphite2 *C* API into the Cargo build framework used by [Tectonic],
//! as well as provide bindings to other tectonic crates.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

use std::convert::Infallible;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::ptr::NonNull;

pub mod sys;

pub use sys::gr_encform as EncForm;

pub struct Label(usize, NonNull<u8>);

impl Label {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.1.as_ptr(), self.0) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn into_raw(self) -> *mut u8 {
        let this = ManuallyDrop::new(self);
        this.1.as_ptr()
    }
}

impl Drop for Label {
    fn drop(&mut self) {
        unsafe { sys::gr_label_destroy(self.1.cast().as_ptr()) }
    }
}

pub struct FeatureRef(Infallible);

impl FeatureRef {
    fn as_ptr(&self) -> *const sys::gr_feature_ref {
        ptr::from_ref(self).cast()
    }

    pub fn id(&self) -> u32 {
        unsafe { sys::gr_fref_id(self.as_ptr()) }
    }

    pub fn num_values(&self) -> usize {
        unsafe { sys::gr_fref_n_values(self.as_ptr()) as usize }
    }

    pub fn value(&self, idx: usize) -> i16 {
        unsafe { sys::gr_fref_value(self.as_ptr(), idx as u16) }
    }

    pub fn feat_value(&self, feat: &FeatureVal) -> u16 {
        unsafe { sys::gr_fref_feature_value(self.as_ptr(), feat.as_ptr()) }
    }

    pub fn set_feat_value(&self, feat: &mut FeatureVal, value: u16) -> Result<(), ()> {
        let res =
            unsafe { sys::gr_fref_set_feature_value(self.as_ptr(), value, feat.as_ptr_mut()) };
        if res != 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn label(&self, lang_id: u16) -> Option<Label> {
        let mut actual_id = lang_id;
        let mut len = 0;
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

    pub fn value_label(&self, idx: usize, lang_id: u16) -> Option<Label> {
        let mut actual_id = lang_id;
        let mut len = 0;
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

pub struct FeatureVal(Infallible);

impl FeatureVal {
    pub fn as_ptr(&self) -> *const sys::gr_feature_val {
        ptr::from_ref(self).cast()
    }

    fn as_ptr_mut(&mut self) -> *mut sys::gr_feature_val {
        ptr::from_mut(self).cast()
    }
}

pub struct OwnFeatureVal(NonNull<FeatureVal>);

impl Deref for OwnFeatureVal {
    type Target = FeatureVal;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for OwnFeatureVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for OwnFeatureVal {
    fn drop(&mut self) {
        unsafe { sys::gr_featureval_destroy(self.0.cast().as_ptr()) }
    }
}

pub struct Face(Infallible);

impl Face {
    pub fn as_ptr(&self) -> *const sys::gr_face {
        ptr::from_ref(self).cast()
    }

    pub fn num_feature_refs(&self) -> usize {
        unsafe { sys::gr_face_n_fref(self.as_ptr()) as usize }
    }

    pub fn feature_ref(&self, idx: usize) -> Option<&FeatureRef> {
        unsafe {
            sys::gr_face_fref(self.as_ptr(), idx as u16)
                .cast::<FeatureRef>()
                .as_ref()
        }
    }

    pub fn find_feature_ref(&self, feat_id: u32) -> Option<&FeatureRef> {
        unsafe {
            sys::gr_face_find_fref(self.as_ptr(), feat_id)
                .cast::<FeatureRef>()
                .as_ref()
        }
    }

    pub fn feature_val_for_lang(&self, lang: u32) -> OwnFeatureVal {
        let ptr = unsafe { sys::gr_face_featureval_for_lang(self.as_ptr(), lang) };
        OwnFeatureVal(NonNull::new(ptr.cast()).unwrap())
    }
}

pub struct Font(Infallible);

impl Font {
    pub fn as_ptr(&self) -> *const sys::gr_font {
        ptr::from_ref(self).cast()
    }
}

pub struct OwnFont(NonNull<Font>);

impl OwnFont {
    pub fn new(pt_size: f32, face: &Face) -> Option<OwnFont> {
        let ptr = unsafe { sys::gr_make_font(pt_size, face.as_ptr()) };
        NonNull::new(ptr.cast()).map(OwnFont)
    }
}

impl Deref for OwnFont {
    type Target = Font;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for OwnFont {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for OwnFont {
    fn drop(&mut self) {
        unsafe { sys::gr_font_destroy(self.0.cast().as_ptr()) }
    }
}

pub trait StrEnc {
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

pub struct Segment(Infallible);

impl Segment {
    fn as_ptr(&self) -> *const sys::gr_segment {
        ptr::from_ref(self).cast()
    }

    pub fn first_slot(&self) -> &Slot {
        unsafe { &*sys::gr_seg_first_slot(self.as_ptr().cast_mut()).cast() }
    }
}

pub struct OwnSegment(NonNull<Segment>);

impl OwnSegment {
    pub fn new<S: ?Sized + StrEnc>(
        font: &Font,
        face: &Face,
        script: u32,
        features: &FeatureVal,
        s: &S,
    ) -> Option<OwnSegment> {
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
        NonNull::new(ptr.cast()).map(OwnSegment)
    }
}

impl Deref for OwnSegment {
    type Target = Segment;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for OwnSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for OwnSegment {
    fn drop(&mut self) {
        unsafe { sys::gr_seg_destroy(self.0.cast().as_ptr()) }
    }
}

pub struct Slot(Infallible);

#[test]
fn linkage() {}
