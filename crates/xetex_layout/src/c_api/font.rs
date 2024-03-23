#[cfg(not(target_os = "macos"))]
use crate::c_api::fc;
#[cfg(target_os = "macos")]
use crate::c_api::mac_core::{
    cf_to_cstr, kCFStringEncodingUTF8, kCFTypeArrayCallBacks, kCFTypeDictionaryKeyCallBacks,
    kCFTypeDictionaryValueCallBacks, kCTFontCascadeListAttribute, kCTFontPostScriptNameKey,
    kCTFontURLAttribute, CFArrayCreate, CFDictionaryCreate, CFIndex, CFRelease, CFStringGetCString,
    CFStringGetLength, CFStringRef, CFURLGetFileSystemRepresentation, CGFloat, CTFontCopyAttribute,
    CTFontCopyName, CTFontCreateWithFontDescriptor, CTFontDescriptorCreateCopyWithAttributes,
    CTFontDescriptorRef, CTFontRef,
};
use crate::c_api::{
    ttstub_input_close, ttstub_input_get_size, ttstub_input_open, ttstub_input_read, xbasename,
    xcalloc, xstrdup, Fixed, GlyphBBox, GlyphID, OTTag, PlatformFontRef, RawPlatformFontRef,
    RsD2Fix, RsFix2D, SyncPtr, XeTeXFont,
};
use libc::{free, strcpy, strlen, strrchr};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::sync::OnceLock;
use std::{mem, ptr, slice};
use tectonic_bridge_core::FileFormat;
use tectonic_bridge_freetype2 as ft;
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
    let face = &*font_data.cast::<ft::Face>();
    let out = match face.get_char_index(ch) {
        Some(cc) => {
            *gid = cc.get();
            true
        }
        None => {
            *gid = 0;
            false
        }
    };
    out as hb_bool_t
}

pub unsafe extern "C" fn _get_variation_glyph(
    _: *mut hb_font_t,
    font_data: *mut (),
    ch: hb_codepoint_t,
    vs: hb_codepoint_t,
    gid: *mut hb_codepoint_t,
    _: *mut (),
) -> hb_bool_t {
    let face = &*font_data.cast::<ft::Face>();
    let out = match face.get_char_variant_index(ch, vs) {
        Some(i) => {
            *gid = i.get();
            true
        }
        None => {
            *gid = 0;
            false
        }
    };
    out as hb_bool_t
}

pub extern "C" fn _get_glyph_advance(
    face: &ft::Face,
    gid: libc::c_uint,
    vertical: bool,
) -> ft::Fixed {
    let flags = if vertical {
        ft::LoadFlags::NO_SCALE | ft::LoadFlags::VERTICAL_LAYOUT
    } else {
        ft::LoadFlags::NO_SCALE
    };
    let out = match face.get_advance(gid, flags) {
        Ok(advance) => {
            if vertical {
                -advance
            } else {
                advance
            }
        }
        Err(_) => 0,
    };
    out as ft::Fixed
}

pub unsafe extern "C" fn _get_glyph_h_advance(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    _: *mut (),
) -> hb_position_t {
    _get_glyph_advance(&*font_data.cast::<ft::Face>(), gid, false) as hb_position_t
}

pub unsafe extern "C" fn _get_glyph_v_advance(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    _: *mut (),
) -> hb_position_t {
    _get_glyph_advance(&*font_data.cast::<ft::Face>(), gid, true) as hb_position_t
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
    let face = &*font_data.cast::<ft::Face>();

    match face.get_kerning(gid1, gid2, ft::KerningMode::Unscaled) {
        Ok(vec) => vec.x as hb_position_t,
        Err(_) => 0,
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
    let face = &mut *font_data.cast::<ft::Face>();

    let out = if face.load_glyph(gid, ft::LoadFlags::NO_SCALE).is_ok() {
        (*extents).x_bearing = face.glyph().metrics().horiBearingX as hb_position_t;
        (*extents).y_bearing = face.glyph().metrics().horiBearingY as hb_position_t;
        (*extents).width = face.glyph().metrics().width as hb_position_t;
        (*extents).height = -face.glyph().metrics().height as hb_position_t;
        true
    } else {
        false
    };

    out as hb_bool_t
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
    let face = &mut *font_data.cast::<ft::Face>();

    let error = face.load_glyph(gid, ft::LoadFlags::NO_SCALE).is_err();
    let out = if !error
        && face.glyph().format() == ft::GlyphFormat::Outline
        && point_index < (face.glyph().outline().n_points as u32)
    {
        *x = (*face.glyph().outline().points.add(point_index as usize)).x as hb_position_t;
        *y = (*face.glyph().outline().points.add(point_index as usize)).y as hb_position_t;
        true
    } else {
        false
    };
    out as hb_bool_t
}

pub unsafe extern "C" fn _get_glyph_name(
    _: *mut hb_font_t,
    font_data: *mut (),
    gid: hb_codepoint_t,
    name: *mut libc::c_char,
    size: libc::c_uint,
    _: *mut (),
) -> hb_bool_t {
    let face = &*font_data.cast::<ft::Face>();
    let buf = slice::from_raw_parts_mut(name.cast::<MaybeUninit<u8>>(), size as usize);

    let out = match face.get_glyph_name(gid, buf) {
        Ok(_) if size != 0 && *name == 0 => false,
        Err(_) => false,
        Ok(_) => true,
    };
    out as hb_bool_t
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
        let _ = Box::from_raw(ptr.cast::<Vec<u8>>());
    }

    let face = &*user_data.cast::<ft::Face>();

    let mut blob = ptr::null_mut();
    if let Ok(mut table) = face.load_sfnt_table(ft::TableTag::Other(tag)) {
        blob = hb_blob_create(
            table.as_mut_ptr().cast(),
            table.len() as libc::c_uint,
            hb_memory_mode_t::Writable,
            Box::into_raw(Box::new(table)).cast(),
            Some(table_free),
        );
    }

    blob
}

#[no_mangle]
pub unsafe extern "C" fn createFont(font_ref: RawPlatformFontRef, point_size: Fixed) -> XeTeXFont {
    let font_ref = match font_ref.try_into() {
        Ok(fr) => fr,
        Err(_) => return ptr::null_mut(),
    };

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

    let mut script_count_sub = hb_ot_layout_table_get_script_tags(
        face,
        HB_OT_TAG_GSUB,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    let sl_sub = xcalloc(script_count_sub as usize, mem::size_of::<*mut hb_tag_t>()).cast();
    hb_ot_layout_table_get_script_tags(face, HB_OT_TAG_GSUB, 0, &mut script_count_sub, sl_sub);

    let mut script_count_pos = hb_ot_layout_table_get_script_tags(
        face,
        HB_OT_TAG_GPOS,
        0,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    let sl_pos = xcalloc(script_count_pos as usize, mem::size_of::<*mut hb_tag_t>()).cast();
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
        if hb_ot_layout_table_find_script(face, table_tag, script, &mut script_idx) != 0
            && (hb_ot_layout_script_select_language(
                face,
                table_tag,
                script_idx,
                1,
                &language,
                &mut lang_idx,
            ) != 0
                || language == 0)
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

    filename: CString,
    index: u32,

    // Boxed to preserve address
    ft_face: Option<Box<ft::Face>>,
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
            filename: CString::new("").unwrap(),
            index: 0,
            ft_face: None,
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
            filename: CString::new("").unwrap(),
            index: 0,
            ft_face: None,
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
        let mut backing_data = vec![0; sz];
        let r = ttstub_input_read(handle, backing_data.as_mut_ptr().cast(), sz);
        if r < 0 || (r as usize) != sz {
            panic!("failed to read font file");
        }
        ttstub_input_close(handle);

        self.ft_face = match ft::Face::new_memory(backing_data, index as usize) {
            Ok(face) => Some(Box::new(face)),
            Err(_) => return 1,
        };

        if !self.ft_face().is_scalable() {
            return 1;
        }

        if index == 0 && !self.ft_face().is_sfnt() {
            // TODO: All this should use normal string manip
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
                let mut backing_data2 = vec![0; sz];
                let r = ttstub_input_read(handle, backing_data2.as_mut_ptr().cast(), sz);

                if r < 0 || (r as usize) != sz {
                    panic!("failed to read AFM file");
                }

                self.ft_face_mut().attach_stream_mem(backing_data2).unwrap();
            }
        }

        self.filename = pathname.to_owned();
        self.index = index as u32;
        self.units_per_em = self.ft_face().units_per_em();
        self.ascent = self.units_to_points(self.ft_face().ascender() as f64) as f32;
        self.descent = self.units_to_points(self.ft_face().descender() as f64) as f32;

        let post_table = self.get_font_table(ft::SfntTag::Post);
        if let Some(table) = post_table {
            self.italic_angle =
                RsFix2D(table.cast::<ft::tables::Postscript>().as_ref().italic_angle as Fixed)
                    as f32;
        }

        let os2_table = self.get_font_table(ft::SfntTag::Os2);
        if let Some(table) = os2_table {
            let table = table.cast::<ft::tables::OS2>().as_ref();
            self.cap_height = self.units_to_points(table.sCapHeight as f64) as f32;
            self.x_height = self.units_to_points(table.sxHeight as f64) as f32;
        }

        let hb_face = hb_face_create_for_tables(
            _get_table,
            // TODO: This is UB city
            ptr::from_mut(self.ft_face_mut()).cast(),
            None,
        );
        hb_face_set_index(hb_face, index as libc::c_uint);
        hb_face_set_upem(hb_face, self.units_per_em as libc::c_uint);
        self.hb_font = hb_font_create(hb_face);
        hb_face_destroy(hb_face);

        hb_font_set_funcs(
            self.hb_font,
            _get_font_funcs(),
            // TODO: This is UB city
            ptr::from_mut(self.ft_face_mut()).cast(),
            None,
        );
        hb_font_set_scale(
            self.hb_font,
            self.units_per_em as libc::c_int,
            self.units_per_em as libc::c_int,
        );
        hb_font_set_ppem(self.hb_font, 0, 0);

        0
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn initialize_mac(&mut self) -> i32 {
        let FontKind::Mac(descriptor, font_ref) = &mut self.kind else {
            return 1;
        };

        let empty_cascade_list =
            CFArrayCreate(ptr::null(), ptr::null_mut(), 0, &kCFTypeArrayCallBacks);
        let mut values = [empty_cascade_list];
        let mut attribute_keys = [kCTFontCascadeListAttribute];
        let attributes = CFDictionaryCreate(
            ptr::null(),
            attribute_keys.as_mut_ptr().cast(),
            values.as_mut_ptr().cast(),
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
    pub unsafe extern "C" fn hasFontTable(font: XeTeXFont, table_tag: OTTag) -> bool {
        // TODO: has_font_table for efficiency
        (*font)
            .load_font_table(ft::TableTag::Other(table_tag))
            .is_some()
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
        match (*font).get_glyph_name(gid) {
            Some(out) => {
                *len = out.as_bytes().len() as libc::c_int;
                CString::into_raw(out)
            }
            None => {
                *len = 0;
                ptr::null()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn freeGlyphName(name: *mut libc::c_char) {
        let _ = CString::from_raw(name);
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

    fn ft_face(&self) -> &ft::Face {
        self.ft_face.as_ref().unwrap()
    }

    fn ft_face_mut(&mut self) -> &mut ft::Face {
        self.ft_face.as_mut().unwrap()
    }

    pub(crate) fn get_glyph_name(&self, gid: u16) -> Option<CString> {
        if self.ft_face().has_glyph_names() {
            let mut buf = vec![0u8; 256];
            self.ft_face().get_glyph_name(gid as u32, &mut buf).unwrap();

            CStr::from_bytes_until_nul(&buf).map(CStr::to_owned).ok()
        } else {
            None
        }
    }

    pub(crate) fn get_glyph_sidebearings(
        &mut self,
        gid: GlyphID,
        lsb: Option<&mut f32>,
        rsb: Option<&mut f32>,
    ) {
        let width = self.get_glyph_width(gid as u32);

        let bbox = self.get_glyph_bounds(gid);

        if let Some(lsb) = lsb {
            *lsb = bbox.x_min;
        }
        if let Some(rsb) = rsb {
            *rsb = width - bbox.x_max;
        }
    }

    pub(crate) fn get_glyph_ital_corr(&mut self, gid: GlyphID) -> f32 {
        let width = self.get_glyph_width(gid as u32);
        let bbox = self.get_glyph_bounds(gid);

        if bbox.x_max > width {
            bbox.x_max - width
        } else {
            0.0
        }
    }

    pub(crate) fn map_char_to_glyph(&self, ch: UChar32) -> GlyphID {
        match self.ft_face().get_char_index(ch as u32) {
            Some(val) => val.get() as GlyphID,
            None => 0,
        }
    }

    pub(crate) fn get_first_char_code(&self) -> UChar32 {
        self.ft_face().get_first_char().0 as UChar32
    }

    pub(crate) fn get_last_char_code(&self) -> UChar32 {
        let ft_face = self.ft_face();

        let (mut ch, mut index) = ft_face.get_first_char();
        let mut prev = ch;
        while index != 0 {
            prev = ch;
            (ch, index) = ft_face.get_next_char(ch);
        }
        prev as UChar32
    }

    pub(crate) fn map_glyph_to_index(&self, glyph_name: &CStr) -> GlyphID {
        match self.ft_face().get_name_index(glyph_name) {
            Some(index) => index.get() as u16,
            None => 0,
        }
    }

    pub(crate) fn load_font_table(&self, tag: ft::TableTag) -> Option<Vec<u8>> {
        self.ft_face().load_sfnt_table(tag).ok()
    }

    pub(crate) fn get_glyph_bounds(&mut self, gid: GlyphID) -> GlyphBBox {
        let mut bbox = GlyphBBox::default();

        if self
            .ft_face_mut()
            .load_glyph(gid as u32, ft::LoadFlags::NO_SCALE)
            .is_err()
        {
            return bbox;
        }

        if let Ok(glyph) = self.ft_face().glyph().get_glyph() {
            let ft_bbox = glyph.get_cbox(ft::BBoxMode::Unscaled);
            bbox.x_min = self.units_to_points(ft_bbox.x_min as f64) as f32;
            bbox.y_min = self.units_to_points(ft_bbox.y_min as f64) as f32;
            bbox.x_max = self.units_to_points(ft_bbox.x_max as f64) as f32;
            bbox.y_max = self.units_to_points(ft_bbox.y_max as f64) as f32;
        }

        bbox
    }

    pub(crate) fn get_glyph_height_depth(
        &mut self,
        gid: GlyphID,
        height: Option<&mut f32>,
        depth: Option<&mut f32>,
    ) {
        let bbox = self.get_glyph_bounds(gid);
        if let Some(height) = height {
            *height = bbox.y_max;
        }
        if let Some(depth) = depth {
            *depth = -bbox.y_min;
        }
    }

    pub(crate) fn get_filename(&self, index: &mut u32) -> &CStr {
        *index = self.index;
        &self.filename
    }

    pub(crate) fn get_font_table(&self, tag: ft::SfntTag) -> Option<NonNull<()>> {
        self.ft_face().get_sfnt_table(tag)
    }

    pub(crate) fn italic_angle(&self) -> f32 {
        self.italic_angle
    }

    pub(crate) fn get_num_glyphs(&self) -> usize {
        self.ft_face().num_glyphs()
    }

    pub(crate) fn get_glyph_width(&self, gid: u32) -> f32 {
        self.units_to_points(_get_glyph_advance(self.ft_face(), gid, false) as f64) as f32
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
            hb_font_destroy(self.hb_font);
            #[cfg(target_os = "macos")]
            if let FontKind::Mac(descriptor, font_ref) = self.kind {
                if !descriptor.is_null() {
                    CFRelease(descriptor.cast());
                }
                if !font_ref.is_null() {
                    CFRelease(font_ref.cast());
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
    cf_to_cstr(name).into_raw()
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn getFileNameFromCTFont(
    ct_font_ref: CTFontRef,
    index: *mut u32,
) -> *const libc::c_char {
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
            let pathname = CStr::from_bytes_until_nul(&pathname).unwrap();
            *index = 0;

            let face = ft::Face::new(pathname, 0);
            if let Ok(face) = face {
                if face.num_faces() > 1 {
                    let num_faces = face.num_faces();
                    let ps_name1 = getNameFromCTFont(ct_font_ref, kCTFontPostScriptNameKey);
                    *index = 0xFFFFFFFF;
                    for i in 0..num_faces {
                        let face = ft::Face::new(pathname, i);
                        if let Ok(face) = face {
                            let ps_name2 = face.get_postscript_name();
                            if (ps_name1.is_null() && ps_name2.is_null())
                                || (!ps_name1.is_null()
                                    && !ps_name2.is_null()
                                    && libc::strcmp(ps_name1, ps_name2) == 0)
                            {
                                *index = i as u32;
                                break;
                            }
                        }
                    }
                    free(ps_name1.cast::<libc::c_void>().cast_mut());
                }
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
