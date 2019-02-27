//! Derived from `hb-ot.h` and `hb-ot-*.h`.

use harfbuzz::Tag;
use harfbuzz::face::hb_face_t;
use harfbuzz::font::hb_font_t;

extern "C" {
    fn hb_ot_font_set_funcs(font: *mut hb_font_t);

    fn hb_ot_layout_has_glyph_classes(face: *mut hb_face_t) -> hb_bool_t;

    fn hb_ot_layout_get_glyph_class(face: *mut hb_face_t,
                                    glyph: hb_codepoint_t) -> hb_ot_layout_glyph_class_t

    fn hb_ot_layout_get_glyphs_in_class(face: *mut hb_face_t,
                                        klass: hb_ot_layout_glyph_class_t,
                                        glyphs: *mut hb_set_t /* out */);


    fn hb_ot_layout_get_attach_points(face: *mut hb_face_t,
                                      glyph: hb_codepoint_t,
                                      start_offset: libc::c_uint,
                                      point_count: *mut libc::c_uint /* in/out */,
                                      point_array: *mut libc::c_uint /* out */) -> libc::c_uint;

    fn hb_ot_layout_get_ligature_carets(font: *mut hb_font_t,
                                        direction: hb_direction_t,
                                        glyph: hb_codepoint_t,
                                        start_offset: libc::c_uint,
                                        caret_count: *mut libc::c_uint /* IN/OUT */,
                                        caret_array: *mut hb_position_t /* OUT */) -> libc::c_uint;

    fn hb_ot_layout_table_get_script_tags(face: *mut hb_face_t,
                                          table_tag: hb_tag_t,
                                          start_offset: libc::c_uint,
                                          script_count: *mut libc::c_uint /* IN/OUT */,
                                          script_tags: *mut hb_tag_t /* OUT */) -> libc::c_uint;
    
    fn hb_ot_layout_table_find_script(face: *mut hb_face_t,
                                      table_tag: hb_tag_t,
                                      script_tag: hb_tag_t,
                                      script_index: *mut libc::c_uint) -> hb_bool_t;

}

// Derived from hb-ot-font.h

#[inline]
pub fn font_set_funcs(font: &mut Font) {
    unsafe {
        hb_ot_font_set_funcs(font.ptr)
    }
}

// Derived from hb-ot-layout.h

/// Equivalent to `HB_OT_TAG_GDEF`.
pub const TAG_GDEF: Tag = hb_tag!('G', 'D', 'E', 'F');

/// Equivalent to `HB_OT_TAG_GDEF`.
pub const TAG_GSUB: Tag = hb_tag!('G', 'S', 'U', 'B');

/// Equivalent to `HB_OT_TAG_GDEF`.
pub const TAG_GPOS: Tag = hb_tag!('G', 'P', 'O', 'S');

/// Equivalent to `HB_OT_TAG_GDEF`.
pub const TAG_JSTF: Tag = hb_tag!('J', 'S', 'T', 'F');


// GDEF

/// Equivalent to `hb_ot_layout_has_glyph_classes`.
#[inline]
pub fn has_glyph_classes(face: &Face) -> bool {
    unsafe {
        hb_ot_layout_has_glyph_classes(face.ptr) != 0
    }
}

/// Equivalent to `hb_ot_layout_glyph_class_t`.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub enum GlyphClass {
  /// Equivalent to `HB_OT_LAYOUT_GLYPH_CLASS_UNCLASSIFIED`.
  Unclassified = 0,
  /// Equivalent to `HB_OT_LAYOUT_GLYPH_CLASS_BASE_GLYPH`
  BaseGlyph = 1,
  /// Equivalent to `HB_OT_LAYOUT_GLYPH_CLASS_LIGATURE`
  Ligature = 2,
  /// Equivalent to `HB_OT_LAYOUT_GLYPH_CLASS_MARK`
  Mark = 3,
  /// Equivalent to `HB_OT_LAYOUT_GLYPH_CLASS_COMPONENT`
  Component = 4,
}

pub fn get_glyph_class(face: &Face, glyph: hb_codepoint_t) -> GlyphClass { TODO }
pub fn get_glyphs_in_class(face: &Face, class: GlyphClass) -> Set { TODO }
/// Not that useful.  Provides list of attach points for a glyph that a client may want to cache.
pub fn get_attach_points(face: &Face, glyph: hb_codepoint_t, start_offset: u32, ???) -> ??? { TODO }
/// Ligature caret positions
pub fn get_ligature_carets(face: &Face, direction: Direction, glyph: hb_codepoint_t, start_offset: u32, ???) -> ??? { TODO }

// GSUB/GPOS feature query and enumeration interface

/// Equivalent to `HB_OT_LAYOUT_NO_SCRIPT_INDEX`.
const NO_SCRIPT_INDEX: u32 = 0xFFFF;
/// Equivalent to `HB_OT_LAYOUT_NO_FEATURE_INDEX`.
const NO_FEATURE_INDEX: u32 = 0xFFFF;
/// Equivalent to `HB_OT_LAYOUT_DEFAULT_LANGUAGE_INDEX`.
const DEFAULT_LANGUAGE_INDEX: u32 = 0xFFFF;

pub fn table_get_script_tags(face: &Face, table_tag: Tag, start_offset: u32, ???) -> ??? { TODO }
pub fn table_find_script(face: &Face, table_tag: Tag, script_tag: Tag) -> ??? { TODO }

// Derived from hb-ot-tag.h

// Derived from hb-ot-shape.h

// XXX XXX XXX XXX XXX ↓ scratch area ↓ XXX XXX XXX XXX XXX


/// Like find_script, but takes zero-terminated array of scripts to test
fn hb_ot_layout_table_choose_script(face: *mut hb_face_t,
                                    table_tag: hb_tag_t,
                                    script_tags: *const hb_tag_t,
                                    script_index: *mut libc::c_uint,
                                    chosen_script: *mut hb_tag_t) -> hb_bool_t;

fn hb_ot_layout_table_get_feature_tags(face: *mut hb_face_t,
                                       table_tag: hb_tag_t,
                                       start_offset: libc::c_uint,
                                       feature_count: *mut libc::c_uint /* IN/OUT */,
                                       feature_tags: *mut hb_tag_t /* OUT */) -> libc::c_uint;

fn hb_ot_layout_script_get_language_tags(face: *mut hb_face_t,
                                         table_tag: hb_tag_t,
                                         script_index: libc::c_uint,
                                         start_offset: libc::c_uint,
                                         language_count: *mut libc::c_uint /* IN/OUT */,
                                         language_tags: *mut hb_tag_t /* OUT */) -> libc::c_uint;

fn hb_ot_layout_script_find_language(face: *mut hb_face_t,
                                     table_tag: hb_tag_t,
                                     script_index: libc::c_uint,
                                     language_tag: hb_tag_t,
                                     language_index: *mut libc::c_uint) -> hb_bool_t;

fn hb_ot_layout_language_get_required_feature_index(face: *mut hb_face_t,
                                                    table_tag: hb_tag_t,
                                                    script_index: libc::c_uint,
                                                    language_index: libc::c_uint,
                                                    feature_index: *mut libc::c_uint) -> hb_bool_t;

fn hb_ot_layout_language_get_required_feature(face: *mut hb_face_t,
                                              table_tag: hb_tag_t,
                                              script_index: libc::c_uint,
                                              language_index: libc::c_uint,
                                              feature_index: *mut libc::c_uint,
                                              feature_tag: *mut hb_tag_t) -> hb_bool_t;

fn hb_ot_layout_language_get_feature_indexes(face: *mut hb_face_t,
                                             table_tag: hb_tag_t,
                                             script_index: libc::c_uint,
                                             language_index: libc::c_uint,
                                             start_offset: libc::c_uint,
                                             feature_count: *mut libc::c_uint /* IN/OUT */,
                                             feature_indexes: *mut libc::c_uint /* OUT */) -> libc::c_uint;

fn hb_ot_layout_language_get_feature_tags(face: *mut hb_face_t,
                                          table_tag: hb_tag_t,
                                          script_index: libc::c_uint,
                                          language_index: libc::c_uint,
                                          start_offset: libc::c_uint,
                                          feature_count: *mut libc::c_uint /* IN/OUT */,
                                          feature_tags: *mut hb_tag_t /* OUT */) -> libc::c_uint;

fn hb_ot_layout_language_find_feature(face: *mut hb_face_t,
                                      table_tag: hb_tag_t,
                                      script_index: libc::c_uint,
                                      language_index: libc::c_uint,
                                      feature_tag: hb_tag_t,
                                      feature_index: *mut libc::c_uint) -> hb_bool_t;

fn hb_ot_layout_feature_get_lookups(face: *mut hb_face_t,
                                    table_tag: hb_tag_t,
                                    feature_index: libc::c_uint,
                                    start_offset: libc::c_uint,
                                    lookup_count: *mut libc::c_uint /* IN/OUT */,
                                    lookup_indexes: *mut libc::c_uint /* OUT */) -> hb_bool_t;

fn hb_ot_layout_table_get_lookup_count(face: *mut hb_face_t,
                                       table_tag: hb_tag_t) -> libc::c_uint;


fn hb_ot_layout_collect_lookups(face: *mut hb_face_t,
			      table_tag: hb_tag_t,
			      scripts: *const hb_tag_t,
			      languages: *const hb_tag_t,
			      features: *const hb_tag_t,
			      lookup_indexes: *mut hb_set_t, /* OUT */);

fn hb_ot_layout_lookup_collect_glyphs(face: *mut hb_face_t,
				    table_tag: hb_tag_t,
				    lookup_index: libc::c_uint,
				    glyphs_before: *mut hb_set_t, /* OUT. May be NULL */
				    glyphs_input: *mut hb_set_t,  /* OUT. May be NULL */
				    glyphs_after: *mut hb_set_t,  /* OUT. May be NULL */
				    glyphs_output: *mut hb_set_t  /* OUT. May be NULL */);

#[cfg(hb_not_implemented)]
#[repr(C)]
struct hb_ot_layout_glyph_sequence_t {
  before: *const hb_codepoint_t,
  before_length: libc::c_uint,
  input: *const hb_codepoint_t,
  input_length: libc::c_uint,
  after: *const hb_codepoint_t,
  after_length: libc::c_uint,
}

#[cfg(hb_not_implemented)]
type hb_ot_layout_glyph_sequence_func_t = Option<extern "C" fn(hb_font_t    *font,
                                                               table_tag: hb_tag_t,
                                                               lookup_index: libc::c_uint,
                                                               sequence: *const hb_ot_layout_glyph_sequence_t,
                                                               user_data: *mut libc::c_void) -> hb_bool_t>;

#[cfg(hb_not_implemented)]
fn Xhb_ot_layout_lookup_enumerate_sequences(face: *mut hb_face_t,
                                            table_tag: hb_tag_t,
                                            lookup_index: libc::c_uint,
                                            callback: hb_ot_layout_glyph_sequence_func_t,
                                            user_data: *mut libc::c_void);


/*
 * GSUB
 */

fn hb_ot_layout_has_substitution(face: *mut hb_face_t) -> hb_bool_t;

fn hb_ot_layout_lookup_would_substitute(face: *mut hb_face_t,
                                        lookup_index: libc::c_uint,
                                        glyphs: *const hb_codepoint_t,
                                        glyphs_length: libc::c_uint,
                                        zero_context: hb_bool_t) -> hb_bool_t;

fn hb_ot_layout_lookup_substitute_closure(face: *mut hb_face_t,
                                          lookup_index: libc::c_uint,
                                          glyphs: *mut hb_set_t,
                                          /* TODO(harfbuzz) inclusive: hb_bool_t */);

#[cfg(hb_not_implemented)]
/// Note: You better have GDEF when using this API, or marks won't do much.
fn Xhb_ot_layout_lookup_substitute(font: *mut hb_font_t,
                                   lookup_index: libc::c_uint,
                                   sequence: *const hb_ot_layout_glyph_sequence_t,
                                   out_size: libc::c_uint,
                                   glyphs_out: *const hb_codepoint_t, /* OUT */
                                   clusters_out: *mut libc::c_uint, /* OUT */
                                   out_length: *mut libc::c_uint /* OUT */) -> hb_bool_t;


/*
 * GPOS
 */

fn hb_ot_layout_has_positioning(face: *mut hb_face_t) -> hb_bool_t;

#[cfg(hb_not_implemented)]
/// Note: You better have GDEF when using this API, or marks won't do much.
fn Xhb_ot_layout_lookup_position(font: *mut hb_font_t,
                                 lookup_index: libc::c_uint,
                                 sequence: *const hb_ot_layout_glyph_sequence_t,
                                 positions: *mut hb_glyph_position_t /* IN / OUT */) -> hb_bool_t;

/* Optical 'size' feature info.  Returns true if found.
 * http://www.microsoft.com/typography/otspec/features_pt.htm#size */
fn hb_ot_layout_get_size_params(face: *mut hb_face_t,
                                design_size: *mut libc::c_uint,       /* OUT.  May be NULL */
                                subfamily_id: *mut libc::c_uint,      /* OUT.  May be NULL */
                                subfamily_name_id: *mut libc::c_uint, /* OUT.  May be NULL */
                                range_start: *mut libc::c_uint,       /* OUT.  May be NULL */
                                range_end: *mut libc::c_uint          /* OUT.  May be NULL */) -> hb_bool_t;
