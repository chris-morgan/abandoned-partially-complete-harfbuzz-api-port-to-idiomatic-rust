//! Derived from `hb-blob.h`.

use std::slice;
use std::ptr;
use std::ops::Deref;

use libc;

use harfbuzz::common::{hb_destroy_func_t, into_user_data};

/// Equivalent to `hb_memory_mode_t`
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum MemoryMode {
    /// `HB_MEMORY_MODE_DUPLICATE`
    Duplicate,
    /// `HB_MEMORY_MODE_READONLY`
    Readonly,
    /// `HB_MEMORY_MODE_WRITABLE`
    Writable,
    /// `HB_MEMORY_MODE_READONLY_MAY_MAKE_WRITABLE`
    ReadonlyMayMakeWritable,
}

define_boxed_type! {
    struct Blob;
    enum hb_blob_t;
    fn hb_blob_get_empty;
    fn hb_blob_reference;
    fn hb_blob_destroy;
    fn hb_blob_set_user_data;
    fn hb_blob_get_user_data;
    fn hb_blob_make_immutable;
    fn hb_blob_is_immutable;
}

impl From<&'static [u8]> for Blob {
    fn from(data: &'static [u8]) -> Blob {
        unsafe {
            Blob::from(hb_blob_create(data as *const [u8] as *const libc::c_char,
                                      data.len() as libc::c_uint,
                                      MemoryMode::Readonly,
                                      ptr::null_mut(),
                                      None))
        }
    }
}

impl From<Vec<u8>> for Blob {
    fn from(data: Vec<u8>) -> Blob {
        unsafe {
            let data_ptr = data.as_ptr() as *const libc::c_char;
            let data_len = data.len() as libc::c_uint;
            let (user_data, destroy) = into_user_data(data);
            Blob::from(hb_blob_create(data_ptr, data_len, MemoryMode::Readonly, user_data,
                                      destroy))
        }
    }
}

impl Blob {
    /// Ideally this’d be `blob[start..end]`, but `Index` requires references.
    pub fn create_sub_blob(&self, offset: u32, length: u32) -> Blob {
        unsafe {
            Blob::from(hb_blob_create_sub_blob(self.ptr, offset, length))
        }
    }

    pub fn len(&self) -> u32 {
        unsafe {
            hb_blob_get_length(self.ptr)
        }
    }
}

impl Deref for Blob {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            let mut length = 0;
            let data_ptr = hb_blob_get_data(self.ptr, &mut length);
            slice::from_raw_parts(data_ptr as *const _, length as usize)
        }
    }
}

impl Blob {
    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        unsafe {
            let mut length = 0;
            let data_ptr = hb_blob_get_data_writable(self.ptr, &mut length);
            if data_ptr.is_null() {
                None
            } else {
                Some(slice::from_raw_parts_mut(data_ptr as *mut _, length as usize))
            }
        }
    }
}

extern "C" {
    fn hb_blob_create(data: *const libc::c_char,
                      length: libc::c_uint,
                      mode: MemoryMode,
                      user_data: *mut libc::c_void,
                      destroy: hb_destroy_func_t) -> *mut hb_blob_t;
    fn hb_blob_create_sub_blob(parent: *mut hb_blob_t,
                               offset: libc::c_uint,
                               length: libc::c_uint) -> *mut hb_blob_t;
    fn hb_blob_get_length(blob: *mut hb_blob_t) -> libc::c_uint;
    fn hb_blob_get_data(blob: *mut hb_blob_t, length: *mut libc::c_uint) -> *const libc::c_char;
    fn hb_blob_get_data_writable(blob: *mut hb_blob_t,
                                 length: *mut libc::c_uint) -> *mut libc::c_char;
}
