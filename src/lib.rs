extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

    let ident = macro_input.ident;
    let len: usize = match macro_input.data {
        Data::Enum(data_enum) => data_enum.variants.len(),
        _ => unreachable!(),
    };

    quote!(
        impl #ident {
            pub fn count() -> usize {
                #len
            }
        }
    )
    .into()
}
