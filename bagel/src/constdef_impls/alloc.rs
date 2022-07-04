extern crate alloc;
use crate::Constdef;

mod vec {
    use super::*;
    use alloc::vec::Vec;
    impl<T> Constdef for Vec<T> {
        const DEFAULT: Self = new!();
    }
}

mod string {
    use super::*;
    use alloc::string::String;
    impl Constdef for String {
        const DEFAULT: Self = new!();
    }
}
