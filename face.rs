//! Derived from `hb-face.h`.

use libc;

use harfbuzz::blob::{Blob, hb_blob_t};
use harfbuzz::common::{Tag, hb_destroy_func_t};

define_boxed_type! {
    struct Face;
    enum hb_face_t;
    fn hb_face_get_empty;
    fn hb_face_reference;
    fn hb_face_destroy;
    fn hb_face_set_user_data;
    fn hb_face_get_user_data;
    fn hb_face_make_immutable;
    fn hb_face_is_immutable;
}

impl Face {
    pub fn new(blob: Blob, index: u32) -> Face {
        unsafe {
            Face::from(hb_face_create(blob.ptr, index))
        }
    }

    /*
    // XXX: I’m not by any means certain this is perfect. I don’t need it myself, see.
    pub fn create_for_tables<T>(reference_table_func: extern "C" fn(face: *mut hb_face_t, tag: Tag, user_data: *mut T) -> Blob,
                                user_data: T) -> Face {
        // We want an fn(&mut Face, Tag, T) -> Blob.
        // It wants an extern fn(*mut hb_face_t, Tag, *mut libc::c_void) -> *mut hb_blob_t.
        // TODO.
        unsafe {
            let (user_data, destroy) = into_user_data(user_data);
            Face::from(hb_face_create_for_tables(
                Some((|face, tag, user_data| reference_table_func(face, tag, user_data).ptr)
                     as hb_reference_table_func_t__inner),
                user_data,
                destroy))
        }
    }*/

    // XXX: should these be &self or &mut self?
    pub fn table(&self, tag: Tag) -> Blob {
        unsafe {
            Blob::from(hb_face_reference_table(self.ptr, tag))
        }
    }

    pub fn blob(&self) -> Blob {
        unsafe {
            Blob::from(hb_face_reference_blob(self.ptr))
        }
    }

    pub fn index(&self) -> u32 {
        unsafe {
            hb_face_get_index(self.ptr)
        }
    }

    pub fn set_index(&self, index: u32) {
        unsafe {
            hb_face_set_index(self.ptr, index)
        }
    }

    pub fn upem(&self) -> u32 {
        unsafe {
            hb_face_get_upem(self.ptr)
        }
    }

    pub fn set_upem(&self, upem: u32) {
        unsafe {
            hb_face_set_upem(self.ptr, upem)
        }
    }

    pub fn glyph_count(&self) -> u32 {
        unsafe {
            hb_face_get_glyph_count(self.ptr)
        }
    }

    pub fn set_glyph_count(&self, glyph_count: u32) {
        unsafe {
            hb_face_set_glyph_count(self.ptr, glyph_count)
        }
    }

}

// FFI:

type hb_reference_table_func_t__inner = extern "C" fn(face: *mut hb_face_t, tag: Tag,
                                                      user_data: *mut libc::c_void)
                                                   -> *mut hb_blob_t;
type hb_reference_table_func_t = Option<hb_reference_table_func_t__inner>;
extern "C" {
    fn hb_face_create(blob: *mut hb_blob_t, index: libc::c_uint) -> *mut hb_face_t;
    fn hb_face_create_for_tables(reference_table_func: hb_reference_table_func_t,
                                 user_data: *mut libc::c_void,
                                 destroy: hb_destroy_func_t) -> *mut hb_face_t;
    fn hb_face_reference_table(face: *mut hb_face_t, tag: Tag) -> *mut hb_blob_t;
    fn hb_face_reference_blob(face: *mut hb_face_t) -> *mut hb_blob_t;
    fn hb_face_set_index(face: *mut hb_face_t, index: libc::c_uint);
    fn hb_face_get_index(face: *mut hb_face_t) -> libc::c_uint;
    fn hb_face_set_upem(face: *mut hb_face_t, upem: libc::c_uint);
    fn hb_face_get_upem(face: *mut hb_face_t) -> libc::c_uint;
    fn hb_face_set_glyph_count(face: *mut hb_face_t, glyph_count: libc::c_uint);
    fn hb_face_get_glyph_count(face: *mut hb_face_t) -> libc::c_uint;
}
