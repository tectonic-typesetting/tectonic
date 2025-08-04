use crate::{
    Blob, Face, FaceRef, FontFuncs, FontFuncsRef, GlyphExtents, ImmutFontFuncs, Position, Tag,
};
use std::cell::RefCell;
use std::ffi::CString;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::OnceLock;
use tectonic_bridge_freetype2 as ft;

fn assets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/assets")
        .canonicalize()
        .unwrap()
}

fn get_face_data() -> Vec<u8> {
    let roman = assets_dir().join("lmroman12-regular.otf");
    fs::read(roman).unwrap()
}

fn get_face_file() -> ft::Face {
    let roman = assets_dir().join("lmroman12-regular.otf");
    let roman_path = CString::new(roman.to_str().unwrap()).unwrap();

    ft::Face::new(&roman_path, 0).unwrap()
}

fn get_face_mem() -> ft::Face {
    let roman_data = get_face_data();

    ft::Face::new_memory(roman_data, 0).unwrap()
}

pub fn test_faces() -> Vec<(Rc<RefCell<ft::Face>>, Face)> {
    fn tables(ft_face: Rc<RefCell<ft::Face>>) -> impl Fn(FaceRef<'_>, Tag) -> Option<Blob> {
        move |_, tag| {
            if let Ok(table) = ft_face
                .borrow()
                .load_sfnt_table(ft::TableTag::Other(tag.to_raw()))
            {
                Some(Blob::new(table))
            } else {
                None
            }
        }
    }

    let file = Rc::new(RefCell::new(get_face_file()));
    let mem = Rc::new(RefCell::new(get_face_mem()));

    vec![
        (Rc::clone(&file), Face::new_tables(tables(file))),
        (Rc::clone(&mem), Face::new_tables(tables(mem))),
    ]
}

fn get_glyph_advance(face: &ft::Face, gid: libc::c_uint, vertical: bool) -> ft::Fixed {
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

pub fn get_font_funcs() -> FontFuncsRef<'static, Rc<RefCell<ft::Face>>> {
    static FONTS: OnceLock<ImmutFontFuncs<Rc<RefCell<ft::Face>>>> = OnceLock::new();

    FONTS
        .get_or_init(|| {
            let mut funcs = FontFuncs::<Rc<RefCell<ft::Face>>>::new();

            let mut f = funcs.as_mut();
            f.nominal_glyph_func(|_, face, ch| face.borrow().get_char_index(ch).map(|cc| cc.get()));
            f.variation_glyph_func(|_, face, ch, vs| {
                face.borrow()
                    .get_char_variant_index(ch, vs)
                    .map(|cc| cc.get())
            });
            f.glyph_h_advance(|_, face, gid| {
                get_glyph_advance(&face.borrow(), gid, false) as Position
            });
            f.glyph_v_advance(|_, face, gid| {
                get_glyph_advance(&face.borrow(), gid, true) as Position
            });
            f.glyph_h_origin(|_, _, _| Some((0, 0)));
            f.glyph_v_origin(|_, _, _| Some((0, 0)));
            f.glyph_h_kerning(|_, face, gid1, gid2| {
                match face
                    .borrow()
                    .get_kerning(gid1, gid2, ft::KerningMode::Unscaled)
                {
                    Ok(vec) => vec.x as Position,
                    Err(_) => 0,
                }
            });
            f.glyph_v_kerning(|_, _, _, _| 0);
            f.glyph_extents(|_, face, gid| {
                let mut face = face.borrow_mut();
                if let Ok(glyph) = face.load_glyph(gid, ft::LoadFlags::NO_SCALE) {
                    Some(GlyphExtents {
                        x_bearing: glyph.metrics().horiBearingX as Position,
                        y_bearing: glyph.metrics().horiBearingY as Position,
                        width: glyph.metrics().width as Position,
                        height: -glyph.metrics().height as Position,
                    })
                } else {
                    None
                }
            });
            f.glyph_contour_point(|_, face, gid, point_index| {
                let mut face = face.borrow_mut();

                if let Ok(glyph) = face.load_glyph(gid, ft::LoadFlags::NO_SCALE) {
                    if let Some(outline) = glyph.outline() {
                        if point_index < (outline.n_points as u32) {
                            let x = outline.points()[point_index as usize].x as Position;
                            let y = outline.points()[point_index as usize].y as Position;
                            return Some((x, y));
                        }
                    }
                }
                None
            });
            f.glyph_name(
                |_, face, gid, buf| match face.borrow().get_glyph_name(gid, buf) {
                    Ok(str) if !str.to_bytes().is_empty() && str.to_bytes()[0] == 0 => 0,
                    Err(_) => 0,
                    Ok(str) => str.to_bytes().len(),
                },
            );

            funcs.make_immutable()
        })
        .as_ref()
}
