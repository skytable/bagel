use ::proc_macro::TokenStream;
use ::syn::{Attribute, Data, DataStruct, DeriveInput, Fields, Ident, Type};

pub(crate) type NamedFieldInfo<'a> = (&'a Ident, &'a Type, &'a Vec<Attribute>);
pub(crate) type UnnamedFieldInfo<'a> = (&'a Type, &'a Vec<Attribute>);
pub(crate) enum FieldInfo<'a> {
    Named(Vec<NamedFieldInfo<'a>>),
    Unnamed(Vec<UnnamedFieldInfo<'a>>),
}

/// Returns the field names and their corresponding type from the AST (returning an error
/// if it isn't a struct)
pub(crate) fn get_struct_field_names(ast: &DeriveInput) -> Result<FieldInfo<'_>, TokenStream> {
    match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => Ok(FieldInfo::Named(
            fields
                .named
                .iter()
                .map(|field| {
                    let fname = field.ident.as_ref().unwrap();
                    (fname, &field.ty, &field.attrs)
                })
                .collect(),
        )),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => Ok(FieldInfo::Unnamed(
            fields
                .unnamed
                .iter()
                .map(|field| (&field.ty, &field.attrs))
                .collect(),
        )),
        _ => Err(
            syn::Error::new_spanned(ast, "this macro can only be used on structs")
                .into_compile_error()
                .into(),
        ),
    }
}
