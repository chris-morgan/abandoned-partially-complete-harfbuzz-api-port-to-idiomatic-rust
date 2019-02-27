use harfbuzz::common::{
    hb_var_int_t, Codepoint, Position, Mask, Tag, Language, Script, Direction,
    hb_bool_t,
    LANGUAGE_INVALID,
};
use harfbuzz::font::{Font, hb_font_t};
use harfbuzz::unicode::{UnicodeFuncs, hb_unicode_funcs_t};

use std::char;
use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str::FromStr;

use libc;

define_boxed_type! {
    struct Buffer;
    enum hb_buffer_t;
    fn hb_buffer_get_empty;
    fn hb_buffer_reference;
    fn hb_buffer_destroy;
    fn hb_buffer_set_user_data;
    fn hb_buffer_get_user_data;
    // Buffers don’t have make_immutable and is_immutable.
    //fn hb_buffer_make_immutable;
    //fn hb_buffer_is_immutable;
}

impl Buffer {
    /// Equivalent to `hb_buffer_create`.
    pub fn new() -> Buffer {
        unsafe {
            Buffer::from(hb_buffer_create())
        }
    }

    /// Equivalent to `hb_buffer_set_content_type`.
    pub fn set_content_type(&mut self, content_type: BufferContentType) {
        unsafe {
            hb_buffer_set_content_type(self.ptr, content_type)
        }
    }

    /// Equivalent to `hb_buffer_get_content_type`.
    pub fn content_type(&self) -> BufferContentType {
        unsafe {
            hb_buffer_get_content_type(self.ptr)
        }
    }

    /// Equivalent to `hb_buffer_set_unicode_funcs`.
    pub fn set_unicode_funcs(&mut self, unicode_funcs: UnicodeFuncs) {
        unsafe {
            hb_buffer_set_unicode_funcs(self.ptr, unicode_funcs.ptr)
        }
    }

    /// Equivalent to `hb_buffer_get_unicode_funcs`.
    pub fn unicode_funcs(&self) -> UnicodeFuncs {
        unsafe {
            UnicodeFuncs::from(hb_buffer_get_unicode_funcs(self.ptr))
        }
    }

    /// Equivalent to `hb_buffer_set_direction`.
    pub fn set_direction(&mut self, direction: Direction) {
        unsafe {
            hb_buffer_set_direction(self.ptr, direction)
        }
    }

    /// Equivalent to `hb_buffer_get_direction`.
    pub fn direction(&self) -> Direction {
        unsafe {
            hb_buffer_get_direction(self.ptr)
        }
    }

    /// Equivalent to `hb_buffer_set_script`.
    pub fn set_script(&mut self, script: Script) {
        unsafe {
            hb_buffer_set_script(self.ptr, script)
        }
    }

    /// Equivalent to `hb_buffer_get_script`.
    pub fn script(&self) -> Script {
        unsafe {
            hb_buffer_get_script(self.ptr)
        }
    }

    /// Equivalent to `hb_buffer_set_language`.
    pub fn set_language(&mut self, language: Language) {
        unsafe {
            hb_buffer_set_language(self.ptr, language)
        }
    }

    /// Equivalent to `hb_buffer_get_language`.
    pub fn language(&self) -> Language {
        unsafe {
            hb_buffer_get_language(self.ptr)
        }
    }

    /// Equivalent to `hb_buffer_set_segment_properties`.
    /// TODO: why are these two *mut-based?
    pub fn set_segment_properties(&mut self, props: *mut SegmentProperties) {
        unsafe {
            hb_buffer_set_segment_properties(self.ptr, props)
        }
    }

    /// Equivalent to `hb_buffer_get_segment_properties`.
    pub fn segment_properties(&self, props: *mut SegmentProperties) {
        unsafe {
            hb_buffer_get_segment_properties(self.ptr, props)
        }
    }

    /// Equivalent to `hb_buffer_guess_segment_properties`.
    pub fn guess_segment_properties(&mut self) {
        unsafe {
            hb_buffer_guess_segment_properties(self.ptr);
        }
    }

    /// Equivalent to `hb_buffer_set_flags`.
    pub fn set_flags(&mut self, flags: BufferFlags) {
        unsafe {
            hb_buffer_set_flags(self.ptr, flags)
        }
    }

    /// Equivalent to `hb_buffer_get_flags`.
    pub fn flags(&self) -> BufferFlags {
        unsafe {
            hb_buffer_get_flags(self.ptr)
        }
    }

    /// Equivalent to `hb_buffer_set_cluster_level`.
    pub fn set_cluster_level(&mut self, cluster_level: BufferClusterLevel) {
        unsafe {
            hb_buffer_set_cluster_level(self.ptr, cluster_level)
        }
    }

    /// Equivalent to `hb_buffer_get_cluster_level`.
    pub fn cluster_level(&self) -> BufferClusterLevel {
        unsafe {
            hb_buffer_get_cluster_level(self.ptr)
        }
    }

    /// Sets codepoint used to replace invalid UTF-8/16/32 entries.
    /// Default is `'\u{FFFD}'`.
    ///
    /// Equivalent to `hb_buffer_set_replacement_codepoint`.
    pub fn set_replacement_codepoint(&mut self, replacement_codepoint: char) {
        unsafe {
            hb_buffer_set_replacement_codepoint(self.ptr, replacement_codepoint as Codepoint)
        }
    }

    /// Equivalent to `hb_buffer_get_replacement_codepoint`.
    pub fn replacement_codepoint(&self) -> char {
        // XXX: this is cheating. Hopefully no one ever tries setting a non-scalar-value codepoint?
        // Yielding U+FFFD in such a case is a subtle joke. (Too subtle if it can actually happen.)
        let codepoint = unsafe {
            hb_buffer_get_replacement_codepoint(self.ptr)
        };
        char::from_u32(codepoint).unwrap_or('\u{FFFD}')
    }

    /// Resets the buffer. Afterwards it's as if it was just created,
    /// except that it may have a larger buffer allocated.
    ///
    /// Equivalent to `hb_buffer_reset`.
    pub fn reset(&mut self) {
        unsafe {
            hb_buffer_reset(self.ptr);
        }
    }

    /// Like reset, but does NOT clear unicode_funcs and replacement_codepoint. */
    ///
    /// Equivalent to `hb_buffer_clear_contents`.
    pub fn clear_contents(&mut self) {
        unsafe {
            hb_buffer_clear_contents(self.ptr);
        }
    }

    /// Returns false if allocation failed.
    ///
    /// Equivalent to `hb_buffer_pre_allocate`.
    pub fn pre_allocate(&mut self, size: u32) -> bool {
        unsafe {
            hb_buffer_pre_allocate(self.ptr, size) != 0
        }
    }

    /// Returns false if allocation has failed before.
    ///
    /// Equivalent to `hb_buffer_allocation_successful`.
    pub fn allocation_successful(&self) -> bool {
        unsafe {
            hb_buffer_allocation_successful(self.ptr) != 0
        }
    }

    /// Equivalent to `hb_buffer_reverse`.
    pub fn reverse(&mut self) {
        unsafe {
            hb_buffer_reverse(self.ptr)
        }
    }

    /// Equivalent to `hb_buffer_reverse`.
    pub fn reverse_range(&mut self, start: u32, end: u32) {
        unsafe {
            hb_buffer_reverse_range(self.ptr, start, end)
        }
    }

    /// Equivalent to `hb_buffer_reverse_clusters`.
    pub fn reverse_clusters(&mut self) {
        unsafe {
            hb_buffer_reverse_clusters(self.ptr)
        }
    }

    // Filling the buffer in.

    /// Equivalent to `hb_buffer_add`.
    pub fn add(&mut self, codepoint: char, cluster: u32) {
        unsafe {
            hb_buffer_add(self.ptr, codepoint as Codepoint, cluster);
        }
    }

    /// Equivalent to `hb_buffer_add_utf8` (`&str`), `hb_buffer_add_utf16` (`&[u16]`),
    /// `hb_buffer_add_utf32` (`&[u32]`), `hb_buffer_add_latin1` (`&[u8]`, only allowing access to
    /// the first 256 codepoints) and `hb_buffer_add_codepoints` (`&[char]`, bypassing the checks
    /// that `&[u32]` needs).
    // TODO: understand item_offset and item_length! Maybe rename too?
    pub fn extend<T: ?Sized + BufferAdd>(&mut self, text: &T, item_offset: u32, item_length: i32) {
        text.add_to_buffer(self, item_offset, item_length)
    }

    /// Clears any new items added at the end.
    ///
    /// Equivalent to `hb_buffer_set_length`.
    // XXX: is this safe?
    pub fn set_length(&mut self, length: u32) -> bool {
        unsafe {
            hb_buffer_set_length(self.ptr, length) != 0
        }
    }

    /// Equivalent to `hb_buffer_get_length`.
    pub fn len(&self) -> u32 {
        unsafe {
            hb_buffer_get_length(self.ptr)
        }
    }

    // Getting glyphs out of the buffer

    /// Equivalent to `hb_buffer_get_glyph_infos` with caveats:
    ///
    /// - The `length` argument is removed as superfluous for Rust; if you don’t want it all, just
    ///   slice the returned slice.
    /// - HarfBuzz returns a `*mut hb_glyph_info_t`, but I *think* that modifying it would be
    ///   undesirable. I’ve made it a `&[GlyphInfo]`, an immutable slice, instead. (This can be
    ///   reversed, it’s just that &self -> &mut anything is not supposed to happen.)
    pub fn glyph_infos(&self) -> &[GlyphInfo] {
        unsafe {
            let start = hb_buffer_get_glyph_infos(self.ptr, ptr::null_mut());
            slice::from_raw_parts(start, self.len() as usize)
        }
    }

    /// Equivalent to `hb_buffer_get_glyph_positions` with the same caveats as `glyph_infos`.
    pub fn glyph_positions(&self) -> &[GlyphPosition] {
        unsafe {
            let start = hb_buffer_get_glyph_positions(self.ptr, ptr::null_mut());
            slice::from_raw_parts(start, self.len() as usize)
        }
    }

    /// Reorders a glyph buffer to have canonical in-cluster glyph order / position.
    /// The resulting clusters should behave identical to pre-reordering clusters.
    /// NOTE: This has nothing to do with Unicode normalization.
    ///
    /// Equivalent to `hb_buffer_normalize_glyphs`.
    pub fn normalize_glyphs(&mut self) {
        unsafe {
            hb_buffer_normalize_glyphs(self.ptr)
        }
    }

    // Serialize

    /// Returns number of items, starting at start, that were serialized.
    ///
    /// Equivalent to `hb_buffer_serialize_glyphs`.
    pub fn serialize_glyphs(&mut self, start: u32, end: u32, buf: &mut String,
                            font: Option<Font>, format: BufferSerializeFormat,
                            flags: BufferSerializeFlags) -> u32 {
        // FIXME: I *believe* it should be legal UTF-8, but what if it’s not? Also using String
        // makes slicing rather difficult because of the absence of &mut str.
        unsafe {
            let buf_ptr = buf.as_ptr();
            let mut buf_len = 0;
            let buf_capacity = buf.capacity();
            mem::forget(mem::replace(buf, String::new()));
            let font_ptr = match font {
                Some(ref font) => font.ptr,
                None => ptr::null_mut(),
            };
            let serialized = hb_buffer_serialize_glyphs(self.ptr, start, end,
                                                        buf_ptr as *mut libc::c_char,
                                                        buf_capacity as u32, &mut buf_len,
                                                        font_ptr, format, flags);
            // len has now been set to a new value, so we can reconstruct the String now.
            // (If it were a Vec, we could use set_len(), but String doesn’t have that.)
            mem::replace(buf, String::from_raw_parts(buf_ptr as *mut u8, buf_len as usize,
                                                     buf_capacity));
            serialized
        }
    }

    /// Equivalent to `hb_buffer_serialize_glyphs`.
    pub fn deserialize_glyphs(&mut self, buf: &str, font: Option<Font>,
                              format: BufferSerializeFormat) -> bool {
        unsafe {
            let font_ptr = match font {
                Some(ref font) => font.ptr,
                None => ptr::null_mut(),
            };
            hb_buffer_deserialize_glyphs(self.ptr, buf.as_ptr() as *mut libc::c_char,
                                         buf.len() as i32, ptr::null_mut(),
                                         font_ptr, format) != 0
        }
    }
}

/// A convenience technique with no direct equivalent in HarfBuzz.
impl<'a> From<&'a str> for Buffer {
    fn from(text: &'a str) -> Buffer {
        let mut buffer = Buffer::new();
        buffer.extend(text, 0, -1);
        buffer
    }
}

pub trait BufferAdd {
    fn add_to_buffer(&self, buffer: &mut Buffer, item_offset: u32, item_length: i32);
}

macro_rules! impl_buffer_add {
    ($ty:ty, $ffi_fn:ident) => {
        impl BufferAdd for $ty {
            fn add_to_buffer(&self, buffer: &mut Buffer, item_offset: u32, item_length: i32) {
                unsafe {
                    $ffi_fn(buffer.ptr, self.as_ptr() as *const _, self.len() as i32,
                            item_offset, item_length);
                }
            }
        }
    }
}

impl_buffer_add!(str, hb_buffer_add_utf8);
impl_buffer_add!([u8], hb_buffer_add_latin1);
impl_buffer_add!([u16], hb_buffer_add_utf16);
impl_buffer_add!([u32], hb_buffer_add_utf32);
impl_buffer_add!([char], hb_buffer_add_codepoints);


/*** hb-buffer.c ***/

/// Equivalent to `hb_glyph_info_t`.
#[repr(C)]
pub struct GlyphInfo {
    // Woe is me, it needs to be `Codepoint` (`u32`) rather than `char` for now. XXX review this.
    pub codepoint: Codepoint,
    pub mask: Mask,
    pub cluster: u32,

    // Private.
    var1: hb_var_int_t,
    var2: hb_var_int_t,
}

impl GlyphInfo {
    /// I believe this *should* always return `Some`, but I’m not *certain* of it. It depends on
    /// whether HarfBuzz is willing to put something other than a Unicode scalar value in an
    /// `hb_glyph_info_t`’s `codepoint` field. XXX: revise this.
    pub fn codepoint(&self) -> Option<char> {
        char::from_u32(self.codepoint)
    }
}

/// Equivalent to `hb_glyph_position_t`.
#[repr(C)]
pub struct GlyphPosition {
    pub x_advance: Position,
    pub y_advance: Position,
    pub x_offset: Position,
    pub y_offset: Position,

    // Private.
    var: hb_var_int_t,
}

/// Equivalent to `hb_segment_properties_t`.
#[repr(C)]
pub struct SegmentProperties {
    pub direction: Direction,
    pub script: Script,
    pub language: Language,

    // Private.
    reserved1: *mut libc::c_void,
    reserved2: *mut libc::c_void,
}

impl SegmentProperties {
    /// Equivalent to `hb_segment_properties_hash`.
    pub fn hash(&self) -> u32 {
        unsafe {
            hb_segment_properties_hash(self)
        }
    }
}

/// Derived from `HB_SEGMENT_PROPERTIES_DEFAULT`.
impl Default for SegmentProperties {
    fn default() -> SegmentProperties {
        SegmentProperties {
            direction: Direction::Invalid,
            script: Script::Invalid,
            language: LANGUAGE_INVALID,
            reserved1: ptr::null_mut(),
            reserved2: ptr::null_mut(),
        }
    }
}

/// Derived from `hb_segment_properties_equal`.
impl PartialEq for SegmentProperties {
    fn eq(&self, other: &SegmentProperties) -> bool {
        unsafe {
            hb_segment_properties_equal(self, other) != 0
        }
    }
}

/// Equivalent to `hb_buffer_content_type_t`.
#[repr(C)]
pub enum BufferContentType {
    Invalid = 0,
    Unicode,
    Glyphs,
}

bitflags! {
    /// Equivalent to `hb_buffer_serialize_flags_t`.
    #[repr(C)]
    pub flags BufferSerializeFlags: u32 {
        /// Equivalent to `HB_BUFFER_SERIALIZE_FLAG_DEFAULT`.
        const BUFFER_SERIALIZE_FLAG_DEFAULT = 0x00000000,
        /// Equivalent to `HB_BUFFER_SERIALIZE_FLAG_NO_CLUSTERS`.
        const BUFFER_SERIALIZE_FLAG_NO_CLUSTERS = 0x00000001,
        /// Equivalent to `HB_BUFFER_SERIALIZE_FLAG_NO_POSITIONS`.
        const BUFFER_SERIALIZE_FLAG_NO_POSITIONS = 0x00000002,
        /// Equivalent to `HB_BUFFER_SERIALIZE_FLAG_NO_GLYPH_NAMES`.
        const BUFFER_SERIALIZE_FLAG_NO_GLYPH_NAMES = 0x00000004,
        /// Equivalent to `HB_BUFFER_SERIALIZE_FLAG_GLYPH_EXTENTS`.
        const BUFFER_SERIALIZE_FLAG_GLYPH_EXTENTS = 0x00000008,
    }
}

/// Equivalent to `hb_buffer_serialize_format_t`.
#[repr(u32)]
pub enum BufferSerializeFormat {
    Text = hb_tag!(b'T', b'E', b'X', b'T').u32,
    Json = hb_tag!(b'J', b'S', b'O', b'N').u32,
    Invalid = 0x00000000,  // hb_tag!(b"\0\0\0\0"),
}

/// Equivalent to `hb_buffer_serialize_format_from_string`.
impl FromStr for BufferSerializeFormat {
    /// … because errors come through as `BufferSerializeFormat::Invalid` instead.
    type Err = !;

    fn from_str(str: &str) -> Result<BufferSerializeFormat, !> {
        unsafe {
            Ok(hb_buffer_serialize_format_from_string(str.as_ptr() as *const libc::c_char,
                                                      str.len() as i32))
        }
    }
}

/// Replacement for `hb_buffer_serialize_format_to_string`.
/// That returns `"text"` for `Text`, `"json"` for `Json` and `NULL` for `Invalid`.
/// We instead emit `"text"`, `"json"` or `""`. If anyone really wants it, we can easily add a
/// method that returns `Option<&'static str>`.
impl fmt::Display for BufferSerializeFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            BufferSerializeFormat::Text => "text",
            BufferSerializeFormat::Json => "json",
            BufferSerializeFormat::Invalid => "",
        })
    }
}

impl BufferSerializeFormat {
    /// Equivalent to `hb_buffer_serialize_list_formats`.
    pub fn list_formats() -> &'static [&'static str] {
        // Copied from hb-buffer-serialize.cc instead of calling hb_buffer_serialize_list_formats.
        // Yes, this is a slightly risky thing to do, but it should be OK.
        const FORMATS: &'static [&'static str] = &["text", "json"];

        FORMATS
    }
}

extern "C" {
    fn hb_buffer_create() -> *mut hb_buffer_t;
    fn hb_buffer_set_content_type(buffer: *mut hb_buffer_t,
                                  content_type: BufferContentType);
    fn hb_buffer_get_content_type(buffer: *mut hb_buffer_t) -> BufferContentType;
    fn hb_buffer_set_unicode_funcs(buffer: *mut hb_buffer_t,
                                   unicode_funcs: *mut hb_unicode_funcs_t);
    fn hb_buffer_get_unicode_funcs(buffer: *mut hb_buffer_t) -> *mut hb_unicode_funcs_t;
    fn hb_buffer_set_direction(buffer: *mut hb_buffer_t, direction: Direction);
    fn hb_buffer_get_direction(buffer: *mut hb_buffer_t) -> Direction;
    fn hb_buffer_set_script(buffer: *mut hb_buffer_t, script: Script);
    fn hb_buffer_get_script(buffer: *mut hb_buffer_t) -> Script;
    fn hb_buffer_set_language(buffer: *mut hb_buffer_t, language: Language);
    fn hb_buffer_get_language(buffer: *mut hb_buffer_t) -> Language;
    
    fn hb_buffer_set_segment_properties(buffer: *mut hb_buffer_t,
                                        props: *const SegmentProperties);
    
    fn hb_buffer_get_segment_properties(buffer: *mut hb_buffer_t,
                                        props: *mut SegmentProperties);
    
    fn hb_buffer_guess_segment_properties(buffer: *mut hb_buffer_t);

    fn hb_buffer_set_flags(buffer: *mut hb_buffer_t, flags: BufferFlags);
    fn hb_buffer_get_flags(buffer: *mut hb_buffer_t) -> BufferFlags;

    fn hb_buffer_set_cluster_level(buffer: *mut hb_buffer_t, cluster_level: BufferClusterLevel);
    fn hb_buffer_get_cluster_level(buffer: *mut hb_buffer_t) -> BufferClusterLevel;

    fn hb_buffer_set_replacement_codepoint(buffer: *mut hb_buffer_t, replacement: Codepoint);
    fn hb_buffer_get_replacement_codepoint(buffer: *mut hb_buffer_t) -> Codepoint;

    fn hb_buffer_reset(buffer: *mut hb_buffer_t);
    fn hb_buffer_clear_contents(buffer: *mut hb_buffer_t);

    fn hb_segment_properties_equal(a: *const SegmentProperties,
                                   b: *const SegmentProperties) -> hb_bool_t;
    fn hb_segment_properties_hash(p: *const SegmentProperties) -> u32;
    fn hb_buffer_pre_allocate(buffer: *mut hb_buffer_t, size: u32) -> hb_bool_t;
    fn hb_buffer_allocation_successful(buffer: *mut hb_buffer_t) -> hb_bool_t;
    fn hb_buffer_reverse(buffer: *mut hb_buffer_t);
    fn hb_buffer_reverse_range (buffer: *mut hb_buffer_t, start: u32, end: u32);
    fn hb_buffer_reverse_clusters(buffer: *mut hb_buffer_t);

    fn hb_buffer_add(buffer: *mut hb_buffer_t, codepoint: Codepoint, cluster: u32);
    fn hb_buffer_add_utf8(buffer: *mut hb_buffer_t, text: *const libc::c_char,
                          text_length: i32, item_offset: u32, item_length: i32);
    fn hb_buffer_add_utf16(buffer: *mut hb_buffer_t, text: *const u16,
                           text_length: i32, item_offset: u32, item_length: i32);
    fn hb_buffer_add_utf32(buffer: *mut hb_buffer_t, text: *const u32,
                           text_length: i32, item_offset: u32, item_length: i32);
    fn hb_buffer_add_latin1(buffer: *mut hb_buffer_t, text: *const u8,
                            text_length: i32, item_offset: u32, item_length: i32);
    fn hb_buffer_add_codepoints(buffer: *mut hb_buffer_t, text: *const Codepoint,
                                text_length: i32, item_offset: u32, item_length: i32);
    fn hb_buffer_set_length(buffer: *mut hb_buffer_t, length: u32) -> hb_bool_t;
    fn hb_buffer_get_length(buffer: *mut hb_buffer_t) -> u32;

    fn hb_buffer_get_glyph_infos(buffer: *mut hb_buffer_t,
                                 length: *mut u32) -> *mut GlyphInfo;
    fn hb_buffer_get_glyph_positions(buffer: *mut hb_buffer_t,
                                     length: *mut u32) -> *mut GlyphPosition;
    fn hb_buffer_normalize_glyphs(buffer: *mut hb_buffer_t);
    fn hb_buffer_serialize_format_from_string(str: *const libc::c_char,
                                              len: i32) -> BufferSerializeFormat;


    //fn hb_buffer_serialize_format_to_string(format: BufferSerializeFormat)
    //    -> *const libc::c_char;
    //fn hb_buffer_serialize_list_formats() -> *const *const libc::c_char;

    fn hb_buffer_serialize_glyphs(buffer: *mut hb_buffer_t,
                                  start: u32,
                                  end: u32,
                                  buf: *mut libc::c_char,
                                  buf_size: u32,
                                  buf_consumed: *mut u32,  // May be NULL
                                  font: *mut hb_font_t,  // May be NULL
                                  format: BufferSerializeFormat,
                                  flags: BufferSerializeFlags) -> u32;

    fn hb_buffer_deserialize_glyphs(buffer: *mut hb_buffer_t,
                                    buf: *const libc::c_char,
                                    buf_len: i32, // -1 means nul-terminated
                                    end_ptr: *mut *const libc::c_char, // May be NULL
                                    font: *mut hb_font_t, // May be NULL
                                    format: BufferSerializeFormat) -> hb_bool_t;
}

bitflags! {
    /// Equivalent to `hb_buffer_flags_t`.
    #[repr(C)]
    pub flags BufferFlags: u32 {
        /// Equivalent to `HB_BUFFER_FLAGS_DEFAULT`.
        const BUFFER_FLAGS_DEFAULT = 0x00000000,
        /// Beginning-of-text. Equivalent to `HB_BUFFER_FLAGS_BOT`.
        const BUFFER_FLAGS_BOT = 0x00000001,
        /// End-of-text. Equivalent to `HB_BUFFER_FLAGS_EOT`.
        const BUFFER_FLAGS_EOT = 0x00000002,
        /// Equivalent to `HB_BUFFER_FLAGS_PRESERVE_DEFAULT_IGNORABLES`.
        const BUFFER_FLAGS_PRESERVE_DEFAULT_IGNORABLES = 0x00000004,
    }
}

/// Equivalent to `hb_buffer_cluster_level_t`.
#[repr(C)]
pub enum BufferClusterLevel {
    /// Equivalent to `HB_BUFFER_CLUSTER_LEVEL_MONOTONE_GRAPHEMES`.
    MonotoneGraphemes = 0,
    /// Equivalent to `HB_BUFFER_CLUSTER_LEVEL_MONOTONE_CHARACTERS`.
    MonotoneCharacters = 1,
    /// Equivalent to `HB_BUFFER_CLUSTER_LEVEL_CHARACTERS`.
    Characters = 2,
}

/// Derived from `HB_BUFFER_CLUSTER_LEVEL_DEFAULT`.
impl Default for BufferClusterLevel {
    fn default() -> BufferClusterLevel {
        BufferClusterLevel::MonotoneGraphemes
    }
}

/// Equivalent to `HB_BUFFER_REPLACEMENT_CODEPOINT_DEFAULT`.
pub const BUFFER_REPLACEMENT_CODEPOINT_DEFAULT: char = '\u{FFFD}';
