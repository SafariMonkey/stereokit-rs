extern crate proc_macro;

use std::iter::once;

use error::UnwrapDirty;
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
macro_rules! emit_error {
    ($e:expr, $($tts:tt)*) => {
        { proc_macro_error::emit_error!($e, $($tts)*); Err(error::Dirty) }
    };
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned, DeriveInput, Meta, NestedMeta};

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

    let mut where_bounds = Vec::new();

    where_bounds.push(quote!(#input_type: #crate_path::Repr));
    where_bounds.push(quote!(T: #crate_path::Repr<Repr = <#input_type as Repr>::Repr>));

    for (i, ty) in struct_
        .fields
        .iter()
        .map(|f| &f.ty)
        .map(|ty| quote!(#ty))
        .chain(once(quote!(#crate_path::EndFields)))
        .enumerate()
    {
        where_bounds.push(quote!(T: #crate_path::FieldType<#i>));
        where_bounds
            .push(quote!(<T as #crate_path::FieldType<#i>>::Type: #crate_path::Transmutable<#ty>));
    }

    let gen = quote! {
        unsafe impl<T> #crate_path::Transmutable<T> for #input_type where #(#where_bounds),* {
        }
        unsafe impl<T> #crate_path::Transmutable<#input_type> for T where #(#where_bounds),* {
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

/// Extracts the contents of a #[repr(...)].
/// Returns empty vec if the contents are not a repr.
/// If the attribute is repr and fails to parse, abort.
/// If the contents are wrongly shaped (e.g. #[repr = ...]),
/// it emits an error and continues.
fn get_meta_items(attr: &syn::Attribute) -> Result<Vec<NestedMeta>, error::Dirty> {
    if !attr.path.is_ident("repr") {
        return Ok(Vec::new());
    }
    match attr.parse_meta() {
        Ok(Meta::List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(_) => {
            emit_error!(attr, "Expected #[repr(...)]")
        }
        Err(e) => {
            crate::abort!(e)
        }
    }
}

#[proc_macro_error]
#[proc_macro_derive(Repr)]
pub fn repr_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_repr(&ast).unwrap_or_abort()
}

fn impl_repr(ast: &syn::DeriveInput) -> Result<TokenStream, error::Dirty> {
    let input_type = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let crate_path = util::crate_ident_new();

    let metas: Vec<NestedMeta> = ast
        .attrs
        .iter()
        .map(get_meta_items)
        .collect::<Result<Vec<Vec<NestedMeta>>, error::Dirty>>()? // return error on _last_ failure
        .into_iter()
        .flatten()
        .collect();
    let repr = match &metas[..] {
        [] => {
            emit_error!(
                ast, "no repr attributes found";
                note = "Repr should only be derived for defined repr";
                help = "add #[repr(C)] or #[repr(transparent)] to the type's attributes"
            )
        }
        [NestedMeta::Meta(Meta::Path(word))] => Ok(word),
        [unexpected] => {
            emit_error!(unexpected.span(), "unexpected attribute")
        }
        [first, .., last] => {
            let span = error::SpanRange {
                first: first.span(),
                last: last.span(),
            };
            emit_error!(span, "multiple repr attributes are not yet supported")
        }
    }?;

    let repr_ty = if repr.is_ident(&syn::Ident::new("C", Span::call_site())) {
        quote!(#crate_path::ReprC)
    } else if repr.is_ident(&syn::Ident::new("transparent", Span::call_site())) {
        quote!(#crate_path::ReprTransparent)
    } else {
        abort!(
            repr,
            "only #[repr(C)] and #[repr(transparent)] are supported"
        )
    };

    let gen = quote!(
        unsafe impl #impl_generics #crate_path::Repr for #input_type #ty_generics #where_clause {
            type Repr = #repr_ty;
        }
    );

    Ok(gen.into())
}
