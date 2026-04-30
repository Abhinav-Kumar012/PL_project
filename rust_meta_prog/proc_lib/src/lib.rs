mod macro_helper;
use macro_helper::{comp::comprehension::Comp, time_attr::time::fn_attrs};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use macro_helper::dr::derive_macro_helper::ItemInfo;

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
	let ir = parse_macro_input!(input as Comp);
	quote! { #ir }.into()
}

#[proc_macro_attribute]
pub fn log_time(
	_attr: TokenStream,
	input: TokenStream,
) -> TokenStream {
	let ir = parse_macro_input!(input as fn_attrs);
	quote! { #ir }.into()
}

#[proc_macro_derive(MyDescribe)]
pub fn derivemacro(input: TokenStream) -> TokenStream {
	let ir = parse_macro_input!(input as ItemInfo);
	quote! { #ir }.into()
}
