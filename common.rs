//! Derived from `hb-common.h`.

use std::fmt;
use std::mem;
use std::str::{self, FromStr};
use std::ptr;

use libc;

#[allow(non_camel_case_types)]
pub(super) type hb_bool_t = i32;

// `hb_codepoint_t` is `u32`, but we have `char` which is better, but not “FFI safe” because of the
// extra restriction of its being a Unicode scalar value, i.e. not greater than `0x10ffff`.
// Handing out a `char` to something expecting a `u32` would be fine, but if it was going to write
// to it, e.g. via a `*mut char`, well, that could break things. Therefore, let’s leave it at `u32`
// here. Ideally we’ll be able to expose it as `char` in public. TODO think about it more.
// TODO pub or rename hb_codepoint_t.
pub(super) type Codepoint = u32;

/// `hb_position_t`. TODO: is this 16.16 fixed? TODO: pub
pub(super) type Position = i32;

/// `hb_mask_t`. TODO: is this 16.16 fixed? TODO: pub
pub(super) type Mask = u32;

/// `hb_var_int_t` (a.k.a. `union _hb_var_int_t`) is not used in any public API,
/// so there’s no need to implement the full unionness of it.
pub(super) type hb_var_int_t = u32;

/// Equivalent to `hb_tag_t`.
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Tag { pub(super) u32: u32 }

#[macro_export]
macro_rules! hb_tag {
    // Preferred form: `hb_tag!(b"liga")`.
    ($chars:expr) => {{
        let [c0, c1, c2, c3]: [u8; 4] = *$chars;
        hb_tag!(c0, c1, c2, c3)
    }};

    // C-style form (directly equivalent to `HB_TAG`): `hb_tag!(b'l', b'i', b'g', b'a')`.
    ($c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
        Tag { u32: ($c1 as u32) << 24 | ($c2 as u32) << 16 | ($c3 as u32) << 8 | ($c4 as u32) }
    };
}

/// A replacement for the `HB_TAG` macro. Note that we may still need that if (a) constants are
/// required, and (b) constant function evaluation hasn’t landed in Rust yet. (Theoretically: how
/// cool is `const LIGA: Tag = b"liga".into()`?) See also the `tag` macro above.
impl From<[u8; 4]> for Tag {
    fn from([c1, c2, c3, c4]: [u8; 4]) -> Tag {
        Tag { u32: (c1 as u32) << 24 | (c2 as u32) << 16 | (c3 as u32) << 8 | (c4 as u32) }
    }
}

/// A replacement for the `HB_UNTAG` macro. Note that we may still need that if (a) constants are
/// required, and (b) constant function evaluation hasn’t landed in Rust yet.
impl From<Tag> for [u8; 4] {
    fn from(tag: Tag) -> [u8; 4] {
        [(tag.u32 >> 24) as u8, (tag.u32 >> 16) as u8, (tag.u32 >> 8) as u8, tag.u32 as u8]
    }
}

/// Equivalent to `HB_TAG_NONE`.
pub(super) const TAG_NONE: Tag = Tag { u32: 0x00000000 };

/// Equivalent to `HB_TAG_MAX`.
pub(super) const TAG_MAX: Tag = Tag { u32: 0xffffffff };

/// Equivalent to `HB_TAG_MAX_SIGNED`.
pub(super) const TAG_MAX_SIGNED: Tag = Tag { u32: 0x7fffffff };

/// Replacement for `hb_tag_from_string`.
///
/// This is not strictly equivalent. `hb_tag_from_string` returns `HB_TAG_NONE` if the string
/// *starts with* a null byte, while we won’t unless all four bytes are null bytes.
// (Frankly that bit of code feels weird and icky.)
/// This is also not as general as `hb_tag_from_string`, because `&str` can only be valid UTF-8.
impl FromStr for Tag {
    type Err = !;

    fn from_str(str: &str) -> Result<Tag, !> {
        let bytes = str.as_bytes();
        Ok(Tag::from([bytes.get(0).map(|&b| b).unwrap_or(b' '),
                      bytes.get(1).map(|&b| b).unwrap_or(b' '),
                      bytes.get(2).map(|&b| b).unwrap_or(b' '),
                      bytes.get(3).map(|&b| b).unwrap_or(b' ')]))
    }
}

/// Replacement for `hb_tag_to_string`.
///
/// This is not completely equivalent: we uphold UTF-8, and in case of a tag that would be illegal
/// UTF-8 emit `"    "` (four spaces) instead.
impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = <[u8; 4]>::from(*self);
        let string = str::from_utf8(&bytes).unwrap_or("    ");
        fmt::Display::fmt(string, f)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes = <[u8; 4]>::from(*self);
        let string = str::from_utf8(&bytes).unwrap_or("    ");
        fmt::Debug::fmt(string, f)
    }
}

/// Equivalent to `hb_direction_t` in HarfBuzz.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum Direction {
  Invalid = 0,
  Ltr = 4,
  Rtl,
  Ttb,
  Btt,
}

/// Replacement for `hb_direction_from_string`.
///
/// This one is equivalent, barring the UTF-8 requirement on the source string.
impl FromStr for Direction {
    type Err = !;

    fn from_str(str: &str) -> Result<Direction, !> {
        // As in hb_direction_from_string, match only the first letter.
        Ok(match str.as_bytes().get(0) {
            Some(&b'L') | Some(&b'l') => Direction::Ltr,
            Some(&b'R') | Some(&b'r') => Direction::Rtl,
            Some(&b'T') | Some(&b't') => Direction::Ttb,
            Some(&b'B') | Some(&b'b') => Direction::Btt,
            _ => Direction::Invalid,
        })
    }
}

/// Replacement for `hb_direction_to_string`.
///
/// This one is completely equivalent, except insofar as it’s not emitting a string.
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match *self {
            Direction::Invalid => "invalid",
            Direction::Ltr => "ltr",
            Direction::Rtl => "rtl",
            Direction::Ttb => "ttb",
            Direction::Btt => "btt",
        };
        fmt::Display::fmt(string, f)
    }
}

impl Direction {
    /// Equivalent to `HB_DIRECTION_IS_VALID(dir)` in HarfBuzz.
    #[inline]
    pub fn is_valid(self) -> bool {
        match self {
            Direction::Invalid => false,
            _ => true,
        }
    }

    /// Equivalent to `HB_DIRECTION_IS_HORIZONTAL(dir)` in HarfBuzz.
    #[inline]
    pub fn is_horizontal(self) -> bool {
        match self {
            Direction::Ltr | Direction::Rtl => true,
            _ => false,
        }
    }

    /// Equivalent to `HB_DIRECTION_IS_VERTICAL(dir)` in HarfBuzz.
    #[inline]
    pub fn is_vertical(self) -> bool {
        match self {
            Direction::Ttb | Direction::Btt => true,
            _ => false,
        }
    }

    /// Equivalent to `HB_DIRECTION_IS_FORWARD(dir)` in HarfBuzz.
    #[inline]
    pub fn is_forward(self) -> bool {
        match self {
            Direction::Ltr | Direction::Ttb => true,
            _ => false,
        }
    }

    /// Equivalent to `HB_DIRECTION_IS_BACKWARD(dir)` in HarfBuzz.
    #[inline]
    pub fn is_backward(self) -> bool {
        match self {
            Direction::Rtl | Direction::Btt => true,
            _ => false,
        }
    }

    /// Equivalent to `HB_DIRECTION_REVERSE(dir)` in HarfBuzz.
    ///
    /// Trivial deviation from `HB_DIRECTION_REVERSE`: it turns `Invalid` (0x0) into an invalid
    /// variant (0x1), which is all very well for their purposes, but not nice from Rust’s
    /// perspective. So at the cost of a teensy bit of execution efficiency, we keep `Invalid` as
    /// `Invalid` (viz. reversing 0x0 yields 0x0 rather than 0x1 as in HarfBuzz).
    #[inline]
    pub fn reverse(self) -> Self {
        match self {
            Direction::Invalid => Direction::Invalid,
            Direction::Ltr => Direction::Ltr,
            Direction::Rtl => Direction::Rtl,
            Direction::Ttb => Direction::Ttb,
            Direction::Btt => Direction::Btt,
        }
    }
}


/// Equivalent to `hb_language_impl_t`.
#[derive(Copy, Clone)]
enum LanguageImpl { }

/// Equivalent to `hb_language_t`.
//#[derive(Copy, Clone)]
#[repr(C)]
pub struct Language(*const LanguageImpl);

// TODO: impl FromStr and Display via extern fns hb_language_from_string and hb_language_to_string.

/// Equivalent to `HB_LANGUAGE_INVALID`, though that is a macro and this is a constant.
pub(super) const LANGUAGE_INVALID: Language = Language(0 as *const LanguageImpl);

impl Language {
    /// TODO: should this be a Default impl?
    pub fn get_default() -> Language {
        unsafe {
            hb_language_get_default()
        }
    }
}

extern "C" {
    fn hb_script_from_iso15924_tag(tag: Tag) -> Script;
    fn hb_script_get_horizontal_direction(script: Script) -> Direction;
    fn hb_language_from_string(str: *const libc::c_char, len: libc::c_int) -> Language;
    fn hb_language_to_string(language: Language) -> *const libc::c_char;
    fn hb_language_get_default() -> Language;
}



/// http://unicode.org/iso15924/
/// http://goo.gl/x9ilM
/// Unicode Character Database property: Script (sc)
///
/// Equivalent to `hb_script_t` in HarfBuzz.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum Script {
    Common = 1517910393,
    Inherited = 1516858984,
    Unknown = 1517976186,
    Arabic = 1098015074,
    Armenian = 1098018158,
    Bengali = 1113943655,
    Cyrillic = 1132032620,
    Devanagari = 1147500129,
    Georgian = 1197830002,
    Greek = 1198679403,
    Gujarati = 1198877298,
    Gurmukhi = 1198879349,
    Hangul = 1214344807,
    Han = 1214344809,
    Hebrew = 1214603890,
    Hiragana = 1214870113,
    Kannada = 1265525857,
    Katakana = 1264676449,
    Lao = 1281453935,
    Latin = 1281455214,
    Malayalam = 1298954605,
    Oriya = 1332902241,
    Tamil = 1415671148,
    Telugu = 1415933045,
    Thai = 1416126825,
    Tibetan = 1416192628,
    Bopomofo = 1114599535,
    Braille = 1114792297,
    CanadianSyllabics = 1130458739,
    Cherokee = 1130915186,
    Ethiopic = 1165256809,
    Khmer = 1265134962,
    Mongolian = 1299148391,
    Myanmar = 1299803506,
    Ogham = 1332175213,
    Runic = 1383427698,
    Sinhala = 1399418472,
    Syriac = 1400468067,
    Thaana = 1416126817,
    Yi = 1500080489,
    Deseret = 1148416628,
    Gothic = 1198486632,
    OldItalic = 1232363884,
    Buhid = 1114990692,
    Hanunoo = 1214344815,
    Tagalog = 1416064103,
    Tagbanwa = 1415669602,
    Cypriot = 1131442804,
    Limbu = 1281977698,
    LinearB = 1281977954,
    Osmanya = 1332964705,
    Shavian = 1399349623,
    TaiLe = 1415670885,
    Ugaritic = 1432838514,
    Buginese = 1114990441,
    Coptic = 1131376756,
    Glagolitic = 1198285159,
    Kharoshthi = 1265131890,
    NewTaiLue = 1415670901,
    OldPersian = 1483761007,
    SylotiNagri = 1400466543,
    Tifinagh = 1415999079,
    Balinese = 1113681001,
    Cuneiform = 1483961720,
    Nko = 1315663727,
    PhagsPa = 1349017959,
    Phoenician = 1349021304,
    Carian = 1130459753,
    Cham = 1130914157,
    KayahLi = 1264675945,
    Lepcha = 1281716323,
    Lycian = 1283023721,
    Lydian = 1283023977,
    OlChiki = 1332503403,
    Rejang = 1382706791,
    Saurashtra = 1398895986,
    Sundanese = 1400204900,
    Vai = 1449224553,
    Avestan = 1098281844,
    Bamum = 1113681269,
    EgyptianHieroglyphs = 1164409200,
    ImperialAramaic = 1098018153,
    InscriptionalPahlavi = 1349020777,
    InscriptionalParthian = 1349678185,
    Javanese = 1247901281,
    Kaithi = 1265920105,
    Lisu = 1281979253,
    MeeteiMayek = 1299473769,
    OldSouthArabian = 1398895202,
    OldTurkic = 1332898664,
    Samaritan = 1398893938,
    TaiTham = 1281453665,
    TaiViet = 1415673460,
    Batak = 1113683051,
    Brahmi = 1114792296,
    Mandaic = 1298230884,
    Chakma = 1130457965,
    MeroiticCursive = 1298494051,
    MeroiticHieroglyphs = 1298494063,
    Miao = 1349284452,
    Sharada = 1399353956,
    SoraSompeng = 1399812705,
    Takri = 1415670642,
    BassaVah = 1113682803,
    CaucasianAlbanian = 1097295970,
    Duployan = 1148547180,
    Elbasan = 1164730977,
    Grantha = 1198678382,
    Khojki = 1265135466,
    Khudawadi = 1399418468,
    LinearA = 1281977953,
    Mahajani = 1298229354,
    Manichaean = 1298230889,
    MendeKikakui = 1298493028,
    Modi = 1299145833,
    Mro = 1299345263,
    Nabataean = 1315070324,
    OldNorthArabian = 1315009122,
    OldPermic = 1348825709,
    PahawhHmong = 1215131239,
    Palmyrene = 1348562029,
    PauCinHau = 1348564323,
    PsalterPahlavi = 1349020784,
    Siddham = 1399415908,
    Tirhuta = 1416196712,
    WarangCiti = 1466004065,
    Ahom = 1097363309,
    AnatolianHieroglyphs = 1215067511,
    Hatran = 1214346354,
    Multani = 1299541108,
    OldHungarian = 1215655527,
    Signwriting = 1399287415,
    Invalid = 0,
}

impl Script {
    /// Equivalent to `hb_script_from_iso15924_tag`.
    pub fn from_iso15924_tag(tag: Tag) -> Script {
        unsafe {
            hb_script_from_iso15924_tag(tag)
        }
    }

    pub fn to_iso15924_tag(self) -> Tag {
        Tag { u32: self as u32 }
    }

    pub fn horizontal_direction(self) -> Direction {
        unsafe {
            hb_script_get_horizontal_direction(self)
        }
    }
}

impl FromStr for Script {
    type Err = !;

    fn from_str(str: &str) -> Result<Script, !> {
        str.parse().map(Script::from_iso15924_tag)
    }
}


// User data

// TODO: replace pub with pub(super) when I clean up the user data API (scrap the pointless trait).
/// Equivalent to `hb_user_data_key_t`.
#[repr(C)]
pub struct hb_user_data_key_t {
    unused: libc::c_char,
}

/// `hb_destroy_func_t`. (Not renamed to DestroyFunc, can’t prettify it as it is.)
/// Note that calling such a function is unsafe, even if the definition doesn’t say so.
#[allow(non_camel_case_types)]
pub(super) type hb_destroy_func_t = Option<extern "C" fn(user_data: *mut libc::c_void)>;

/// Convert an arbitrary Rust value into suitable user data with a destroy function which is just
/// the Rust destructor (`Drop`).
///
/// There is no equivalent in HarfBuzz.
pub(super) unsafe fn into_user_data<T>(user_data: T) -> (*mut libc::c_void, hb_destroy_func_t) {
    let ptr = Box::into_raw(Box::new(user_data)) as *mut libc::c_void;

    unsafe extern "C" fn destroy<T>(user_data: *mut T) {
        let _ = Box::from_raw(user_data);
    }

    // Note that we can’t do `as extern "C" fn(*mut libc::c_void)`: “non-scalar cast”.
    // So let’s just transmute instead. What could possibly go wrong?
    (ptr, Some(mem::transmute(destroy::<T> as unsafe extern "C" fn(*mut T))))
}

use std::any::TypeId;

/// This doesn’t come directly from a HarfBuzz header file; it’s a higher-level, safe API
/// replacement for `hb_*_set_user_data` and `hb_*_get_user_data`.
///
/// Note how it is deliberately limited; in HarfBuzz, the `hb_user_data_key_t` is a pointer-sized
/// opaque key (`const *char`), and the data type is always `void *`. This is obviously wildly
/// unsafe and thus unsuitable for Rust. One interesting pattern it does make possible is defining
/// an array of keys; but given that the user data is basically an array-backed map with O(n)
/// lookup it’s better to use one key and store an array of values inside it—presuming that the
/// values are of the same type.
///
/// Anyway, I choose to turn this into what is basically a `TypeMap`. There is one important
/// caveat which, strictly speaking, makes this unsafe on 32-bit platforms: Rust’s TypeId is a u64,
/// but the key type is a pointer, which may be less than 64 bits. TypeIds are known to be evenly
/// distributed (they’re hashed values), so we just take the last 32 bits on a 32-bit platform.
/// This makes collisions of type IDs possible. You’re cringing, aren’t you? I don’t blame you.
pub struct UserData<P: HasUserData> {
    parent: *mut P::Ptr,
}

impl<P: HasUserData> From<*mut P::Ptr> for UserData<P> {
    fn from(ptr: *mut P::Ptr) -> UserData<P> {
        UserData {
            parent: ptr,
        }
    }
}

pub trait UserDataKey: 'static {
    type Data;
}

impl<P: HasUserData> UserData<P> {
    pub fn set<K: UserDataKey>(&mut self, _key: K, data: K::Data, replace: bool) -> bool {
        unsafe {
            let (data, destroy) = into_user_data(data);
            P::set(self.parent, key::<K>(), data, destroy, replace as hb_bool_t) != 0
        }
    }

    pub fn remove<K: UserDataKey>(&mut self, _key: K) -> bool {
        unsafe {
            P::set(self.parent, key::<K>(), ptr::null_mut(), None, 0) != 0
        }
    }

    pub fn get<K: UserDataKey>(&self, _key: K) -> Option<&K::Data> {
        unsafe {
            let data = P::get(self.parent, key::<K>());
            (data as *const K::Data).as_ref()
        }
    }

    pub fn get_mut<K: UserDataKey>(&mut self, _key: K) -> Option<&mut K::Data> {
        unsafe {
            let data = P::get(self.parent, key::<K>());
            (data as *mut K::Data).as_mut()
        }
    }
}

pub trait HasUserData {
    type Ptr;

    unsafe fn get(ptr: *mut Self::Ptr, key: *mut hb_user_data_key_t) -> *mut libc::c_void;

    unsafe fn set(ptr: *mut Self::Ptr, key: *mut hb_user_data_key_t, data: *mut libc::c_void,
                  destroy: hb_destroy_func_t, replace: hb_bool_t) -> hb_bool_t;
}

fn key<T: UserDataKey>() -> *mut hb_user_data_key_t {
    unsafe {
        let key = mem::transmute::<TypeId, u64>(TypeId::of::<T>()) as usize;
        key as *mut hb_user_data_key_t
    }
}

macro_rules! define_boxed_type {
    {
        struct $Foo:ident;
        enum $hb_foo_t:ident;
        // Alas, concat_idents! is still useless for this sort of thing. (`enum concat_idents!(…)`
        // doesn’t work, `fn concat_idents!(…)` ditto.)
        fn $hb_foo_get_empty:ident;
        fn $hb_foo_reference:ident;
        fn $hb_foo_destroy:ident;
        fn $hb_foo_set_user_data:ident;
        fn $hb_foo_get_user_data:ident;
        $(
        fn $hb_foo_make_immutable:ident;
        fn $hb_foo_is_immutable:ident;
        )*
    } => {
        #[doc(hidden)]  // Only public so user_data works.
        #[allow(non_camel_case_types)]
        pub enum $hb_foo_t { }

        pub struct $Foo {
            pub(super) ptr: *mut $hb_foo_t,
            /// Replacement for `hb_foo_get_user_data` and `hb_foo_set_user_data`.
            // This design allows the user to replace the user_data and make it misbehave. Too bad.
            pub user_data: super::common::UserData<$Foo>,
        }

        impl super::common::HasUserData for $Foo {
            type Ptr = $hb_foo_t;

            unsafe fn get(ptr: *mut Self::Ptr, key: *mut super::common::hb_user_data_key_t) -> *mut ::libc::c_void {
                $hb_foo_get_user_data(ptr, key)
            }

            unsafe fn set(ptr: *mut Self::Ptr, key: *mut super::common::hb_user_data_key_t, data: *mut ::libc::c_void,
                          destroy: super::common::hb_destroy_func_t, replace: super::common::hb_bool_t) -> super::common::hb_bool_t {
                $hb_foo_set_user_data(ptr, key, data, destroy, replace)
            }
        }

        impl ::std::fmt::Pointer for $Foo {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Pointer::fmt(&self.ptr, f)
            }
        }

        impl From<*mut $hb_foo_t> for $Foo {
            fn from(raw: *mut $hb_foo_t) -> $Foo {
                use super::common::UserData;
                $Foo {
                    ptr: raw,
                    user_data: UserData::from(raw),
                }
            }
        }

        impl $Foo {
            pub fn get_empty() -> $Foo {
                unsafe {
                    $Foo::from($hb_foo_get_empty())
                }
            }

            $(
            pub fn make_immutable(&mut self) {
                unsafe {
                    $hb_foo_make_immutable(self.ptr)
                }
            }

            pub fn is_immutable(&self) -> bool {
                unsafe {
                    $hb_foo_is_immutable(self.ptr) != 0
                }
            }
            )*

        }

        impl Clone for $Foo {
            fn clone(&self) -> Self {
                $Foo::from(unsafe { $hb_foo_reference(self.ptr) })
            }
        }

        impl Drop for $Foo {
            fn drop(&mut self) {
                unsafe {
                    $hb_foo_destroy(self.ptr);
                }
            }
        }

        extern "C" {
            fn $hb_foo_get_empty() -> *mut $hb_foo_t;
            fn $hb_foo_reference(self_: *mut $hb_foo_t) -> *mut $hb_foo_t;
            fn $hb_foo_destroy(self_: *mut $hb_foo_t);
            fn $hb_foo_set_user_data(self_: *mut $hb_foo_t,
                                     key: *mut super::common::hb_user_data_key_t,
                                     data: *mut ::libc::c_void,
                                     destroy: super::common::hb_destroy_func_t,
                                     replace: super::common::hb_bool_t) -> super::common::hb_bool_t;
            fn $hb_foo_get_user_data(self_: *mut $hb_foo_t,
                                     key: *mut super::common::hb_user_data_key_t) -> *mut ::libc::c_void;
            $(
            fn $hb_foo_make_immutable(self_: *mut $hb_foo_t);
            fn $hb_foo_is_immutable(self_: *mut $hb_foo_t) -> super::common::hb_bool_t;
            )*
        }
    }
}
