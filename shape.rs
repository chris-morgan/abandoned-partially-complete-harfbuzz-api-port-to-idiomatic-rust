//! Derived from hb-shape.h.

use std::ffi::{CStr, CString};
use std::iter::FromIterator;
use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;

use libc;

use harfbuzz::common::{Tag, hb_bool_t};
use harfbuzz::buffer::{Buffer, hb_buffer_t};
use harfbuzz::font::{Font, hb_font_t};

/// Equivalent to `hb_feature_t`.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Feature {
    pub tag: Tag,
    pub value: u32,
    pub start: u32,
    pub end: u32,
}

impl FromStr for Feature {
    type Err = ();

    fn from_str(str: &str) -> Result<Feature, ()> {
        unsafe {
            let mut feature = Feature::default();
            if hb_feature_from_string(str.as_ptr() as *const libc::c_char, str.len() as i32,
                                      &mut feature) != 0 {
                Ok(feature)
            } else {
                Err(())
            }
        }
    }
}

// TODO: implement Display with hb_feature_to_string. Step one is knowing how much buffer we need!

pub fn shape(font: &Font, buffer: &mut Buffer, features: &[Feature]) {
    unsafe {
        hb_shape(font.ptr, buffer.ptr, features.as_ptr(), features.len() as u32);
    }
}

pub fn shape_full(font: &Font, buffer: &mut Buffer, features: &[Feature],
                  shaper_list: &ShaperList) -> bool {
    unsafe {
        hb_shape_full(font.ptr, buffer.ptr, features.as_ptr(), features.len() as u32,
                      shaper_list.as_ptr()) != 0
    }
}

/// On the C side, this needs to be `*const *const c_char`, a null-terminated list of
/// null-terminated C strings. In Rust we’d prefer something like `&[&str]`, but that’s rather
/// incompatible. Converting between `&[&str]` and `*const *const c_char` on each `hb_shape_full`
/// call is also prohibitively expensive. `ShaperList` is the answer to this: it can easily be
/// constructed in Rust, but
pub struct ShaperList {
    // Invariant: the last (and *only* the last) item in the Vec is NULL.
    // Invariant: the *const c_char members are actually CStrings in disguise
    // (for efficiency of use in C).
    shapers: Vec<*const c_char>,
}

/// A convenience conversion: `["foo", "bar"].into()`.
impl<'a, 'b> From<&'a [&'b str]> for ShaperList {
    fn from(slice: &'a [&'b str]) -> ShaperList {
        slice.iter().map(|&str| str).collect()
    }
}

/// Note that if any of the strings contain null bytes, the result will not be what you expect
/// (though it’ll still be memory safe): that entry will be treated as terminating the list.
impl<'a> FromIterator<&'a str> for ShaperList {
    fn from_iter<T>(iter: T) -> ShaperList
    where T: IntoIterator<Item = &'a str> {
        let mut iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();
        let mut vec = Vec::with_capacity(lower.saturating_add(1));
        vec.extend(iterator.map(|str| CString::new(str).unwrap_or(CString::default()).into_raw()
                                      as *const c_char));
        vec.push(ptr::null());
        ShaperList {
            shapers: vec,
        }
    }
}

impl ShaperList {
    fn as_ptr(&self) -> *const *const c_char {
        self.shapers.as_ptr()
    }
}

impl Drop for ShaperList {
    fn drop(&mut self) {
        unsafe {
            for &shaper in self.shapers.iter() {
                if !shaper.is_null() {
                    // These *const c_chars are actually CStrings, so let’s avoid leaking them.
                    let _ = CString::from_raw(shaper as *mut c_char);
                }
            }
        }
    }
}

pub struct Shapers {
    shapers: *mut *const c_char,
}

impl Iterator for Shapers {
    type Item = &'static CStr;

    fn next(&mut self) -> Option<&'static CStr> {
        unsafe {
            if self.shapers.is_null() {
                // Should be unreachable, but I’m not *positive* about that.
                None
            } else {
                let ptr = *self.shapers;
                if ptr.is_null() {
                    None
                } else {
                    self.shapers = self.shapers.offset(1);
                    Some(CStr::from_ptr(ptr))
                }
            }
        }
    }
}

pub fn list_shapers() -> Shapers {
    unsafe {
        Shapers {
            shapers: hb_shape_list_shapers(),
        }
    }
}

extern "C" {
    fn hb_feature_from_string(str: *const libc::c_char, len: i32,
                              feature: *mut Feature) -> hb_bool_t;
    fn hb_feature_to_string(feature: *mut Feature, buf: *mut libc::c_char, size: libc::c_uint);

    fn hb_shape(font: *mut hb_font_t, buffer: *mut hb_buffer_t, features: *const Feature,
                num_features: libc::c_uint);

    fn hb_shape_full(font: *mut hb_font_t, buffer: *mut hb_buffer_t, features: *const Feature,
                     num_features: libc::c_uint,
                     shaper_list: *const *const libc::c_char) -> hb_bool_t;

    fn hb_shape_list_shapers() -> *mut *const libc::c_char;
}
