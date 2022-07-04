mod alloc;
mod core;
mod primitive;

mod std {
    mod collections {
        use crate::Constdef;
        use std::collections::LinkedList;
        impl<T> Constdef for LinkedList<T> {
            const DEFAULT: Self = LinkedList::new();
        }
    }
}
