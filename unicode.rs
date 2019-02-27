//! Derived from hb-unicode.rs

use std::mem;
use libc;

/// Unicode Character Database property: General_Category (gc)
///
/// Equivalent to `hb_unicode_general_category_t`.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum UnicodeGeneralCategory {
    /// Cc
    Control,
    /// Cf
    Format,
    /// Cn
    Unassigned,
    /// Co
    PrivateUse,
    /// Cs
    Surrogate,
    /// Ll
    LowercaseLetter,
    /// Lm
    ModifierLetter,
    /// Lo
    OtherLetter,
    /// Lt
    TitlecaseLetter,
    /// Lu
    UppercaseLetter,
    /// Mc
    SpacingMark,
    /// Me
    EnclosingMark,
    /// Mn
    NonSpacingMark,
    /// Nd
    DecimalNumber,
    /// Nl
    LetterNumber,
    /// No
    OtherNumber,
    /// Pc
    ConnectPunctuation,
    /// Pd
    DashPunctuation,
    /// Pe
    ClosePunctuation,
    /// Pf
    FinalPunctuation,
    /// Pi
    InitialPunctuation,
    /// Po
    OtherPunctuation,
    /// Ps
    OpenPunctuation,
    /// Sc
    CurrencySymbol,
    /// Sk
    ModifierSymbol,
    /// Sm
    MathSymbol,
    /// So
    OtherSymbol,
    /// Zl
    LineSeparator,
    /// Zp
    ParagraphSeparator,
    /// Zs
    SpaceSeparator,
}

pub(super) type hb_unicode_combining_class_t = libc::c_uint;

/// Unicode Character Database property: Canonical_Combining_Class (ccc)
///
/// Mostly equivalent to `hb_unicode_combining_class_t`, but not quite strictly equivalent:
///
/// > Note: newer versions of Unicode may add new values. Clients should be ready to handle
/// > any value in the 0..254 range being returned from hb_unicode_combining_class().
///
/// In this library, any such values are not accessible; if you need them, get this library
/// updated. Sorry, it just can’t be done both nicely and backwards compatibly in current Rust.
#[repr(C)]
pub enum UnicodeCombiningClass {
    NotReordered = 0,
    Overlay = 1,
    Nukta = 7,
    KanaVoicing = 8,
    Virama = 9,

    // Hebrew
    Ccc10 = 10,
    Ccc11 = 11,
    Ccc12 = 12,
    Ccc13 = 13,
    Ccc14 = 14,
    Ccc15 = 15,
    Ccc16 = 16,
    Ccc17 = 17,
    Ccc18 = 18,
    Ccc19 = 19,
    Ccc20 = 20,
    Ccc21 = 21,
    Ccc22 = 22,
    Ccc23 = 23,
    Ccc24 = 24,
    Ccc25 = 25,
    Ccc26 = 26,

    // Arabic
    Ccc27 = 27,
    Ccc28 = 28,
    Ccc29 = 29,
    Ccc30 = 30,
    Ccc31 = 31,
    Ccc32 = 32,
    Ccc33 = 33,
    Ccc34 = 34,
    Ccc35 = 35,

    // Syriac
    Ccc36 = 36,

    // Telugu
    Ccc84 = 84,
    Ccc91 = 91,

    // Thai
    Ccc103 = 103,
    Ccc107 = 107,

    // Lao
    Ccc118 = 118,
    Ccc122 = 122,

    // Tibetan
    Ccc129 = 129,
    Ccc130 = 130,
    Ccc133 = 132,

    AttachedBelowLeft  = 200,
    AttachedBelow      = 202,
    AttachedAbove      = 214,
    AttachedAboveRight = 216,
    BelowLeft          = 218,
    Below              = 220,
    BelowRight         = 222,
    Left               = 224,
    Right              = 226,
    AboveLeft          = 228,
    Above              = 230,
    AboveRight         = 232,
    DoubleBelow        = 233,
    DoubleAbove        = 234,

    IotaSubscript      = 240,

    Invalid = 255,

    // As noted, a hb_unicode_combining_class_t may not correspond to a value in this enum.
    #[doc(hidden)]
    __Nonexhaustive = 254,
}

impl From<hb_unicode_combining_class_t> for UnicodeCombiningClass {
    fn from(ucc: hb_unicode_combining_class_t) -> UnicodeCombiningClass {
        match ucc {
            0 | 1 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 |
            23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 84 | 91 | 103 |
            107 | 118 | 122 | 129 | 130 | 132 | 200 | 202 | 214 | 216 | 218 | 220 | 222 | 224 |
            226 | 228 | 230 | 232 | 233 | 234 | 240 | 255 => unsafe { mem::transmute(ucc) },
            _ => UnicodeCombiningClass::__Nonexhaustive,
        }
    }
}

/*impl From<UnicodeCombiningClass> for hb_unicode_combining_class_t {
    fn from(ucc: UnicodeCombiningClass) -> hb_unicode_combining_class_t {
        ucc as hb_unicode_combining_class_t
    }
}*/


// hb_unicode_funcs_t

define_boxed_type! {
    struct UnicodeFuncs;
    enum hb_unicode_funcs_t;
    fn hb_unicode_funcs_get_empty;
    fn hb_unicode_funcs_reference;
    fn hb_unicode_funcs_destroy;
    fn hb_unicode_funcs_set_user_data;
    fn hb_unicode_funcs_get_user_data;
    fn hb_unicode_funcs_make_immutable;
    fn hb_unicode_funcs_is_immutable;
}

impl UnicodeFuncs {
    /// “Just give me the best implementation you’ve got there.”
    pub fn get_default() -> UnicodeFuncs {
        UnicodeFuncs::from(unsafe { hb_unicode_funcs_get_default() })
    }

    pub fn create(parent: UnicodeFuncs) -> UnicodeFuncs {
        UnicodeFuncs::from(unsafe { hb_unicode_funcs_create(parent.ptr) })
    }

    pub fn get_parent(&self) -> UnicodeFuncs {
        UnicodeFuncs::from(unsafe { hb_unicode_funcs_get_parent(self.ptr) })
    }
}

extern "C" {
    fn hb_unicode_funcs_get_default() -> *mut hb_unicode_funcs_t;
    fn hb_unicode_funcs_create(parent: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
    fn hb_unicode_funcs_get_parent(ufuncs: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
}

/*
/*
 * funcs
 */

/* typedefs */

typedef hb_unicode_combining_class_t    (*hb_unicode_combining_class_func_t)    (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      unicode,
                                                                                 void               *user_data);
typedef unsigned int                    (*hb_unicode_eastasian_width_func_t)    (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      unicode,
                                                                                 void               *user_data);
typedef UnicodeGeneralCategory          (*hb_unicode_general_category_func_t)   (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      unicode,
                                                                                 void               *user_data);
typedef hb_codepoint_t                  (*hb_unicode_mirroring_func_t)          (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      unicode,
                                                                                 void               *user_data);
typedef hb_script_t                     (*hb_unicode_script_func_t)             (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      unicode,
                                                                                 void               *user_data);

typedef hb_bool_t                       (*hb_unicode_compose_func_t)            (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      a,
                                                                                 hb_codepoint_t      b,
                                                                                 hb_codepoint_t     *ab,
                                                                                 void               *user_data);
typedef hb_bool_t                       (*hb_unicode_decompose_func_t)          (hb_unicode_funcs_t *ufuncs,
                                                                                 hb_codepoint_t      ab,
                                                                                 hb_codepoint_t     *a,
                                                                                 hb_codepoint_t     *b,
                                                                                 void               *user_data);

/**
 * hb_unicode_decompose_compatibility_func_t:
 * @ufuncs: a Unicode function structure
 * @u: codepoint to decompose
 * @decomposed: address of codepoint array (of length %HB_UNICODE_MAX_DECOMPOSITION_LEN) to write decomposition into
 * @user_data: user data pointer as passed to hb_unicode_funcs_set_decompose_compatibility_func()
 *
 * Fully decompose @u to its Unicode compatibility decomposition. The codepoints of the decomposition will be written to @decomposed.
 * The complete length of the decomposition will be returned.
 *
 * If @u has no compatibility decomposition, zero should be returned.
 *
 * The Unicode standard guarantees that a buffer of length %HB_UNICODE_MAX_DECOMPOSITION_LEN codepoints will always be sufficient for any
 * compatibility decomposition plus an terminating value of 0.  Consequently, @decompose must be allocated by the caller to be at least this length.  Implementations
 * of this function type must ensure that they do not write past the provided array.
 *
 * Return value: number of codepoints in the full compatibility decomposition of @u, or 0 if no decomposition available.
 */
typedef unsigned int                    (*hb_unicode_decompose_compatibility_func_t)    (hb_unicode_funcs_t *ufuncs,
                                                                                         hb_codepoint_t      u,
                                                                                         hb_codepoint_t     *decomposed,
                                                                                         void               *user_data);

/* See Unicode 6.1 for details on the maximum decomposition length. */
#define HB_UNICODE_MAX_DECOMPOSITION_LEN (18+1) /* codepoints */

/* setters */

/**
 * hb_unicode_funcs_set_combining_class_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 1.0
 **/
void
hb_unicode_funcs_set_combining_class_func (hb_unicode_funcs_t *ufuncs,
                                           hb_unicode_combining_class_func_t func,
                                           void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_eastasian_width_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 1.0
 **/
void
hb_unicode_funcs_set_eastasian_width_func (hb_unicode_funcs_t *ufuncs,
                                           hb_unicode_eastasian_width_func_t func,
                                           void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_general_category_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 1.0
 **/
void
hb_unicode_funcs_set_general_category_func (hb_unicode_funcs_t *ufuncs,
                                            hb_unicode_general_category_func_t func,
                                            void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_mirroring_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 1.0
 **/
void
hb_unicode_funcs_set_mirroring_func (hb_unicode_funcs_t *ufuncs,
                                     hb_unicode_mirroring_func_t func,
                                     void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_script_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 1.0
 **/
void
hb_unicode_funcs_set_script_func (hb_unicode_funcs_t *ufuncs,
                                  hb_unicode_script_func_t func,
                                  void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_compose_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 0.9.2
 **/
void
hb_unicode_funcs_set_compose_func (hb_unicode_funcs_t *ufuncs,
                                   hb_unicode_compose_func_t func,
                                   void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_decompose_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 0.9.2
 **/
void
hb_unicode_funcs_set_decompose_func (hb_unicode_funcs_t *ufuncs,
                                     hb_unicode_decompose_func_t func,
                                     void *user_data, hb_destroy_func_t destroy);

/**
 * hb_unicode_funcs_set_decompose_compatibility_func:
 * @ufuncs: a Unicode function structure
 * @func: (closure user_data) (destroy destroy) (scope notified):
 * @user_data:
 * @destroy:
 *
 * 
 *
 * Since: 0.9.2
 **/
void
hb_unicode_funcs_set_decompose_compatibility_func (hb_unicode_funcs_t *ufuncs,
                                                   hb_unicode_decompose_compatibility_func_t func,
                                                   void *user_data, hb_destroy_func_t destroy);

/* accessors */

/**
 * Since: 0.9.2
 **/
hb_unicode_combining_class_t
hb_unicode_combining_class (hb_unicode_funcs_t *ufuncs,
                            hb_codepoint_t unicode);

/**
 * Since: 0.9.2
 **/
unsigned int
hb_unicode_eastasian_width (hb_unicode_funcs_t *ufuncs,
                            hb_codepoint_t unicode);

/**
 * Since: 0.9.2
 **/
UnicodeGeneralCategory
hb_unicode_general_category (hb_unicode_funcs_t *ufuncs,
                             hb_codepoint_t unicode);

/**
 * Since: 0.9.2
 **/
hb_codepoint_t
hb_unicode_mirroring (hb_unicode_funcs_t *ufuncs,
                      hb_codepoint_t unicode);

/**
 * Since: 0.9.2
 **/
hb_script_t
hb_unicode_script (hb_unicode_funcs_t *ufuncs,
                   hb_codepoint_t unicode);

/**
 * Since: 0.9.2
 **/
hb_bool_t
hb_unicode_compose (hb_unicode_funcs_t *ufuncs,
                    hb_codepoint_t      a,
                    hb_codepoint_t      b,
                    hb_codepoint_t     *ab);

/**
 * Since: 0.9.2
 **/
hb_bool_t
hb_unicode_decompose (hb_unicode_funcs_t *ufuncs,
                      hb_codepoint_t      ab,
                      hb_codepoint_t     *a,
                      hb_codepoint_t     *b);

/**
 * Since: 0.9.2
 **/
unsigned int
hb_unicode_decompose_compatibility (hb_unicode_funcs_t *ufuncs,
                                    hb_codepoint_t      u,
                                    hb_codepoint_t     *decomposed);

*/
