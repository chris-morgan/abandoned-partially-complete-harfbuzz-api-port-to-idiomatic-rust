// Scale seems to have this weird 16.16 arrangement. TODO find out more about it.

#[repr(C)]
struct Fixed(i32);

impl From<f64> for Fixed {
    fn from(f: f64) -> Fixed {
        Fixed(((1i32 << 16) as f64 * f) as i32)
    }
}

impl From<Fixed> for f64 {
    fn from(fixed: Fixed) -> f64 {
        fixed.0 as f64 * 1.0f64 / ((1i32 << 16) as f64)
    }
}
