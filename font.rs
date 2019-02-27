//! Derived from `hb-font.h`.

use libc;

use harfbuzz::face::{Face, hb_face_t};
use harfbuzz::common::{Position, Direction, Codepoint, hb_bool_t, hb_destroy_func_t};

define_boxed_type! {
    struct Font;
    enum hb_font_t;
    fn hb_font_get_empty;
    fn hb_font_reference;
    fn hb_font_destroy;
    fn hb_font_set_user_data;
    fn hb_font_get_user_data;
    fn hb_font_make_immutable;
    fn hb_font_is_immutable;
}

impl Font {
    pub fn new(face: Face) -> Font {
        unsafe {
            Font::from(hb_font_create(face.ptr))
        }
    }

    pub fn create_sub_font(&self) -> Font {
        unsafe {
            Font::from(hb_font_create_sub_font(self.ptr))
        }
    }

    pub fn parent(&self) -> Font {
        unsafe {
            Font::from(hb_font_get_parent(self.ptr))
        }
    }

    pub fn face(&self) -> Face {
        unsafe {
            Face::from(hb_font_get_face(self.ptr))
        }
    }

    /*pub unsafe fn set_funcs<T>(&mut self, class: FontFuncs, font_data: T) {
        let font_data = Some(font_data);
        let font_data_ptr = font_data.as_mut().map(|x| x as *mut _).unwrap_or(ptr::null_mut());
        let destroy = || {
            font_data.take();
        };
        hb_font_set_funcs(self.ptr, class.ptr, font_data_ptr, Some(destroy as hb_destroy_func_t__fn))
    }

    pub unsafe fn set_funcs_data<T>(&mut self, font_data: T) {
        let font_data = Some(font_data);
        let font_data_ptr = font_data.as_mut().map(|x| x as *mut _).unwrap_or(ptr::null_mut());
        let destroy = || {
            font_data.take();
        };
        hb_font_set_funcs_data(self.ptr, font_data_ptr, Some(destroy as hb_destroy_func_t__fn))
    }*/

    pub fn scale(&self) -> (i32, i32) {
        let mut x_scale = 0;
        let mut y_scale = 0;
        unsafe {
            hb_font_get_scale(self.ptr, &mut x_scale, &mut y_scale);
        }
        (x_scale, y_scale)
    }

    pub fn set_scale(&self, x_scale: i32, y_scale: i32) {
        unsafe {
            hb_font_set_scale(self.ptr, x_scale, y_scale);
        }
    }

    pub fn ppem(&self) -> (u32, u32) {
        let mut x_ppem = 0;
        let mut y_ppem = 0;
        unsafe {
            hb_font_get_ppem(self.ptr, &mut x_ppem, &mut y_ppem);
        }
        (x_ppem, y_ppem)
    }

    pub fn set_ppem(&self, x_ppem: u32, y_ppem: u32) {
        unsafe {
            hb_font_set_ppem(self.ptr, x_ppem, y_ppem);
        }
    }
}

impl Font {
    // XXX: unicode and variation_selector are both Codepoint, *should* they both be char?
    pub fn get_glyph(&self, unicode: char, variation_selector: char) -> Option<Codepoint> {
        unsafe {
            let mut out = 0;
            if hb_font_get_glyph(self.ptr, unicode as Codepoint, variation_selector as Codepoint,
                                 &mut out) != 0 {
                Some(out)
            } else {
                None
            }
        }
    }

    pub fn get_glyph_h_advance(&self, glyph: Codepoint) -> Position {
        unsafe {
            hb_font_get_glyph_h_advance(self.ptr, glyph)
        }
    }

    pub fn get_glyph_v_advance(&self, glyph: Codepoint) -> Position {
        unsafe {
            hb_font_get_glyph_v_advance(self.ptr, glyph)
        }
    }

    pub fn get_glyph_h_origin(&self, glyph: Codepoint) -> Option<(Position, Position)> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            if hb_font_get_glyph_h_origin(self.ptr, glyph, &mut x, &mut y) != 0 {
                Some((x, y))
            } else {
                None
            }
        }
    }

    pub fn get_glyph_v_origin(&self, glyph: Codepoint) -> Option<(Position, Position)> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            if hb_font_get_glyph_v_origin(self.ptr, glyph, &mut x, &mut y) != 0 {
                Some((x, y))
            } else {
                None
            }
        }
    }

    pub fn get_glyph_h_kerning(&self, left_glyph: Codepoint, right_glyph: Codepoint) -> Position {
        unsafe {
            hb_font_get_glyph_h_kerning(self.ptr, left_glyph, right_glyph)
        }
    }

    pub fn get_glyph_v_kerning(&self, top_glyph: Codepoint, bottom_glyph: Codepoint) -> Position {
        unsafe {
            hb_font_get_glyph_v_kerning(self.ptr, top_glyph, bottom_glyph)
        }
    }

    pub fn get_glyph_extents(&self, glyph: Codepoint) -> Option<GlyphExtents> {
        unsafe {
            let mut extents = GlyphExtents::default();
            if hb_font_get_glyph_extents(self.ptr, glyph, &mut extents) != 0 {
                Some(extents)
            } else {
                None
            }
        }
    }

    pub fn get_glyph_contour_point(&self, glyph: Codepoint, point_index: u32)
            -> Option<(Position, Position)> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            if hb_font_get_glyph_contour_point(self.ptr, glyph, point_index, &mut x, &mut y) != 0 {
                Some((x, y))
            } else {
                None
            }
        }
    }

    /*pub fn get_glyph_name(&self, glyph: Codepoint) -> Option<String> {
        unsafe {
            let size: u32 = …;
            if hb_font_get_glyph_name(self.ptr, glyph, &mut name, size) {
                Some(name)
            } else {
                None
            }
        }
    }

    fn hb_font_get_glyph_name(font: *mut hb_font_t, glyph: Codepoint, name: *mut libc::c_char,
                              size: libc::c_uint) -> hb_bool_t;
    fn hb_font_get_glyph_from_name(font: *mut hb_font_t, name: *const libc::c_char,
                                   len: libc::c_int, glyph: *mut Codepoint) -> hb_bool_t;
    */

    // high-level funcs, with fallback

    pub fn get_glyph_advance_for_direction(&self, glyph: Codepoint, direction: Direction)
            -> (Position, Position) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            hb_font_get_glyph_advance_for_direction(self.ptr, glyph, direction, &mut x, &mut y);
            (x, y)
        }
    }

    pub fn get_glyph_origin_for_direction(&self, glyph: Codepoint, direction: Direction)
            -> (Position, Position) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            hb_font_get_glyph_origin_for_direction(self.ptr, glyph, direction, &mut x, &mut y);
            (x, y)
        }
    }

    pub fn add_glyph_origin_for_direction(&self, glyph: Codepoint, direction: Direction,
                                          x: &mut Position, y: &mut Position) {
        unsafe {
            hb_font_add_glyph_origin_for_direction(self.ptr, glyph, direction, x, y)
        }
    }

    pub fn subtract_glyph_origin_for_direction(&self, glyph: Codepoint, direction: Direction,
                                               x: &mut Position, y: &mut Position) {
        unsafe {
            hb_font_subtract_glyph_origin_for_direction(self.ptr, glyph, direction, x, y)
        }
    }

    pub fn get_glyph_kerning_for_direction(&self, first_glyph: Codepoint, second_glyph: Codepoint,
                                           direction: Direction) -> (Position, Position) {
        unsafe {
            let mut x = 0;
            let mut y = 0;

            hb_font_get_glyph_kerning_for_direction(self.ptr, first_glyph, second_glyph, direction,
                                                    &mut x, &mut y);
            (x, y)
        }
    }

    pub fn get_glyph_extents_for_origin(&self, glyph: Codepoint, direction: Direction)
            -> Option<GlyphExtents> {
        unsafe {
            let mut extents = GlyphExtents::default();
            if hb_font_get_glyph_extents_for_origin(self.ptr, glyph, direction,
                                                    &mut extents) != 0 {
                Some(extents)
            } else {
                None
            }
        }
    }

    pub fn get_glyph_contour_point_for_origin(&self, glyph: Codepoint, point_index: u32,
                                              direction: Direction)
            -> Option<(Position, Position)> {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            if hb_font_get_glyph_contour_point_for_origin(self.ptr, glyph, point_index, direction,
                                                          &mut x, &mut y) != 0 {
                Some((x, y))
            } else {
                None
            }
        }
    }

    /*/// Generates gidDDD if glyph has no name.
    pub fn glyph_to_string(&self, glyph: Codepoint) -> String {
        let s: *mut libc::c_char;
        let size: u32;
        hb_font_glyph_to_string(self.ptr, glyph, s, size);
        // ???
    }*/

    /// Parses gidDDD and uniUUUU strings automatically.
    pub fn glyph_from_string(&self, s: &str) -> Option<Codepoint> {
        unsafe {
            let mut glyph = 0;
            if hb_font_glyph_from_string(self.ptr, s.as_ptr() as *const libc::c_char,
                                         s.len() as i32, &mut glyph) != 0 {
                Some(glyph)
            } else {
                None
            }
        }
    }
}

// hb_font_funcs_t

define_boxed_type! {
    struct FontFuncs;
    enum hb_font_funcs_t;
    fn hb_font_funcs_get_empty;
    fn hb_font_funcs_reference;
    fn hb_font_funcs_destroy;
    fn hb_font_funcs_set_user_data;
    fn hb_font_funcs_get_user_data;
    fn hb_font_funcs_make_immutable;
    fn hb_font_funcs_is_immutable;
}

impl FontFuncs {
    pub fn new() -> FontFuncs {
        unsafe {
            FontFuncs::from(hb_font_funcs_create())
        }
    }
}

/// Note that height is negative in coordinate systems that grow up.
///
/// Equivalent to `hb_glyph_extents_t`.
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct GlyphExtents {
    /// left side of glyph from origin.
    x_bearing: Position,
    /// top side of glyph from origin.
    y_bearing: Position,
    /// distance from left to right side.
    width: Position,
    /// distance from top to bottom side.
    height: Position,
}

// FFI:

extern "C" {
    fn hb_font_funcs_create() -> *mut hb_font_funcs_t;

    // func setters

    fn hb_font_funcs_set_glyph_func(ffuncs: *mut hb_font_funcs_t,
                                    func: hb_font_get_glyph_func_t,
                                    user_data: *mut libc::c_void,
                                    destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_h_advance_func(ffuncs: *mut hb_font_funcs_t,
                                              func: hb_font_get_glyph_h_advance_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_v_advance_func(ffuncs: *mut hb_font_funcs_t,
                                              func: hb_font_get_glyph_v_advance_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_h_origin_func(ffuncs: *mut hb_font_funcs_t,
                                             func: hb_font_get_glyph_h_origin_func_t,
                                             user_data: *mut libc::c_void,
                                             destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_v_origin_func(ffuncs: *mut hb_font_funcs_t,
                                             func: hb_font_get_glyph_v_origin_func_t,
                                             user_data: *mut libc::c_void,
                                             destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_h_kerning_func(ffuncs: *mut hb_font_funcs_t,
                                              func: hb_font_get_glyph_h_kerning_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_v_kerning_func(ffuncs: *mut hb_font_funcs_t,
                                              func: hb_font_get_glyph_v_kerning_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_extents_func(ffuncs: *mut hb_font_funcs_t,
                                            func: hb_font_get_glyph_extents_func_t,
                                            user_data: *mut libc::c_void,
                                            destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_contour_point_func(ffuncs: *mut hb_font_funcs_t,
                                                  func: hb_font_get_glyph_contour_point_func_t,
                                                  user_data: *mut libc::c_void,
                                                  destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_name_func(ffuncs: *mut hb_font_funcs_t,
                                         func: hb_font_get_glyph_name_func_t,
                                         user_data: *mut libc::c_void,
                                         destroy: hb_destroy_func_t);
    fn hb_font_funcs_set_glyph_from_name_func(ffuncs: *mut hb_font_funcs_t,
                                              func: hb_font_get_glyph_from_name_func_t,
                                              user_data: *mut libc::c_void,
                                              destroy: hb_destroy_func_t);

    // func dispatch

    fn hb_font_get_glyph(font: *mut hb_font_t, unicode: Codepoint,
                         variation_selector: Codepoint, glyph: *mut Codepoint) -> hb_bool_t;
    fn hb_font_get_glyph_h_advance(font: *mut hb_font_t, glyph: Codepoint) -> Position;
    fn hb_font_get_glyph_v_advance(font: *mut hb_font_t, glyph: Codepoint) -> Position;
    fn hb_font_get_glyph_h_origin(font: *mut hb_font_t, glyph: Codepoint,
                                  x: *mut Position, y: *mut Position) -> hb_bool_t;
    fn hb_font_get_glyph_v_origin(font: *mut hb_font_t, glyph: Codepoint,
                                  x: *mut Position, y: *mut Position) -> hb_bool_t;
    fn hb_font_get_glyph_h_kerning(font: *mut hb_font_t,
                                   left_glyph: Codepoint, right_glyph: Codepoint) -> Position;
    fn hb_font_get_glyph_v_kerning(font: *mut hb_font_t,
                                   top_glyph: Codepoint, bottom_glyph: Codepoint) -> Position;
    fn hb_font_get_glyph_extents(font: *mut hb_font_t,
                                 glyph: Codepoint, extents: *mut GlyphExtents) -> hb_bool_t;
    fn hb_font_get_glyph_contour_point(font: *mut hb_font_t, glyph: Codepoint,
                                       point_index: libc::c_uint,
                                       x: *mut Position, y: *mut Position) -> hb_bool_t;
    fn hb_font_get_glyph_name(font: *mut hb_font_t, glyph: Codepoint, name: *mut libc::c_char,
                              size: libc::c_uint) -> hb_bool_t;
    fn hb_font_get_glyph_from_name(font: *mut hb_font_t, name: *const libc::c_char,
                                   len: libc::c_int, glyph: *mut Codepoint) -> hb_bool_t;

    // high-level funcs, with fallback

    fn hb_font_get_glyph_advance_for_direction(font: *mut hb_font_t,
                                               glyph: Codepoint, direction: Direction,
                                               x: *mut Position, y: *mut Position);
    fn hb_font_get_glyph_origin_for_direction(font: *mut hb_font_t,
                                              glyph: Codepoint, direction: Direction,
                                              x: *mut Position, y: *mut Position);
    fn hb_font_add_glyph_origin_for_direction(font: *mut hb_font_t,
                                              glyph: Codepoint, direction: Direction,
                                              x: *mut Position, y: *mut Position);
    fn hb_font_subtract_glyph_origin_for_direction(font: *mut hb_font_t,
                                                   glyph: Codepoint, direction: Direction,
                                                   x: *mut Position, y: *mut Position);
    fn hb_font_get_glyph_kerning_for_direction(font: *mut hb_font_t, first_glyph: Codepoint,
                                               second_glyph: Codepoint, direction: Direction,
                                               x: *mut Position, y: *mut Position);
    fn hb_font_get_glyph_extents_for_origin(font: *mut hb_font_t, glyph: Codepoint,
                                            direction: Direction,
                                            extents: *mut GlyphExtents) -> hb_bool_t;
    fn hb_font_get_glyph_contour_point_for_origin(font: *mut hb_font_t, glyph: Codepoint,
                                                  point_index: libc::c_uint, direction: Direction,
                                                  x: *mut Position, y: *mut Position) -> hb_bool_t;

    // Generates gidDDD if glyph has no name.
    fn hb_font_glyph_to_string(font: *mut hb_font_t, glyph: Codepoint,
                               s: *mut libc::c_char, size: libc::c_uint);
    // Parses gidDDD and uniUUUU strings automatically.
    fn hb_font_glyph_from_string(font: *mut hb_font_t, s: *const libc::c_char,
                                 len: libc::c_int, glyph: *mut Codepoint) -> hb_bool_t;

    // hb_font_t
    // Fonts are very light-weight objects.

    fn hb_font_create(face: *mut hb_face_t) -> *mut hb_font_t;
    fn hb_font_create_sub_font(parent: *mut hb_font_t) -> *mut hb_font_t;
    fn hb_font_get_parent(font: *mut hb_font_t) -> *mut hb_font_t;
    fn hb_font_get_face(font: *mut hb_font_t) -> *mut hb_face_t;
    fn hb_font_set_funcs(font: *mut hb_font_t, klass: *mut hb_font_funcs_t,
                         font_data: *mut libc::c_void, destroy: hb_destroy_func_t);
    // Be *very* careful with this function!
    fn hb_font_set_funcs_data(font: *mut hb_font_t, font_data: *mut libc::c_void,
                              destroy: hb_destroy_func_t);
    fn hb_font_set_scale(font: *mut hb_font_t, x_scale: libc::c_int, y_scale: libc::c_int);
    fn hb_font_get_scale(font: *mut hb_font_t,
                         x_scale: *mut libc::c_int, y_scale: *mut libc::c_int);
    // A zero value means "no hinting in that direction"
    fn hb_font_set_ppem(font: *mut hb_font_t, x_ppem: libc::c_uint, y_ppem: libc::c_uint);
    fn hb_font_get_ppem(font: *mut hb_font_t,
                        x_ppem: *mut libc::c_uint, y_ppem: *mut libc::c_uint);
}

// func types

type hb_font_get_glyph_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                     font_data: *mut libc::c_void,
                                                     unicode: Codepoint,
                                                     variation_selector: Codepoint,
                                                     glyph: *mut Codepoint,
                                                     user_data: *mut libc::c_void)
                                                  -> hb_bool_t>;

type hb_font_get_glyph_advance_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                             font_data: *mut libc::c_void,
                                                             glyph: Codepoint,
                                                             user_data: *mut libc::c_void)
                                                          -> Position>;
type hb_font_get_glyph_h_advance_func_t = hb_font_get_glyph_advance_func_t;
type hb_font_get_glyph_v_advance_func_t = hb_font_get_glyph_advance_func_t;

type hb_font_get_glyph_origin_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                            font_data: *mut libc::c_void,
                                                            glyph: Codepoint,
                                                            x: *mut Position,
                                                            y: *mut Position,
                                                            user_data: *mut libc::c_void)
                                                         -> hb_bool_t>;
type hb_font_get_glyph_h_origin_func_t = hb_font_get_glyph_origin_func_t;
type hb_font_get_glyph_v_origin_func_t = hb_font_get_glyph_origin_func_t;

type hb_font_get_glyph_kerning_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                             font_data: *mut libc::c_void,
                                                             first_glyph: Codepoint,
                                                             second_glyph: Codepoint,
                                                             user_data: *mut libc::c_void)
                                                          -> Position>;
type hb_font_get_glyph_h_kerning_func_t = hb_font_get_glyph_kerning_func_t;
type hb_font_get_glyph_v_kerning_func_t = hb_font_get_glyph_kerning_func_t;

type hb_font_get_glyph_extents_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                             font_data: *mut libc::c_void,
                                                             glyph: Codepoint,
                                                             extents: *mut GlyphExtents,
                                                             user_data: *mut libc::c_void)
                                                          -> hb_bool_t>;
type hb_font_get_glyph_contour_point_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                                   font_data: *mut libc::c_void,
                                                                   glyph: Codepoint,
                                                                   point_index: libc::c_uint,
                                                                   x: *mut Position,
                                                                   y: *mut Position,
                                                                   user_data: *mut libc::c_void)
                                                                -> hb_bool_t>;
type hb_font_get_glyph_name_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                          font_data: *mut libc::c_void,
                                                          glyph: Codepoint,
                                                          name: *mut libc::c_char,
                                                          size: libc::c_uint,
                                                          user_data: *mut libc::c_void)
                                                       -> hb_bool_t>;
type hb_font_get_glyph_from_name_func_t = Option<extern "C" fn(font: *mut hb_font_t,
                                                               font_data: *mut libc::c_void,
                                                               name: *const libc::c_char,
                                                               len: libc::c_int,
                                                               glyph: *mut Codepoint,
                                                               user_data: *mut libc::c_void)
                                                            -> hb_bool_t>;
