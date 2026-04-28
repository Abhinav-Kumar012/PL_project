use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

struct Comp {
	mapping: Mapping,
	for_if_clause: ForIfCaluse,
}

impl Parse for Comp {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self {
			mapping: input.parse()?,
			for_if_clause: input.parse()?,
		})
	}
}

impl ToTokens for Comp {
	fn to_tokens(
		&self,
		tokens: &mut TokenStream2,
	) {
		let Mapping(mp) = &self.mapping;
		let ForIfCaluse {
			pattern,
			expression : sequence,
			conditions,
		} = &self.for_if_clause;



		tokens.extend( quote! {
			core::iter::IntoIterator::into_iter(#sequence).filter_map(
				|#pattern| {
					(true #(&& (#conditions))*).then(|| #mp)
				}
			)
		});
	}
}
struct Mapping(syn::Expr);

impl Parse for Mapping {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self(input.parse()?))
	}
}

impl ToTokens for Mapping {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.0.to_tokens(tokens);
	}
}
struct ForIfCaluse {
	pattern: Pattern,
	expression: syn::Expr,
	conditions: Vec<Condition>,
}

impl Parse for ForIfCaluse {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		_ = input.parse::<syn::Token![for]>()?;
		let pattern = input.parse()?;
		_ = input.parse::<syn::Token![in]>()?;
		let expression = input.parse()?;
		let conditions = parse_one_or_more(input);
		Ok(Self {
			pattern,
			expression,
			conditions,
		})
	}
}

fn parse_one_or_more<T>(input: ParseStream) -> Vec<T>
where
	T: Parse,
{
	let mut res = Vec::new();
	while let Ok(item) = input.parse::<T>() {
		res.push(item);
	}
	res
}

struct Pattern(syn::Pat);

impl Parse for Pattern {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Self(syn::Pat::parse_single(input)?))
	}
}

impl ToTokens for Pattern {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.0.to_tokens(tokens);
	}
}

struct Condition(syn::Expr);

impl Parse for Condition {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		_ = input.parse::<syn::Token![if]>()?;
		Ok(Self(input.parse()?))
	}
}

impl ToTokens for Condition {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		self.0.to_tokens(tokens);
	}
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let ir = syn::parse_macro_input!(input as Comp);
	quote!{ #ir }.into()
}
