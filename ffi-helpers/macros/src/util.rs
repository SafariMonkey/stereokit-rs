use proc_macro2::{Ident, Span};
use proc_macro_crate::crate_name;

pub fn crate_ident_new() -> Ident {
    use proc_macro_crate::FoundCrate;

    let crate_name = match crate_name("ffi-helpers") {
        Err(e) => crate::abort!(Span::call_site(), &e),
        Ok(found) => match found {
            FoundCrate::Name(name) => name,
            FoundCrate::Itself => "crate".to_owned(),
        },
    };

    Ident::new(&crate_name, Span::call_site())
}
#[cfg(test)]
mod tests {
    #[derive()]
    struct Basic {
        foo: i32,
    }
}
