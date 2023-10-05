#[cfg(not(target_os = "macos"))]
use super::fc::*;
use crate::c_api::{
    ttstub_input_close, ttstub_input_get_size, ttstub_input_open, ttstub_input_read, xbasename,
    xcalloc, xmalloc, xstrdup, Fixed, OTTag, PlatformFontRef, RsFix2D, SyncPtr, XeTeXFont,
};
use libc::{free, strcpy, strlen, strrchr};
use std::alloc::{alloc, dealloc, Layout};
use std::sync::OnceLock;
use std::{mem, ptr};
use tectonic_bridge_core::FileFormat;
use tectonic_bridge_freetype2::{
    FT_Attach_Stream, FT_Done_Face, FT_Face, FT_Face_GetCharVariantIndex, FT_Fixed, FT_Get_Advance,
    FT_Get_Char_Index, FT_Get_Glyph_Name, FT_Get_Kerning, FT_Get_Sfnt_Table, FT_Glyph_Format,
    FT_Init_FreeType, FT_Kerning_Mode, FT_LibraryRec, FT_Load_Glyph, FT_Load_Sfnt_Table,
    FT_New_Memory_Face, FT_Open_Args, FT_Sfnt_Tag, FT_Vector, TT_Postscript, FT_IS_SCALABLE,
    FT_IS_SFNT, FT_LOAD_NO_SCALE, FT_LOAD_VERTICAL_LAYOUT, FT_OPEN_MEMORY, TT_OS2,
};
use tectonic_bridge_harfbuzz::{
    hb_blob_create, hb_blob_t, hb_bool_t, hb_codepoint_t, hb_face_create_for_tables,
    hb_face_destroy, hb_face_set_index, hb_face_set_upem, hb_face_t, hb_font_create,
    hb_font_destroy, hb_font_funcs_create, hb_font_funcs_set_glyph_contour_point_func,
    hb_font_funcs_set_glyph_extents_func, hb_font_funcs_set_glyph_h_advance_func,
    hb_font_funcs_set_glyph_h_kerning_func, hb_font_funcs_set_glyph_h_origin_func,
    hb_font_funcs_set_glyph_name_func, hb_font_funcs_set_glyph_v_advance_func,
    hb_font_funcs_set_glyph_v_kerning_func, hb_font_funcs_set_glyph_v_origin_func,
    hb_font_funcs_set_nominal_glyph_func, hb_font_funcs_set_variation_glyph_func, hb_font_funcs_t,
    hb_font_get_face, hb_font_set_funcs, hb_font_set_ppem, hb_font_set_scale, hb_font_t,
    hb_glyph_extents_t, hb_memory_mode_t, hb_ot_layout_language_get_feature_tags,
    hb_ot_layout_script_get_language_tags, hb_ot_layout_script_select_language,
    hb_ot_layout_table_find_script, hb_ot_layout_table_get_script_tags, hb_position_t, hb_tag_t,
    HB_OT_TAG_GPOS, HB_OT_TAG_GSUB,
};

pub unsafe extern "C" fn _get_nominal_glyph(
    _: *mut hb_font_t,
    font_data: *mut (),
    ch: hb_codepoint_t,
    gid: *mut hb_codepoint_t,
    _: *mut (),
) -> hb_bool_t {
    let face = font_data as FT_Face;
    *gid = FT_Get_Char_Index(face, ch as libc::c_ulong);
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
    *gid = FT_Face_GetCharVariantIndex(face, ch as libc::c_ulong, vs as libc::c_ulong);
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
    _get_glyph_advance(font_data as FT_Face, gid, false) as hb_position_t
}

pub unsafe extern "C" fn _get_glyph_v_advance(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    _: *mut (),
) -> hb_position_t {
    _get_glyph_advance(font_data as FT_Face, gid, true) as hb_position_t
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
        kerning.x as hb_position_t
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
        (*extents).x_bearing = (*(*face).glyph).metrics.horiBearingX as hb_position_t;
        (*extents).y_bearing = (*(*face).glyph).metrics.horiBearingY as hb_position_t;
        (*extents).width = (*(*face).glyph).metrics.width as hb_position_t;
        (*extents).height = -(*(*face).glyph).metrics.height as hb_position_t;
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
        *x = (*(*(*face).glyph).outline.points.add(point_index as usize)).x as hb_position_t;
        *y = (*(*(*face).glyph).outline.points.add(point_index as usize)).y as hb_position_t;
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
    let error =
        FT_Load_Sfnt_Table(face, tag as libc::c_ulong, 0, ptr::null_mut(), &mut length) != 0;

    let mut blob = ptr::null_mut();
    if !error {
        let table = alloc(Layout::array::<libc::c_char>(length as usize).unwrap());
        if !table.is_null() {
            let error = FT_Load_Sfnt_Table(face, tag as libc::c_ulong, 0, table, &mut length) != 0;
            if !error {
                blob = hb_blob_create(
                    table.cast(),
                    length as libc::c_uint,
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

#[no_mangle]
pub unsafe extern "C" fn createFont(font_ref: PlatformFontRef, point_size: Fixed) -> XeTeXFont {
    let mut status = 0;
    let out;
    #[cfg(target_os = "macos")]
    {
        /*
        XeTeXFontInst* font = new XeTeXFontInst_Mac(fontRef, RsFix2D(pointSize), status);
         */
        todo!()
    }
    #[cfg(not(target_os = "macos"))]
    {
        let mut pathname = ptr::null();
        FcPatternGetString(font_ref, FC_FILE, 0, &mut pathname);
        let mut index = 0;
        FcPatternGetInteger(font_ref, FC_INDEX, 0, &mut index);

        out = Box::into_raw(Box::new(XeTeXFontBase::new(
            pathname,
            index,
            RsFix2D(point_size) as f32,
            &mut status,
        )));
    }

    if status != 0 {
        let _ = Box::from_raw(out);
        return ptr::null_mut();
    }

    out
}

#[no_mangle]
pub unsafe extern "C" fn createFontFromFile(
    filename: *const libc::c_char,
    index: libc::c_int,
    point_size: Fixed,
) -> XeTeXFont {
    let mut status = 0;
    let out = Box::into_raw(Box::new(XeTeXFontBase::new(
        filename,
        index,
        RsFix2D(point_size) as f32,
        &mut status,
    )));
    if status != 0 {
        let _ = Box::from_raw(out);
        return ptr::null_mut();
    }
    out
}

#[no_mangle]
pub unsafe extern "C" fn deleteFont(font: XeTeXFont) {
    let _ = Box::from_raw(font);
}

#[no_mangle]
pub unsafe extern "C" fn getLargerScriptListTable(
    font: XeTeXFont,
    script_list: *mut *mut hb_tag_t,
) -> libc::c_uint {
    let mut rval = 0;

    let face = hb_font_get_face((*font).get_hb_font());

    let sl_sub;
    let sl_pos;

    let mut script_count_sub = hb_ot_layout_table_get_script_tags(
        face,
        HB_OT_TAG_GSUB,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    sl_sub = xcalloc(script_count_sub as usize, mem::size_of::<*mut hb_tag_t>()).cast();
    hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GSUB, 0, &mut script_count_sub, sl_sub);

    let mut script_count_pos = hb_ot_layout_table_get_script_tags(
        face,
        HB_OT_TAG_GPOS,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    sl_pos = xcalloc(script_count_pos as usize, mem::size_of::<*mut hb_tag_t>()).cast();
    hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GPOS, 0, &mut script_count_pos, sl_pos);

    if script_count_sub > script_count_pos {
        if !script_list.is_null() {
            *script_list = sl_sub;
        }
        rval = script_count_sub;
    } else {
        if !script_list.is_null() {
            *script_list = sl_pos;
        }
        rval = script_count_pos;
    }

    rval
}

#[no_mangle]
pub unsafe extern "C" fn countScripts(font: XeTeXFont) -> libc::c_uint {
    getLargerScriptListTable(font, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn countLanguages(font: XeTeXFont, script: hb_tag_t) -> libc::c_uint {
    let mut rval = 0;

    let face = hb_font_get_face((*font).get_hb_font());

    let mut script_list = ptr::null_mut();
    let count = getLargerScriptListTable(font, &mut script_list);
    if !script_list.is_null() {
        for i in 0..count {
            if *script_list.add(i as usize) == script {
                rval += hb_ot_layout_script_get_language_tags(
                    face,
                    HB_OT_TAG_GSUB,
                    i,
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                rval += hb_ot_layout_script_get_language_tags(
                    face,
                    HB_OT_TAG_GPOS,
                    i,
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                break;
            }
        }
    }

    rval
}

#[no_mangle]
pub unsafe extern "C" fn countFeatures(
    font: XeTeXFont,
    script: hb_tag_t,
    language: hb_tag_t,
) -> libc::c_uint {
    let mut rval = 0;

    let face = hb_font_get_face((*font).get_hb_font());

    for i in 0..2 {
        let mut script_idx = 0;
        let mut lang_idx = 0;

        let table_tag = if i == 0 {
            HB_OT_TAG_GSUB
        } else {
            HB_OT_TAG_GPOS
        };
        if hb_ot_layout_table_find_script(face, table_tag, script, &mut script_idx) != 0 {
            if hb_ot_layout_script_select_language(
                face,
                table_tag,
                script_idx,
                1,
                &language,
                &mut lang_idx,
            ) != 0
                || language == 0
            {
                rval += hb_ot_layout_language_get_feature_tags(
                    face,
                    table_tag,
                    script_idx,
                    lang_idx,
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
            }
        }
    }
    rval
}

/// cbindgen:rename-all=camelCase
#[repr(C)]
pub struct XeTeXFontBase {
    // TODO: This is UB to rely on layout
    vtable: *mut (),

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
    pub(crate) unsafe fn new(
        path: *const libc::c_char,
        index: libc::c_int,
        point_size: f32,
        status: &mut libc::c_int,
    ) -> XeTeXFontBase {
        let mut out = XeTeXFontBase {
            vtable: ptr::null_mut(),
            units_per_em: 0,
            point_size,
            ascent: 0.0,
            descent: 0.0,
            cap_height: 0.0,
            x_height: 0.0,
            italic_angle: 0.0,
            vertical: false,
            filename: ptr::null_mut(),
            index: 0,
            ft_face: ptr::null_mut(),
            backing_data: ptr::null_mut(),
            backing_data2: ptr::null_mut(),
            hb_font: ptr::null_mut(),
        };
        if !path.is_null() {
            out.initializeFont(path, index, status);
        }
        out
    }

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
            index as libc::c_long,
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
            free(afm.cast());

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
        self.ascent = self.units_to_points((*self.ft_face).ascender as f64) as f32;
        self.descent = self.units_to_points((*self.ft_face).descender as f64) as f32;

        let post_table = self
            .get_font_table(FT_Sfnt_Tag::Post)
            .cast::<TT_Postscript>();
        if !post_table.is_null() {
            self.italic_angle = RsFix2D((*post_table).italic_angle as Fixed) as f32;
        }

        let os2_table = self.get_font_table(FT_Sfnt_Tag::Os2).cast::<TT_OS2>();
        if !os2_table.is_null() {
            self.cap_height = self.units_to_points((*os2_table).sCapHeight as f64) as f32;
            self.x_height = self.units_to_points((*os2_table).sxHeight as f64) as f32;
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

    pub(crate) unsafe fn get_font_table_ot(&self, tag: OTTag) -> *mut () {
        let mut tmp_len = 0;
        let error = FT_Load_Sfnt_Table(
            self.ft_face,
            tag as libc::c_ulong,
            0,
            ptr::null_mut(),
            &mut tmp_len,
        );
        if error != 0 {
            return ptr::null_mut();
        }

        let table = xmalloc(tmp_len as usize);
        if !table.is_null() {
            let error = FT_Load_Sfnt_Table(
                self.ft_face,
                tag as libc::c_ulong,
                0,
                table.cast(),
                &mut tmp_len,
            );
            if error != 0 {
                free(table.cast());
                return ptr::null_mut();
            }
        }

        table.cast()
    }

    unsafe fn get_font_table(&self, tag: FT_Sfnt_Tag) -> *mut () {
        FT_Get_Sfnt_Table(self.ft_face, tag)
    }

    pub(crate) fn italic_angle(&self) -> f32 {
        self.italic_angle
    }

    pub(crate) unsafe fn get_num_glyphs(&self) -> u32 {
        (*self.ft_face).num_glyphs as u32
    }

    pub(crate) unsafe fn get_glyph_width(&self, gid: u32) -> f32 {
        self.units_to_points(_get_glyph_advance(self.ft_face, gid, false) as f64) as f32
    }

    pub(crate) fn set_layout_dir_vertical(&mut self, vertical: bool) {
        self.vertical = vertical;
    }

    pub(crate) fn get_hb_font(&self) -> *mut hb_font_t {
        self.hb_font
    }

    fn units_to_points(&self, units: f64) -> f64 {
        (units * self.point_size as f64) / (self.units_per_em as f64)
    }

    fn points_to_units(&self, points: f64) -> f64 {
        (points * (self.units_per_em as f64)) / self.point_size as f64
    }
}

impl Drop for XeTeXFontBase {
    fn drop(&mut self) {
        unsafe {
            if !self.ft_face.is_null() {
                FT_Done_Face(self.ft_face);
            }
            hb_font_destroy(self.hb_font);
            free(self.backing_data.cast());
            free(self.backing_data2.cast());
            free(self.filename.cast());
        }
    }
}
