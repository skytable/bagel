use crate::Constdef;

macro_rules! impl_numeric {
    ($($ty:ty),*) => {
        $(impl Constdef for $ty {
            const DEFAULT: $ty = 0;
        })*
    };
}

impl_numeric! {
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
}

impl Constdef for f32 {
    const DEFAULT: f32 = 0.0;
}

impl Constdef for f64 {
    const DEFAULT: f64 = 0.0;
}

impl Constdef for char {
    const DEFAULT: char = '\0';
}

impl<'a> Constdef for &'a str {
    const DEFAULT: &'a str = "";
}

impl Constdef for bool {
    const DEFAULT: bool = false;
}

impl Constdef for () {
    const DEFAULT: () = ();
}
