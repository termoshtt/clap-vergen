use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    print_impl(input.into()).into()
}

fn print_impl(input: TokenStream2) -> TokenStream2 {
    let _ = input;
    unimplemented!()
}
