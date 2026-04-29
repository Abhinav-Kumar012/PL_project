mod macro_helper;
use macro_helper::comp::comprehension::Comp;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
	let ir = parse_macro_input!(input as Comp);
	quote! { #ir }.into()
}
