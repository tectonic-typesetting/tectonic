use super::*;

unsafe extern "C" fn nominal_glyph_func<T, F: Fn(&mut Font, &T, Codepoint) -> Option<Codepoint>>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    ch: Codepoint,
    gid: *mut Codepoint,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = &mut *font.cast();
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
    F: Fn(&mut Font, &T, Codepoint, Codepoint) -> Option<Codepoint>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    ch: Codepoint,
    vs: Codepoint,
    gid: *mut Codepoint,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = &mut *font.cast();
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

unsafe extern "C" fn glyph_advance<T, F: Fn(&mut Font, &T, Codepoint) -> Position>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    user_data: *mut (),
) -> Position {
    let font = &mut *font.cast();
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    func(font, data, gid)
}

unsafe extern "C" fn glyph_origin<
    T,
    F: Fn(&mut Font, &T, Codepoint) -> Option<(Position, Position)>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    x: *mut Position,
    y: *mut Position,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = &mut *font.cast();
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();

    match func(font, data, gid) {
        Some(val) => {
            *x = val.0;
            *y = val.1;
            true as sys::hb_bool_t
        }
        None => {
            debug_assert_eq!(*x, 0);
            debug_assert_eq!(*y, 0);
            *x = 0;
            *y = 0;
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn glyph_kerning<T, F: Fn(&mut Font, &T, Codepoint, Codepoint) -> Position>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid1: Codepoint,
    gid2: Codepoint,
    user_data: *mut (),
) -> Position {
    let font = &mut *font.cast();
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    func(font, data, gid1, gid2)
}

unsafe extern "C" fn glyph_extents<T, F: Fn(&mut Font, &T, Codepoint) -> Option<GlyphExtents>>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    extents: *mut GlyphExtents,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = &mut *font.cast();
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
    F: Fn(&mut Font, &T, Codepoint, u32) -> Option<(Position, Position)>,
>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    index: libc::c_uint,
    x: *mut Position,
    y: *mut Position,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = &mut *font.cast();
    let data = &*font_data.cast::<T>();
    let func = &*user_data.cast::<F>();
    match func(font, data, gid, index as u32) {
        Some(val) => {
            *x = val.0;
            *y = val.1;
            true as sys::hb_bool_t
        }
        None => {
            debug_assert_eq!(*x, 0);
            debug_assert_eq!(*y, 0);
            *x = 0;
            *y = 0;
            false as sys::hb_bool_t
        }
    }
}

unsafe extern "C" fn glyph_name<T, F: Fn(&mut Font, &T, Codepoint, &mut [u8]) -> usize>(
    font: *mut sys::hb_font_t,
    font_data: *mut (),
    gid: Codepoint,
    name: *mut libc::c_char,
    size: libc::c_uint,
    user_data: *mut (),
) -> sys::hb_bool_t {
    let font = &mut *font.cast();
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

pub struct FontFuncs<T>(*mut sys::hb_font_funcs_t, PhantomData<T>);

unsafe impl<T> Send for FontFuncs<T> {}
unsafe impl<T> Sync for FontFuncs<T> {}

impl<T> FontFuncs<T> {
    pub fn new() -> FontFuncs<T> {
        FontFuncs(unsafe { sys::hb_font_funcs_create() }, PhantomData)
    }

    pub fn nominal_glyph_func<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint) -> Option<Codepoint> + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_nominal_glyph_func(
                self.0,
                nominal_glyph_func::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn variation_glyph_func<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint, Codepoint) -> Option<Codepoint> + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_variation_glyph_func(
                self.0,
                variation_glyph_func::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_h_advance<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint) -> Position + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_h_advance_func(
                self.0,
                glyph_advance::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_v_advance<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint) -> Position + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_v_advance_func(
                self.0,
                glyph_advance::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_h_origin<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint) -> Option<(Position, Position)> + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_h_origin_func(
                self.0,
                glyph_origin::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_v_origin<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint) -> Option<(Position, Position)> + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_v_origin_func(
                self.0,
                glyph_origin::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_h_kerning<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint, Codepoint) -> Position + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_h_kerning_func(
                self.0,
                glyph_kerning::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_v_kerning<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint, Codepoint) -> Position + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_v_kerning_func(
                self.0,
                glyph_kerning::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_extents<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint) -> Option<GlyphExtents> + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_extents_func(
                self.0,
                glyph_extents::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_contour_point<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint, u32) -> Option<(Position, Position)> + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_contour_point_func(
                self.0,
                glyph_contour_point::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub fn glyph_name<F>(&mut self, f: F)
    where
        F: Fn(&mut Font, &T, Codepoint, &mut [u8]) -> usize + 'static,
    {
        unsafe {
            sys::hb_font_funcs_set_glyph_name_func(
                self.0,
                glyph_name::<T, F>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<F>),
            )
        }
    }

    pub(crate) fn into_raw(self) -> *mut sys::hb_font_funcs_t {
        self.0
    }
}

impl<T> Clone for FontFuncs<T> {
    fn clone(&self) -> Self {
        FontFuncs(self.0, PhantomData)
    }
}

impl<T> Default for FontFuncs<T> {
    fn default() -> Self {
        Self::new()
    }
}
