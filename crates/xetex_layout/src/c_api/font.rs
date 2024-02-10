#[cfg(not(target_os = "macos"))]
use crate::c_api::fc;
#[cfg(target_os = "macos")]
use crate::c_api::mac_core::{
    kCFStringEncodingUTF8, kCFTypeArrayCallBacks, kCFTypeDictionaryKeyCallBacks,
    kCFTypeDictionaryValueCallBacks, kCTFontCascadeListAttribute, kCTFontPostScriptNameKey,
    kCTFontURLAttribute, CFArrayCreate, CFDictionaryCreate, CFIndex, CFRelease, CFStringGetCString,
    CFStringGetLength, CFStringRef, CFURLGetFileSystemRepresentation, CGFloat, CTFontCopyAttribute,
    CTFontCopyName, CTFontCreateWithFontDescriptor, CTFontDescriptorCreateCopyWithAttributes,
    CTFontDescriptorRef, CTFontRef,
};
use crate::c_api::{
    ttstub_input_close, ttstub_input_get_size, ttstub_input_open, ttstub_input_read, xbasename,
    xcalloc, xmalloc, xstrdup, Fixed, GlyphBBox, GlyphID, OTTag, PlatformFontRef, RsD2Fix, RsFix2D,
    SyncPtr, XeTeXFont,
};
use libc::{free, strcpy, strlen, strrchr};
use std::alloc::{alloc, dealloc, Layout};
use std::cell::UnsafeCell;
use std::ffi::CStr;
use std::sync::OnceLock;
use std::{mem, ptr};
use tectonic_bridge_core::FileFormat;
use tectonic_bridge_freetype2::{
    FT_Attach_Stream, FT_BBox, FT_Done_Face, FT_Done_Glyph, FT_Face, FT_Face_GetCharVariantIndex,
    FT_Fixed, FT_Get_Advance, FT_Get_Char_Index, FT_Get_First_Char, FT_Get_Glyph,
    FT_Get_Glyph_Name, FT_Get_Kerning, FT_Get_Name_Index, FT_Get_Next_Char, FT_Get_Sfnt_Table,
    FT_Glyph_BBox_Mode, FT_Glyph_Format, FT_Glyph_Get_CBox, FT_Init_FreeType, FT_Kerning_Mode,
    FT_LibraryRec, FT_Load_Glyph, FT_Load_Sfnt_Table, FT_New_Memory_Face, FT_Open_Args,
    FT_Sfnt_Tag, FT_Vector, TT_Postscript, FT_HAS_GLYPH_NAMES, FT_IS_SCALABLE, FT_IS_SFNT,
    FT_LOAD_NO_SCALE, FT_LOAD_VERTICAL_LAYOUT, FT_OPEN_MEMORY, TT_OS2,
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
use tectonic_bridge_icu::UChar32;

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
    _: *mut (),
    _: hb_codepoint_t,
    _: *mut hb_position_t,
    _: *mut hb_position_t,
    _: *mut (),
) -> hb_bool_t {
    true as hb_bool_t
}

pub unsafe extern "C" fn _get_glyph_v_origin(
    _: *mut hb_font_t,
    _: *mut (),
    _: hb_codepoint_t,
    _: *mut hb_position_t,
    _: *mut hb_position_t,
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
    _: *mut (),
    _: hb_codepoint_t,
    _: hb_codepoint_t,
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
    match XeTeXFontBase::new(font_ref, RsFix2D(point_size) as f32) {
        Err(_) => ptr::null_mut(),
        Ok(out) => Box::into_raw(Box::new(out)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn createFontFromFile(
    filename: *const libc::c_char,
    index: libc::c_int,
    point_size: Fixed,
) -> XeTeXFont {
    let filename = if filename.is_null() {
        None
    } else {
        Some(CStr::from_ptr(filename))
    };

    match XeTeXFontBase::new_path_index(filename, index, RsFix2D(point_size) as f32) {
        Err(_) => ptr::null_mut(),
        Ok(out) => Box::into_raw(Box::new(out)),
    }
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
        script_count_sub
    } else {
        if !script_list.is_null() {
            *script_list = sl_pos;
        }
        script_count_pos
    }
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

enum FontKind {
    FtFont,
    #[cfg(target_os = "macos")]
    Mac(CTFontDescriptorRef, CTFontRef),
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

    kind: FontKind,
}

impl XeTeXFontBase {
    #[cfg(not(target_os = "macos"))]
    pub(crate) unsafe fn new(font: PlatformFontRef, point_size: f32) -> Result<XeTeXFontBase, i32> {
        let path = font.get::<fc::pat::File>(0).ok();
        let index = font.get::<fc::pat::Index>(0).unwrap_or(0);

        XeTeXFontBase::new_path_index(path, index, point_size)
    }

    #[cfg(target_os = "macos")]
    pub(crate) unsafe fn new(
        descriptor: CTFontDescriptorRef,
        point_size: f32,
    ) -> Result<XeTeXFontBase, i32> {
        let mut out = XeTeXFontBase {
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
            kind: FontKind::Mac(descriptor, ptr::null()),
        };
        let status = out.initialize_mac();
        if status != 0 {
            Err(status)
        } else {
            Ok(out)
        }
    }

    pub(crate) unsafe fn new_path_index(
        path: Option<&CStr>,
        index: libc::c_int,
        point_size: f32,
    ) -> Result<XeTeXFontBase, i32> {
        let mut out = XeTeXFontBase {
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
            kind: FontKind::FtFont,
        };
        let status = if let Some(path) = path {
            out.initialize_ft(path, index)
        } else {
            0
        };
        if status != 0 {
            Err(status)
        } else {
            Ok(out)
        }
    }

    unsafe fn initialize_ft(&mut self, pathname: &CStr, index: libc::c_int) -> i32 {
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

        let mut handle = ttstub_input_open(pathname.as_ptr(), FileFormat::OpenType, 0);
        if handle.is_null() {
            handle = ttstub_input_open(pathname.as_ptr(), FileFormat::TrueType, 0);
        }
        if handle.is_null() {
            handle = ttstub_input_open(pathname.as_ptr(), FileFormat::Type1, 0);
        }
        if handle.is_null() {
            return 1;
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
            return 1;
        }

        if index == 0 && !FT_IS_SFNT(self.ft_face) {
            let afm = xstrdup(xbasename(pathname.as_ptr()));
            let p = strrchr(afm, b'.' as libc::c_int);
            if !p.is_null()
                && strlen(p) == 4
                && (*p.add(1) as u8).to_ascii_lowercase() == b'p'
                && (*p.add(2) as u8).to_ascii_lowercase() == b'f'
            {
                strcpy(p, c!(".afm"));
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

        self.filename = xstrdup(pathname.as_ptr());
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

        0
    }

    #[cfg(not(target_os = "macos"))]
    pub unsafe fn initialize(&mut self, pathname: &CStr, index: libc::c_int) -> i32 {
        self.initialize_ft(pathname, index)
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn initialize_mac(&mut self) -> i32 {
        let FontKind::Mac(descriptor, font_ref) = &mut self.kind else {
            return 1;
        };

        let empty_cascade_list =
            CFArrayCreate(ptr::null(), ptr::null_mut(), 0, &kCFTypeArrayCallBacks);
        let mut values = &[empty_cascade_list];
        let mut attribute_keys = &[kCTFontCascadeListAttribute];
        let attributes = CFDictionaryCreate(
            ptr::null(),
            &mut (attribute_keys as *const [_]).cast(),
            &mut (values as *const [_]).cast(),
            1,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        CFRelease(empty_cascade_list.cast());

        *descriptor = CTFontDescriptorCreateCopyWithAttributes(*descriptor, attributes);
        CFRelease(attributes.cast());
        *font_ref = CTFontCreateWithFontDescriptor(
            *descriptor,
            (self.point_size * 72.0 / 72.27) as CGFloat,
            ptr::null(),
        );
        if !font_ref.is_null() {
            let mut index = 0;
            let pathname = CStr::from_ptr(getFileNameFromCTFont(*font_ref, &mut index));
            self.initialize_ft(pathname, index as libc::c_int)
        } else {
            CFRelease((*descriptor).cast());
            *descriptor = ptr::null();
            1
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn getFontTablePtr(font: XeTeXFont, table_tag: OTTag) -> *mut () {
        (*font).get_font_table_ot(table_tag)
    }

    #[no_mangle]
    pub unsafe extern "C" fn getSlant(font: XeTeXFont) -> Fixed {
        let angle = (*font).italic_angle() as f64;
        RsD2Fix(f64::tan(-angle * std::f64::consts::PI / 180.0))
    }

    #[no_mangle]
    pub unsafe extern "C" fn countGlyphs(font: XeTeXFont) -> libc::c_uint {
        (*font).get_num_glyphs() as libc::c_uint
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphWidth(font: XeTeXFont, gid: u32) -> f32 {
        (*font).get_glyph_width(gid)
    }

    #[no_mangle]
    pub unsafe extern "C" fn setFontLayoutDir(font: XeTeXFont, vertical: libc::c_int) {
        (*font).set_layout_dir_vertical(vertical != 0)
    }

    #[no_mangle]
    pub unsafe extern "C" fn getIndScript(font: XeTeXFont, index: libc::c_uint) -> hb_tag_t {
        let mut rval = 0;
        let mut script_list = ptr::null_mut();

        let script_count = getLargerScriptListTable(font, &mut script_list);
        if !script_list.is_null() && index < script_count {
            rval = *script_list.add(index as usize);
        }

        rval
    }

    #[no_mangle]
    pub unsafe extern "C" fn getIndLanguage(
        font: XeTeXFont,
        script: hb_tag_t,
        index: libc::c_uint,
    ) -> hb_tag_t {
        let mut rval = 0;
        let face = hb_font_get_face((*font).get_hb_font());
        let mut script_list = ptr::null_mut();

        let script_count = getLargerScriptListTable(font, &mut script_list);
        if !script_list.is_null() {
            for i in 0..script_count {
                if *script_list.add(i as usize) == script {
                    let mut lang_count = hb_ot_layout_script_get_language_tags(
                        face,
                        HB_OT_TAG_GSUB,
                        i,
                        0,
                        ptr::null_mut(),
                        ptr::null_mut(),
                    );
                    let mut lang_list = vec![0, lang_count];
                    hb_ot_layout_script_get_language_tags(
                        face,
                        HB_OT_TAG_GSUB,
                        i,
                        0,
                        &mut lang_count,
                        lang_list.as_mut_ptr(),
                    );

                    if index < lang_count {
                        rval = lang_list[index as usize];
                        break;
                    }

                    lang_count = hb_ot_layout_script_get_language_tags(
                        face,
                        HB_OT_TAG_GPOS,
                        i,
                        0,
                        ptr::null_mut(),
                        ptr::null_mut(),
                    );
                    lang_list.resize(lang_count as usize, 0);

                    hb_ot_layout_script_get_language_tags(
                        face,
                        HB_OT_TAG_GPOS,
                        i,
                        0,
                        &mut lang_count,
                        lang_list.as_mut_ptr(),
                    );

                    if index < lang_count {
                        rval = lang_list[index as usize];
                        break;
                    }
                }
            }
        }

        rval
    }

    #[no_mangle]
    pub unsafe extern "C" fn getIndFeature(
        font: XeTeXFont,
        script: hb_tag_t,
        language: hb_tag_t,
        mut index: libc::c_uint,
    ) -> hb_tag_t {
        let mut rval = 0;

        let face = hb_font_get_face((*font).get_hb_font());

        for i in 0..2 {
            let mut script_index = 0;
            let mut lang_index = 0;
            let table_tag = if i == 0 {
                HB_OT_TAG_GSUB
            } else {
                HB_OT_TAG_GPOS
            };

            if hb_ot_layout_table_find_script(face, table_tag, script, &mut script_index) != 0
                && (hb_ot_layout_script_select_language(
                    face,
                    table_tag,
                    script_index,
                    1,
                    &language,
                    &mut lang_index,
                ) != 0
                    || language == 0)
            {
                let mut feat_count = hb_ot_layout_language_get_feature_tags(
                    face,
                    table_tag,
                    script_index,
                    lang_index,
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                let mut feat_list = vec![0; feat_count as usize];

                hb_ot_layout_language_get_feature_tags(
                    face,
                    table_tag,
                    script_index,
                    lang_index,
                    0,
                    &mut feat_count,
                    feat_list.as_mut_ptr(),
                );

                if index < feat_count {
                    rval = feat_list[index as usize];
                    break;
                }

                index -= feat_count;
            }
        }

        rval
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphName(
        font: XeTeXFont,
        gid: u16,
        len: *mut libc::c_int,
    ) -> *const libc::c_char {
        (*font).get_glyph_name(gid, &mut *len)
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_font_units_to_points(font: XeTeXFont, units: f32) -> f32 {
        (*font).units_to_points(units as f64) as f32
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_font_points_to_units(font: XeTeXFont, points: f32) -> f32 {
        (*font).points_to_units(points as f64) as f32
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_font_get_point_size(font: XeTeXFont) -> f32 {
        (*font).point_size()
    }

    pub(crate) unsafe fn get_glyph_name(
        &self,
        gid: u16,
        len: &mut libc::c_int,
    ) -> *const libc::c_char {
        if FT_HAS_GLYPH_NAMES(self.ft_face) {
            thread_local! {
                static BUFFER: UnsafeCell<[u8; 256]> = UnsafeCell::new([0; 256]);
            }

            FT_Get_Glyph_Name(
                self.ft_face,
                gid as libc::c_uint,
                BUFFER.with(|b| b.get()).cast(),
                256,
            );
            *len = strlen(BUFFER.with(|b| b.get()).cast()) as libc::c_int;
            BUFFER.with(|b| b.get()).cast()
        } else {
            *len = 0;
            ptr::null()
        }
    }

    pub(crate) unsafe fn get_glyph_sidebearings(
        &self,
        gid: GlyphID,
        lsb: Option<&mut f32>,
        rsb: Option<&mut f32>,
    ) {
        let width = self.get_glyph_width(gid as u32);

        let mut bbox = GlyphBBox::default();
        self.get_glyph_bounds(gid, &mut bbox);

        if let Some(lsb) = lsb {
            *lsb = bbox.x_min;
        }
        if let Some(rsb) = rsb {
            *rsb = width - bbox.x_max;
        }
    }

    pub(crate) unsafe fn get_glyph_ital_corr(&self, gid: GlyphID) -> f32 {
        let width = self.get_glyph_width(gid as u32);
        let mut bbox = GlyphBBox::default();
        self.get_glyph_bounds(gid, &mut bbox);

        if bbox.x_max > width {
            bbox.x_max - width
        } else {
            0.0
        }
    }

    pub(crate) unsafe fn map_char_to_glyph(&self, ch: UChar32) -> GlyphID {
        FT_Get_Char_Index(self.ft_face, ch as libc::c_ulong) as GlyphID
    }

    pub(crate) unsafe fn get_first_char_code(&self) -> UChar32 {
        let mut gindex = 0;
        FT_Get_First_Char(self.ft_face, &mut gindex) as UChar32
    }

    pub(crate) unsafe fn get_last_char_code(&self) -> UChar32 {
        let mut gindex = 0;
        let mut ch = FT_Get_First_Char(self.ft_face, &mut gindex);
        let mut prev = ch;
        while gindex != 0 {
            prev = ch;
            ch = FT_Get_Next_Char(self.ft_face, ch, &mut gindex);
        }
        prev as UChar32
    }

    pub(crate) unsafe fn map_glyph_to_index(&self, glyph_name: *const libc::c_char) -> GlyphID {
        FT_Get_Name_Index(self.ft_face, glyph_name) as GlyphID
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

    pub(crate) unsafe fn get_glyph_bounds(&self, gid: GlyphID, bbox: &mut GlyphBBox) {
        bbox.x_min = 0.0;
        bbox.y_min = 0.0;
        bbox.x_max = 0.0;
        bbox.y_max = 0.0;

        let error = FT_Load_Glyph(self.ft_face, gid as libc::c_uint, FT_LOAD_NO_SCALE);
        if error != 0 {
            return;
        }

        let mut glyph = ptr::null_mut();
        let error = FT_Get_Glyph((*self.ft_face).glyph, &mut glyph);
        if error == 0 {
            let mut ft_bbox = FT_BBox::default();
            FT_Glyph_Get_CBox(glyph, FT_Glyph_BBox_Mode::Unscaled, &mut ft_bbox);
            bbox.x_min = self.units_to_points(ft_bbox.xMin as f64) as f32;
            bbox.y_min = self.units_to_points(ft_bbox.yMin as f64) as f32;
            bbox.x_max = self.units_to_points(ft_bbox.xMax as f64) as f32;
            bbox.y_max = self.units_to_points(ft_bbox.yMax as f64) as f32;
            FT_Done_Glyph(glyph);
        }
    }

    pub(crate) unsafe fn get_glyph_height_depth(
        &self,
        gid: GlyphID,
        height: Option<&mut f32>,
        depth: Option<&mut f32>,
    ) {
        let mut bbox = GlyphBBox::default();
        self.get_glyph_bounds(gid, &mut bbox);
        if let Some(height) = height {
            *height = bbox.y_max;
        }
        if let Some(depth) = depth {
            *depth = -bbox.y_min;
        }
    }

    pub(crate) fn get_filename(&self, index: &mut u32) -> *const libc::c_char {
        *index = self.index;
        self.filename
    }

    pub(crate) unsafe fn get_font_table(&self, tag: FT_Sfnt_Tag) -> *mut () {
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

    pub(crate) fn layout_dir_vertical(&self) -> bool {
        self.vertical
    }

    pub(crate) fn set_layout_dir_vertical(&mut self, vertical: bool) {
        self.vertical = vertical;
    }

    pub(crate) fn point_size(&self) -> f32 {
        self.point_size
    }

    pub(crate) fn ascent(&self) -> f32 {
        self.ascent
    }

    pub(crate) fn descent(&self) -> f32 {
        self.descent
    }

    pub(crate) fn cap_height(&self) -> f32 {
        self.cap_height
    }

    pub(crate) fn x_height(&self) -> f32 {
        self.x_height
    }

    pub(crate) fn get_hb_font(&self) -> *mut hb_font_t {
        self.hb_font
    }

    /* Tectonic: these are modified from the base XeTeX code to use doubles;
     * otherwise roundoff errors can accumulate leading to differences in the
     * XDV outputs. */
    pub(crate) fn units_to_points(&self, units: f64) -> f64 {
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
            #[cfg(target_os = "macos")]
            {
                if !self.descriptor.is_null() {
                    CFRelease(self.descriptor.cast());
                }
                if !self.font_ref.is_null() {
                    CFRelease(self.font_ref.cast());
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn getNameFromCTFont(
    ct_font_ref: CTFontRef,
    name_key: CFStringRef,
) -> *const libc::c_char {
    let name = CTFontCopyName(ct_font_ref, name_key);
    let mut len = CFStringGetLength(name);
    len = len * 6 + 1;
    let buf = xmalloc(len as _);
    if CFStringGetCString(name, buf, len, kCFStringEncodingUTF8) {
        buf
    } else {
        free(buf.cast());
        ptr::null()
    }
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn getFileNameFromCTFont(
    ct_font_ref: CTFontRef,
    index: *mut u32,
) -> *const libc::c_char {
    use std::cell::Cell;
    use tectonic_bridge_freetype2::{FT_Get_Postscript_Name, FT_Library, FT_New_Face};

    thread_local! {
        static FREE_TYPE_LIBRARY: Cell<FT_Library> = const { Cell::new(ptr::null_mut()) };
    }

    let mut url = ptr::null();

    #[cfg(feature = "MACOS_LE_10_6")]
    {
        let mut status;
        let ats_font = CTFontGetPlatformFont(ct_font_ref, ptr::null_mut());
        let mut fs_ref = 0;
        status = ATSFontGetFileReference(ats_font, &mut fsref);
        if status == noErr {
            url = CFUrlCreateFromFSRef(ptr::null_mut(), &mut fs_ref);
        }
    }
    #[cfg(not(feature = "MACOS_LE_10_6"))]
    {
        url = CTFontCopyAttribute(ct_font_ref, kCTFontURLAttribute);
    }

    if !url.is_null() {
        let mut pathname = [0u8; libc::PATH_MAX as usize];
        let ret = if CFURLGetFileSystemRepresentation(
            url.cast(),
            true,
            pathname.as_mut_ptr(),
            libc::PATH_MAX as CFIndex,
        ) {
            let mut face = ptr::null_mut();

            *index = 0;

            if FREE_TYPE_LIBRARY.get().is_null() {
                let mut library = ptr::null_mut();
                let error = FT_Init_FreeType(&mut library);
                if error != 0 {
                    panic!("FreeType initialization failed; error {}", error);
                } else {
                    FREE_TYPE_LIBRARY.set(library);
                }
            }

            let error = FT_New_Face(
                FREE_TYPE_LIBRARY.get(),
                pathname.as_ptr().cast(),
                0,
                &mut face,
            );
            if error == 0 && (*face).num_faces > 1 {
                let num_faces = (*face).num_faces;
                let ps_name1 = getNameFromCTFont(ct_font_ref, kCTFontPostScriptNameKey);
                *index = 0xFFFFFFFF;
                FT_Done_Face(face);
                for i in 0..num_faces {
                    let error = FT_New_Face(
                        FREE_TYPE_LIBRARY.get(),
                        pathname.as_ptr().cast(),
                        i,
                        &mut face,
                    );
                    if error == 0 {
                        let ps_name2 = FT_Get_Postscript_Name(face);
                        if (ps_name1.is_null() && ps_name2.is_null())
                            || (!ps_name1.is_null()
                                && !ps_name2.is_null()
                                && libc::strcmp(ps_name1, ps_name2) == 0)
                        {
                            *index = i as u32;
                            break;
                        }
                        FT_Done_Face(face);
                    }
                }
                free(ps_name1.cast::<libc::c_void>().cast_mut());
            }

            if *index != 0xFFFFFFFF {
                libc::strdup(pathname.as_ptr().cast())
            } else {
                ptr::null()
            }
        } else {
            ptr::null()
        };
        CFRelease(url);
        ret
    } else {
        ptr::null()
    }
}
