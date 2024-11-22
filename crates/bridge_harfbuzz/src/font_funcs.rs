use super::*;
use std::mem::ManuallyDrop;

unsafe extern "C" fn nominal_glyph_func<
    T,
    F: Fn(FontRef<'_>, &T, Codepoint) -> Option<Codepoint>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    ch: Codepoint,
    gid: *mut Codepoint,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();

    match func(font, data, ch) {
        Some(val) => {
            *gid = val;
            true as sys::hb_bool_t
        }
        None => {
            *gid = 0;
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn variation_glyph_func<
    T,
    F: Fn(FontRef<'_>, &T, Codepoint, Codepoint) -> Option<Codepoint>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    ch: Codepoint,
    vs: Codepoint,
    gid: *mut Codepoint,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();

    match func(font, data, ch, vs) {
        Some(val) => {
            *gid = val;
            true as sys::hb_bool_t
        }
        None => {
            *gid = 0;
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn glyph_advance<T, F: Fn(FontRef<'_>, &T, Codepoint) -> Position>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    user_data: *mut (),
) -> Position {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    func(font, data, gid)
}

unsafe extern "C" fn glyph_origin<
    T,
    F: Fn(FontRef<'_>, &T, Codepoint) -> Option<(Position, Position)>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    x: *mut Position,
    y: *mut Position,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();

    match func(font, data, gid) {
        Some(val) => {
            *x = val.0;
            *y = val.1;
            true as sys::hb_bool_t
        }
        None => {
            *x = 0;
            *y = 0;
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn glyph_kerning<T, F: Fn(FontRef<'_>, &T, Codepoint, Codepoint) -> Position>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid1: Codepoint,
    gid2: Codepoint,
    user_data: *mut (),
) -> Position {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    func(font, data, gid1, gid2)
}

unsafe extern "C" fn glyph_extents<T, F: Fn(FontRef<'_>, &T, Codepoint) -> Option<GlyphExtents>>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    extents: *mut GlyphExtents,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    match func(font, data, gid) {
        Some(out) => {
            *extents = out;
            true as sys::hb_bool_t
        }
        None => {
            *extents = GlyphExtents::default();
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn glyph_contour_point<
    T,
    F: Fn(FontRef<'_>, &T, Codepoint, u32) -> Option<(Position, Position)>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    index: libc::c_uint,
    x: *mut Position,
    y: *mut Position,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    match func(font, data, gid, index as u32) {
        Some(val) => {
            *x = val.0;
            *y = val.1;
            true as sys::hb_bool_t
        }
        None => {
            *x = 0;
            *y = 0;
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn glyph_name<T, F: Fn(FontRef<'_>, &T, Codepoint, &mut [u8]) -> usize>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    name: *mut libc::c_char,
    size: libc::c_uint,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = FontRef::from_raw(NonNull::new(font).unwrap());
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();

    let name = if name.is_null() {
        &mut []
    } else {
        ptr::write_bytes(name, 0, size as usize);
        slice::from_raw_parts_mut(name.cast::<u8>(), size as usize)
    };

    match func(font, data, gid, name) {
        0 => false as sys::hb_bool_t,
        _ => true as sys::hb_bool_t,
    }
}

pub struct FontFuncsRef<'a, T>(
    NonNull<sys::hb_font_funcs_t>,
    PhantomData<(&'a sys::hb_font_funcs_t, T)>,
);

impl<'a, T> FontFuncsRef<'a, T> {
    pub(crate) fn as_ptr(&self) -> *mut sys::hb_font_funcs_t {
        self.0.as_ptr()
    }
}

pub struct FontFuncsMut<'a, T>(
    FontFuncsRef<'a, T>,
    PhantomData<&'a mut sys::hb_font_funcs_t>,
);

impl<'a, T> FontFuncsMut<'a, T> {
    fn as_mut_ptr(&self) -> *mut sys::hb_font_funcs_t {
        self.0.as_ptr()
    }

    pub fn nominal_glyph_func<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint) -> Option<Codepoint> + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_nominal_glyph_func(
                self.as_mut_ptr(),
                nominal_glyph_func::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn variation_glyph_func<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint, Codepoint) -> Option<Codepoint> + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_variation_glyph_func(
                self.as_mut_ptr(),
                variation_glyph_func::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_h_advance<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint) -> Position + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_h_advance_func(
                self.as_mut_ptr(),
                glyph_advance::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_v_advance<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint) -> Position + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_v_advance_func(
                self.as_mut_ptr(),
                glyph_advance::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_h_origin<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint) -> Option<(Position, Position)> + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_h_origin_func(
                self.as_mut_ptr(),
                glyph_origin::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_v_origin<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint) -> Option<(Position, Position)> + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_v_origin_func(
                self.as_mut_ptr(),
                glyph_origin::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_h_kerning<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint, Codepoint) -> Position + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_h_kerning_func(
                self.as_mut_ptr(),
                glyph_kerning::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_v_kerning<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint, Codepoint) -> Position + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_v_kerning_func(
                self.as_mut_ptr(),
                glyph_kerning::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_extents<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint) -> Option<GlyphExtents> + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_extents_func(
                self.as_mut_ptr(),
                glyph_extents::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_contour_point<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint, u32) -> Option<(Position, Position)> + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_contour_point_func(
                self.as_mut_ptr(),
                glyph_contour_point::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_name<F>(&mut self, f: F)
    where
        F: Fn(FontRef<'_>, &T, Codepoint, &mut [u8]) -> usize + 'static,
    {
        // SAFETY: Internal pointer guaranteed valid. Ownership of closure is passed to Harfbuzz,
        //         and deallocated by the dealloc function
        unsafe {
            sys::hb_font_funcs_set_glyph_name_func(
                self.as_mut_ptr(),
                glyph_name::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }
}

impl<'a, T> Deref for FontFuncsMut<'a, T> {
    type Target = FontFuncsRef<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FontFuncs<T>(NonNull<sys::hb_font_funcs_t>, PhantomData<T>);

impl<T> FontFuncs<T> {
    pub fn new() -> FontFuncs<T> {
        // SAFETY: This is always safe to call
        let ptr = unsafe { sys::hb_font_funcs_create() };
        FontFuncs(NonNull::new(ptr).unwrap(), PhantomData)
    }

    pub fn as_ref(&self) -> FontFuncsRef<'_, T> {
        FontFuncsRef(self.0, PhantomData)
    }

    pub fn as_mut(&mut self) -> FontFuncsMut<'_, T> {
        FontFuncsMut(self.as_ref(), PhantomData)
    }

    pub fn make_immutable(self) -> ImmutFontFuncs<T> {
        let this = ManuallyDrop::new(self);
        // SAFETY: Internal pointer guaranteed valid. This cannot cause clones to exhibit UB -
        //         unexpected behavior, perhaps, when mutable references stop working, but not UB.
        unsafe { sys::hb_font_funcs_make_immutable(this.0.as_ptr()) };
        ImmutFontFuncs(this.0, this.1)
    }
}

impl<T> Clone for FontFuncs<T> {
    fn clone(&self) -> Self {
        // SAFETY: Internal pointer guaranteed valid.
        unsafe { sys::hb_font_funcs_reference(self.0.as_ptr()) };
        FontFuncs(self.0, PhantomData)
    }
}

impl<T> Default for FontFuncs<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for FontFuncs<T> {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer.
        unsafe { sys::hb_font_funcs_destroy(self.0.as_ptr()) }
    }
}

pub struct ImmutFontFuncs<T>(NonNull<sys::hb_font_funcs_t>, PhantomData<T>);

impl<T> ImmutFontFuncs<T> {
    pub fn as_ref(&self) -> FontFuncsRef<'_, T> {
        FontFuncsRef(self.0, PhantomData)
    }
}

impl<T> Clone for ImmutFontFuncs<T> {
    fn clone(&self) -> Self {
        // SAFETY: Internal pointer guaranteed valid.
        unsafe { sys::hb_font_funcs_reference(self.0.as_ptr()) };
        ImmutFontFuncs(self.0, PhantomData)
    }
}

impl<T> Drop for ImmutFontFuncs<T> {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer.
        unsafe { sys::hb_font_funcs_destroy(self.0.as_ptr()) }
    }
}

// SAFETY: ImmutFontFuncs is gained by calling `make_immutable` on a FontFuncs object, which renders
//         future attempts to change the value no-ops. This in turn means the object becomes safe to
//         send to other threads. The contained data isn't bound because it is tied to the font,
//         which is not Send or Sync and as such will not use the data across threads.
unsafe impl<T> Send for ImmutFontFuncs<T> {}
// SAFETY: ImmutFontFuncs is gained by calling `make_immutable` on a FontFuncs object, which renders
//         future attempts to change the value no-ops. This in turn means the object becomes safe to
//         reference from other threads. The contained data isn't bound because it is tied to the font,
//         which is not Send or Sync and as such will not use the data across threads.
unsafe impl<T> Sync for ImmutFontFuncs<T> {}
