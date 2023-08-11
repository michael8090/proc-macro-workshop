use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input_ast: syn::DeriveInput = syn::parse(input).expect("could not parse the input as DeriveInput");

    eprintln!("{:#?}", derive_input_ast);

    TokenStream::new()
}
