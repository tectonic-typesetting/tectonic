use crate::c_api::{
    ttstub_input_close, ttstub_input_get_size, ttstub_input_open, ttstub_input_read, xbasename,
    xmalloc, xstrdup, RsFix2D, SyncPtr,
};
use libc::{strcpy, strlen, strrchr};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::sync::OnceLock;
use tectonic_bridge_core::FileFormat;
use tectonic_bridge_freetype2::{
    FT_Attach_Stream, FT_Face, FT_Face_GetCharVariantIndex, FT_Fixed, FT_Get_Advance,
    FT_Get_Char_Index, FT_Get_Glyph_Name, FT_Get_Kerning, FT_Get_Sfnt_Table, FT_Glyph_Format,
    FT_Init_FreeType, FT_Kerning_Mode, FT_LibraryRec, FT_Load_Glyph, FT_Load_Sfnt_Table,
    FT_New_Memory_Face, FT_Open_Args, FT_Sfnt_Tag, FT_Vector, TT_Postscript, FT_IS_SCALABLE,
    FT_IS_SFNT, FT_LOAD_NO_SCALE, FT_LOAD_VERTICAL_LAYOUT, FT_OPEN_MEMORY, TT_OS2,
};
use tectonic_bridge_harfbuzz::{
    hb_blob_create, hb_blob_t, hb_bool_t, hb_codepoint_t, hb_face_create_for_tables,
    hb_face_destroy, hb_face_set_index, hb_face_set_upem, hb_face_t, hb_font_create,
    hb_font_funcs_create, hb_font_funcs_set_glyph_contour_point_func,
    hb_font_funcs_set_glyph_extents_func, hb_font_funcs_set_glyph_h_advance_func,
    hb_font_funcs_set_glyph_h_kerning_func, hb_font_funcs_set_glyph_h_origin_func,
    hb_font_funcs_set_glyph_name_func, hb_font_funcs_set_glyph_v_advance_func,
    hb_font_funcs_set_glyph_v_kerning_func, hb_font_funcs_set_glyph_v_origin_func,
    hb_font_funcs_set_nominal_glyph_func, hb_font_funcs_set_variation_glyph_func, hb_font_funcs_t,
    hb_font_set_funcs, hb_font_set_ppem, hb_font_set_scale, hb_font_t, hb_glyph_extents_t,
    hb_memory_mode_t, hb_position_t, hb_tag_t,
};

pub unsafe extern "C" fn _get_nominal_glyph(
    _: *mut hb_font_t,
    font_data: *mut (),
    ch: hb_codepoint_t,
    gid: *mut hb_codepoint_t,
    _: *mut (),
) -> hb_bool_t {
    let face = font_data as FT_Face;
    *gid = FT_Get_Char_Index(face, ch);
    return (*gid != 0) as hb_bool_t;
}

pub unsafe extern "C" fn _get_variation_glyph(
    _: *mut hb_font_t,
    font_data: *mut (),
    ch: hb_codepoint_t,
    vs: hb_codepoint_t,
    gid: *mut hb_codepoint_t,
    _: *mut (),
) -> hb_bool_t {
    let face = font_data as FT_Face;
    *gid = FT_Face_GetCharVariantIndex(face, ch, vs);
    return (*gid != 0) as hb_bool_t;
}

#[no_mangle]
pub unsafe extern "C" fn _get_glyph_advance(
    face: FT_Face,
    gid: libc::c_uint,
    vertical: bool,
) -> FT_Fixed {
    let mut flags = FT_LOAD_NO_SCALE;

    if vertical {
        flags |= FT_LOAD_VERTICAL_LAYOUT
    }

    let mut advance = 0;
    let error = FT_Get_Advance(face, gid, flags, &mut advance);
    if error != 0 {
        advance = 0;
    }
    if vertical {
        advance = -advance;
    }

    advance
}

pub unsafe extern "C" fn _get_glyph_h_advance(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    _: *mut (),
) -> hb_position_t {
    _get_glyph_advance(font_data as FT_Face, gid, false)
}

pub unsafe extern "C" fn _get_glyph_v_advance(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    _: *mut (),
) -> hb_position_t {
    _get_glyph_advance(font_data as FT_Face, gid, true)
}

pub unsafe extern "C" fn _get_glyph_h_origin(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    _: *mut (),
) -> hb_bool_t {
    true as hb_bool_t
}

pub unsafe extern "C" fn _get_glyph_v_origin(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    _: *mut (),
) -> hb_bool_t {
    true as hb_bool_t

    // TODO
    // Keep the code below for reference, for now we want to keep vertical
    // origin at (0, 0) for compatibility with pre-0.9999.
    // Reconsider this (e.g. using BASE table) when we get around overhauling
    // the text directionality model and implementing real vertical typesetting.

    /*
    FT_Face face = (FT_Face) font_data;
    FT_Error error;

    error = FT_Load_Glyph (face, gid, FT_LOAD_NO_SCALE);
    if (!error) {
        *x = face->glyph->metrics.horiBearingX -   face->glyph->metrics.vertBearingX;
        *y = face->glyph->metrics.horiBearingY - (-face->glyph->metrics.vertBearingY);
    }

    return !error;
     */
}

pub unsafe extern "C" fn _get_glyph_h_kerning(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid1: hb_codepoint_t,
    gid2: hb_codepoint_t,
    _: *mut (),
) -> hb_position_t {
    let face = font_data as FT_Face;

    let mut kerning = FT_Vector::default();
    let error = FT_Get_Kerning(face, gid1, gid2, FT_Kerning_Mode::Unscaled, &mut kerning);
    if error != 0 {
        0
    } else {
        kerning.x
    }
}

pub unsafe extern "C" fn _get_glyph_v_kerning(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid1: hb_codepoint_t,
    gid2: hb_codepoint_t,
    _: *mut (),
) -> hb_bool_t {
    false as hb_bool_t
}

pub unsafe extern "C" fn _get_glyph_extents(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    extents: *mut hb_glyph_extents_t,
    _: *mut (),
) -> hb_bool_t {
    let face = font_data as FT_Face;

    let error = FT_Load_Glyph(face, gid, FT_LOAD_NO_SCALE) != 0;
    if !error {
        (*extents).x_bearing = (*(*face).glyph).metrics.horiBearingX;
        (*extents).y_bearing = (*(*face).glyph).metrics.horiBearingY;
        (*extents).width = (*(*face).glyph).metrics.width;
        (*extents).height = -(*(*face).glyph).metrics.height;
    }

    (!error) as hb_bool_t
}

pub unsafe extern "C" fn _get_glyph_contour_point(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    point_index: libc::c_uint,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    _: *mut (),
) -> hb_bool_t {
    let face = font_data as FT_Face;

    let error = FT_Load_Glyph(face, gid, FT_LOAD_NO_SCALE) != 0;
    (if !error
        && (*(*face).glyph).format == FT_Glyph_Format::Outline
        && point_index < ((*(*face).glyph).outline.n_points as u32)
    {
        *x = (*(*(*face).glyph).outline.points.add(point_index as usize)).x;
        *y = (*(*(*face).glyph).outline.points.add(point_index as usize)).y;
        true
    } else {
        false
    }) as hb_bool_t
}

pub unsafe extern "C" fn _get_glyph_name(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    name: *mut libc::c_char,
    size: libc::c_uint,
    _: *mut (),
) -> hb_bool_t {
    let face = font_data as FT_Face;

    let ret = !FT_Get_Glyph_Name(face, gid, name.cast(), size);
    if ret != 0 && (size != 0) && (*name == 0) {
        false as hb_bool_t
    } else {
        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn _get_font_funcs() -> *mut hb_font_funcs_t {
    static FONTS: OnceLock<SyncPtr<hb_font_funcs_t>> = OnceLock::new();

    FONTS
        .get_or_init(|| {
            let funcs = hb_font_funcs_create();

            hb_font_funcs_set_nominal_glyph_func(funcs, _get_nominal_glyph, ptr::null_mut(), None);
            hb_font_funcs_set_variation_glyph_func(
                funcs,
                _get_variation_glyph,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_h_advance_func(
                funcs,
                _get_glyph_h_advance,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_v_advance_func(
                funcs,
                _get_glyph_v_advance,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_h_origin_func(
                funcs,
                _get_glyph_h_origin,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_v_origin_func(
                funcs,
                _get_glyph_v_origin,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_h_kerning_func(
                funcs,
                _get_glyph_h_kerning,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_v_kerning_func(
                funcs,
                _get_glyph_v_kerning,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_extents_func(funcs, _get_glyph_extents, ptr::null_mut(), None);
            hb_font_funcs_set_glyph_contour_point_func(
                funcs,
                _get_glyph_contour_point,
                ptr::null_mut(),
                None,
            );
            hb_font_funcs_set_glyph_name_func(funcs, _get_glyph_name, ptr::null_mut(), None);

            SyncPtr(funcs)
        })
        .0
}

#[no_mangle]
pub unsafe extern "C" fn _get_table(
    _: *mut hb_face_t,
    tag: hb_tag_t,
    user_data: *mut (),
) -> *mut hb_blob_t {
    unsafe extern "C" fn table_free(ptr: *mut ()) {
        let ptr = Box::from_raw(ptr.cast::<(usize, *mut u8)>());
        dealloc(ptr.1.cast(), Layout::array::<libc::c_char>(ptr.0).unwrap())
    }

    let face = user_data as FT_Face;

    let mut length = 0;
    let error = FT_Load_Sfnt_Table(face, tag, 0, ptr::null_mut(), &mut length) != 0;

    let mut blob = ptr::null_mut();
    if !error {
        let table = alloc(Layout::array::<libc::c_char>(length as usize).unwrap());
        if !table.is_null() {
            let error = FT_Load_Sfnt_Table(face, tag, 0, table, &mut length) != 0;
            if !error {
                blob = hb_blob_create(
                    table.cast(),
                    length,
                    hb_memory_mode_t::Writable,
                    Box::into_raw(Box::new((length as usize, table))).cast(),
                    Some(table_free),
                );
            } else {
                dealloc(
                    table,
                    Layout::array::<libc::c_char>(length as usize).unwrap(),
                );
            }
        }
    }

    blob
}

/// cbindgen:rename-all=camelCase
#[repr(C)]
pub struct XeTeXFontBase {
    units_per_em: libc::c_ushort,
    point_size: f32,
    ascent: f32,
    descent: f32,
    cap_height: f32,
    x_height: f32,
    italic_angle: f32,

    vertical: bool,

    filename: *mut libc::c_char,
    index: u32,

    ft_face: FT_Face,
    backing_data: *mut libc::c_uchar,
    backing_data2: *mut libc::c_uchar,
    hb_font: *mut hb_font_t,
}

impl XeTeXFontBase {
    #[no_mangle]
    pub unsafe extern "C" fn initializeFont(
        &mut self,
        pathname: *const libc::c_char,
        index: libc::c_int,
        status: &mut libc::c_int,
    ) {
        static FREE_TYPE_LIBRARY: OnceLock<SyncPtr<FT_LibraryRec>> = OnceLock::new();

        let ft_lib = FREE_TYPE_LIBRARY
            .get_or_init(|| {
                let mut lib = ptr::null_mut();
                let error = FT_Init_FreeType(&mut lib);
                if error != 0 {
                    panic!("FreeType initialization failed, error {}", error);
                }
                SyncPtr(lib)
            })
            .0;

        let mut handle = ttstub_input_open(pathname, FileFormat::OpenType, 0);
        if handle.is_null() {
            handle = ttstub_input_open(pathname, FileFormat::TrueType, 0);
        }
        if handle.is_null() {
            handle = ttstub_input_open(pathname, FileFormat::Type1, 0);
        }
        if handle.is_null() {
            *status = 1;
            return;
        }

        let sz = ttstub_input_get_size(handle);
        self.backing_data = xmalloc(sz).cast();
        let r = ttstub_input_read(handle, self.backing_data.cast(), sz);
        if r < 0 || (r as usize) != sz {
            panic!("failed to read font file");
        }
        ttstub_input_close(handle);

        let error = FT_New_Memory_Face(
            ft_lib,
            self.backing_data,
            sz as libc::c_long,
            index,
            &mut self.ft_face,
        ) != 0;
        if error || !FT_IS_SCALABLE(self.ft_face) {
            *status = 1;
            return;
        }

        if index == 0 && !FT_IS_SFNT(self.ft_face) {
            let afm = xstrdup(xbasename(pathname));
            let p = strrchr(afm, b'.' as libc::c_int);
            if !p.is_null()
                && strlen(p) == 4
                && (*p.add(1) as u8).to_ascii_lowercase() == b'p'
                && (*p.add(2) as u8).to_ascii_lowercase() == b'f'
            {
                strcpy(p, (b".afm" as *const [u8]).cast());
            }

            let afm_handle = ttstub_input_open(afm, FileFormat::Afm, 0);
            libc::free(afm.cast());

            if !afm_handle.is_null() {
                let sz = ttstub_input_get_size(afm_handle);
                self.backing_data2 = xmalloc(sz).cast();
                let r = ttstub_input_read(handle, self.backing_data2.cast(), sz);

                if r < 0 || (r as usize) != sz {
                    panic!("failed to read AFM file");
                }

                let mut open_args = FT_Open_Args::default();
                open_args.flags = FT_OPEN_MEMORY;
                open_args.memory_base = self.backing_data2;
                open_args.memory_size = sz as libc::c_long;

                FT_Attach_Stream(self.ft_face, &mut open_args);
            }
        }

        self.filename = xstrdup(pathname);
        self.index = index as u32;
        self.units_per_em = (*self.ft_face).units_per_EM;
        self.ascent = self.units_to_points((*self.ft_face).ascender as f32);
        self.descent = self.units_to_points((*self.ft_face).descender as f32);

        let post_table = self
            .get_font_table(FT_Sfnt_Tag::Post)
            .cast::<TT_Postscript>();
        if !post_table.is_null() {
            self.italic_angle = RsFix2D((*post_table).italic_angle) as f32;
        }

        let os2_table = self.get_font_table(FT_Sfnt_Tag::Os2).cast::<TT_OS2>();
        if !os2_table.is_null() {
            self.cap_height = self.units_to_points((*os2_table).sCapHeight as f32);
            self.x_height = self.units_to_points((*os2_table).sxHeight as f32);
        }

        let hb_face = hb_face_create_for_tables(_get_table, self.ft_face.cast(), None);
        hb_face_set_index(hb_face, index as libc::c_uint);
        hb_face_set_upem(hb_face, self.units_per_em as libc::c_uint);
        self.hb_font = hb_font_create(hb_face);
        hb_face_destroy(hb_face);

        hb_font_set_funcs(self.hb_font, _get_font_funcs(), self.ft_face.cast(), None);
        hb_font_set_scale(
            self.hb_font,
            self.units_per_em as libc::c_int,
            self.units_per_em as libc::c_int,
        );
        hb_font_set_ppem(self.hb_font, 0, 0);
    }

    unsafe fn get_font_table(&self, tag: FT_Sfnt_Tag) -> *mut () {
        FT_Get_Sfnt_Table(self.ft_face, tag)
    }

    fn units_to_points(&self, units: f32) -> f32 {
        (units * self.point_size) / (self.units_per_em as f32)
    }

    fn points_to_units(&self, points: f32) -> f32 {
        (points * (self.units_per_em as f32)) / self.point_size
    }
}
