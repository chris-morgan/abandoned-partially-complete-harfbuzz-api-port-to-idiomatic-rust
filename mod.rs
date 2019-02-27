#![allow(non_camel_case_types)]

#[macro_use]
mod common;
mod blob;
mod buffer;
mod face;
mod font;
mod shape;
mod unicode;

pub use self::common::{Language, Tag, Direction, Script};
pub use self::blob::{Blob, MemoryMode};
pub use self::buffer::{Buffer, GlyphInfo, GlyphPosition, SegmentProperties, BufferContentType,
                       BufferFlags, BufferSerializeFlags,
                       BufferSerializeFormat, BufferClusterLevel,

                       BUFFER_REPLACEMENT_CODEPOINT_DEFAULT,

                       BUFFER_FLAGS_BOT,
                       BUFFER_FLAGS_DEFAULT,
                       BUFFER_FLAGS_EOT,
                       BUFFER_FLAGS_PRESERVE_DEFAULT_IGNORABLES,

                       BUFFER_SERIALIZE_FLAG_DEFAULT,
                       BUFFER_SERIALIZE_FLAG_GLYPH_EXTENTS,
                       BUFFER_SERIALIZE_FLAG_NO_CLUSTERS,
                       BUFFER_SERIALIZE_FLAG_NO_GLYPH_NAMES,
                       BUFFER_SERIALIZE_FLAG_NO_POSITIONS};
pub use self::face::Face;
pub use self::font::{Font, FontFuncs, GlyphExtents};
pub use self::shape::{Feature, shape, shape_full, ShaperList, list_shapers, Shapers};
pub use self::unicode::{UnicodeGeneralCategory, UnicodeCombiningClass, UnicodeFuncs};
