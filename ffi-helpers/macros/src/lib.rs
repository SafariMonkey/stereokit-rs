extern crate proc_macro;

use std::iter::once;

use error::UnwrapDirty;
use proc_macro_error::{abort, proc_macro_error};
macro_rules! emit_error {
    ($e:expr, $($tts:tt)*) => {
        { proc_macro_error::emit_error!($e, $($tts)*); Err(error::Dirty) }
    };
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned, DeriveInput};

mod error;
mod util;

#[proc_macro_error]
#[proc_macro_derive(Transmutable)]
pub fn transmutable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_transmutable(&ast).unwrap_or_abort()
}

fn impl_transmutable(ast: &syn::DeriveInput) -> Result<TokenStream, error::Dirty> {
    let input_type = &ast.ident;
    let crate_path = util::crate_ident_new();

    let struct_ = match &ast.data {
        syn::Data::Struct(struct_) => Ok(struct_),
        syn::Data::Enum(_) => emit_error!(
            ast.span(),
            "Enums are not yet supported for #[derive(Transmutable)]"
        ),
        syn::Data::Union(_) => emit_error!(
            ast.span(),
            "Unions are not yet supported for #[derive(Transmutable)]"
        ),
    }?;

    let mut field_bounds = Vec::new();

    for (i, ty) in struct_
        .fields
        .iter()
        .map(|f| &f.ty)
        .map(|ty| quote!(#ty))
        .chain(once(quote!(#crate_path::EndFields)))
        .enumerate()
    {
        field_bounds.push(quote!(T: #crate_path::FieldType<#i>));
        field_bounds
            .push(quote!(<T as #crate_path::FieldType<#i>>::Type: #crate_path::Transmutable<#ty>));
    }

    let gen = quote! {
        unsafe impl<T> #crate_path::Transmutable<T> for #input_type where #(#field_bounds),* {
        }
        unsafe impl<T> #crate_path::Transmutable<#input_type> for T where #(#field_bounds),* {
        }
    };

    Ok(gen.into())
}

#[proc_macro_error]
#[proc_macro_derive(FieldType)]
pub fn field_type_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_field_type(&ast).unwrap_or_abort()
}

fn impl_field_type(ast: &syn::DeriveInput) -> Result<TokenStream, error::Dirty> {
    let mut impls = Vec::<proc_macro2::TokenStream>::new();
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let crate_path = util::crate_ident_new();

    let struct_ = match &ast.data {
        syn::Data::Struct(struct_) => Ok(struct_),
        syn::Data::Enum(_) => emit_error!(
            ast.span(),
            "Enums are not yet supported for #[derive(FieldType)]"
        ),
        syn::Data::Union(_) => emit_error!(
            ast.span(),
            "Unions are not yet supported for #[derive(FieldType)]"
        ),
    }?;

    for (i, ty) in struct_
        .fields
        .iter()
        .map(|f| &f.ty)
        .map(|ty| quote!(#ty))
        .chain(once(quote!(#crate_path::EndFields)))
        .enumerate()
    {
        impls.push(quote!(
            unsafe impl #impl_generics #crate_path::FieldType<#i> for #input_type #ty_generics #where_clause {
                type Type = #ty;
            }
        ));
    }

    let gen = quote! {
        #(#impls)*
    };

    Ok(gen.into())
}
