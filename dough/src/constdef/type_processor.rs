use quote::quote;
use syn::{Type, TypeArray, TypeTuple};

pub(super) fn recursive_process_tuple(tuple: &TypeTuple) -> Option<quote::__private::TokenStream> {
    let mut inner_decl = quote! {};
    for elem in tuple.elems.iter() {
        match elem {
            Type::Path(_) => {
                inner_decl = quote! {
                    #inner_decl
                    ::bagel::Constdef::DEFAULT,
                };
            }
            Type::Tuple(ref tuple) => {
                let ret = self::recursive_process_tuple(tuple)?;
                inner_decl = quote! {
                    #inner_decl
                    #ret,
                };
            }
            Type::Array(ref arr) => {
                let ret = self::recursive_process_array(arr)?;
                inner_decl = quote! {
                    #inner_decl
                    [#ret],
                };
            }
            _ => return None,
        }
    }
    Some(inner_decl)
}

pub(super) fn recursive_process_array(array: &TypeArray) -> Option<quote::__private::TokenStream> {
    let mut inner_decl = quote! {};
    let len = &array.len;
    match &*array.elem {
        Type::Path(_) => {
            inner_decl = quote! {
                #inner_decl
                ::bagel::Constdef::DEFAULT; #len
            };
        }
        Type::Array(arr) => {
            let ret = self::recursive_process_array(arr)?;
            inner_decl = quote! {
                #inner_decl
                #ret; #len
            };
        }
        Type::Tuple(ref tuple) => {
            let ret = self::recursive_process_tuple(tuple)?;
            inner_decl = quote! {
                #inner_decl
                (#ret); #len
            };
        }
        _ => return None,
    }
    Some(inner_decl)
}
