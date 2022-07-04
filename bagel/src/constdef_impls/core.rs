mod mem {
    use crate::Constdef;
    use core::mem::MaybeUninit;
    impl<T> Constdef for MaybeUninit<T> {
        const DEFAULT: Self = Self::uninit();
    }
}

mod cell {
    use crate::Constdef;
    use core::cell::Cell;
    use core::cell::RefCell;
    use core::cell::UnsafeCell;
    impl<T: Constdef> Constdef for Cell<T> {
        const DEFAULT: Self = Cell::new(T::DEFAULT);
    }
    impl<T: Constdef> Constdef for RefCell<T> {
        const DEFAULT: Self = RefCell::new(T::DEFAULT);
    }
    impl<T: Constdef> Constdef for UnsafeCell<T> {
        const DEFAULT: Self = UnsafeCell::new(T::DEFAULT);
    }
}

mod marker {
    use crate::Constdef;
    use core::marker::PhantomData;
    impl<T> Constdef for PhantomData<T> {
        const DEFAULT: Self = Self;
    }
}

mod option {
    use crate::Constdef;
    use core::option::Option;
    impl<T> Constdef for Option<T> {
        const DEFAULT: Self = Self::None;
    }
}

mod result {
    use crate::Constdef;
    use core::result::Result;
    impl<T, E: Constdef> Constdef for Result<T, E> {
        const DEFAULT: Self = Self::Err(E::DEFAULT);
    }
}

mod sync {
    mod atomic {
        // allow this because Constdef can also be used for statics
        #![allow(clippy::declare_interior_mutable_const)]
        use crate::Constdef;
        use core::ptr;
        use core::sync::atomic::AtomicBool;
        use core::sync::atomic::AtomicI16;
        use core::sync::atomic::AtomicI32;
        use core::sync::atomic::AtomicI64;
        use core::sync::atomic::AtomicI8;
        use core::sync::atomic::AtomicIsize;
        use core::sync::atomic::AtomicPtr;
        use core::sync::atomic::AtomicU16;
        use core::sync::atomic::AtomicU32;
        use core::sync::atomic::AtomicU64;
        use core::sync::atomic::AtomicU8;
        use core::sync::atomic::AtomicUsize;

        impl Constdef for AtomicBool {
            const DEFAULT: Self = Self::new(false);
        }
        macro_rules! impl_atomic_numeric {
            ($($atomic:ty),*) => {
                $(impl $crate::Constdef for $atomic { const DEFAULT: Self = new!($atomic, 0); })*
            };
        }
        impl_atomic_numeric!(
            AtomicU8,
            AtomicI8,
            AtomicU16,
            AtomicI16,
            AtomicU32,
            AtomicI32,
            AtomicU64,
            AtomicI64,
            AtomicUsize,
            AtomicIsize
        );
        impl<T> Constdef for AtomicPtr<T> {
            const DEFAULT: Self = Self::new(ptr::null_mut());
        }
    }
}

mod time {
    use crate::Constdef;
    use core::time::Duration;
    impl Constdef for Duration {
        const DEFAULT: Self = Self::from_secs(0);
    }
}
