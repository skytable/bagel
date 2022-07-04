//! [`dough`]
//!
//! Consider using the [`bagel`](https://crates.io/crates/bagel) crate. The `dough` crate provides
//! supporting macros for bagel
//!

use proc_macro::TokenStream;

#[macro_use]
mod macros;
mod constdef;
mod ctor;
mod gtor;
mod stor;
mod util;

#[proc_macro_derive(Ctor, attributes(ctor_const, phantom))]
pub fn derive_ctor(input: TokenStream) -> TokenStream {
    ctor::derive_ctor(input)
}

#[proc_macro_derive(Gtor, attributes(gtor_const, gtor_copy, gtor_skip, phantom, gtor))]
pub fn derive_gtor(input: TokenStream) -> TokenStream {
    gtor::derive_gtor(input)
}

#[proc_macro_derive(Stor, attributes(stor_skip, phantom))]
pub fn derive_stor(input: TokenStream) -> TokenStream {
    stor::derive_stor(input)
}

#[proc_macro_derive(Constdef)]
pub fn derive_constdef(input: TokenStream) -> TokenStream {
    constdef::derive(input)
}
