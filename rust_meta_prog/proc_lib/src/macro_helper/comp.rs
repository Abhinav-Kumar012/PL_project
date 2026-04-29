pub(crate) mod comprehension {
	use proc_macro2::TokenStream as TokenStream2;
	use quote::{ToTokens, quote};
	use syn::parse::{Parse, ParseStream};
	pub(crate) struct Comp {
		mapping: Mapping,
		for_if_clause: ForIfCaluse,
		additional_for_if_clauses: Vec<ForIfCaluse>,
	}

	impl Parse for Comp {
		fn parse(input: ParseStream) -> syn::Result<Self> {
			Ok(Self {
				mapping: input.parse()?,
				for_if_clause: input.parse()?,
				additional_for_if_clauses: parse_one_or_more(input),
			})
		}
	}

	impl ToTokens for Comp {
		fn to_tokens(
			&self,
			tokens: &mut TokenStream2,
		) {
			// let Mapping(mp) = &self.mapping;
			// let ForIfCaluse {
			// 	pattern,
			// 	expression : sequence,
			// 	conditions,
			// } = &self.for_if_clause;

			// tokens.extend( quote! {
			// 	::core::iter::IntoIterator::into_iter(#sequence).flat_map(
			// 		|#pattern| {
			// 			(true #(&& (#conditions))*).then(|| #mp)
			// 		}
			// 	)
			// });
			let all_for_if_clauses =
				std::iter::once(&self.for_if_clause).chain(&self.additional_for_if_clauses);
			let mut innermost_to_outermost = all_for_if_clauses.rev();

			let mut output = {
				// innermost is a special case--here we do the mapping
				let innermost = innermost_to_outermost
					.next()
					.expect("We know we have at least one ForIfClause (self.for_if_clause)");
				let ForIfCaluse {
					pattern,
					expression: sequence,
					conditions,
				} = innermost;

				let Mapping(mapping) = &self.mapping;

				quote! {
					core::iter::IntoIterator::into_iter(#sequence).filter_map(move |#pattern| {
						(true #(&& (#conditions))*).then(|| #mapping)
					})
				}
			};

			// Now we walk through the rest of the ForIfClauses, wrapping the current `output` in a new layer of iteration each time.
			// We also add an extra call to '.flatten()'.
			output = innermost_to_outermost.fold(output, |current_output, next_layer| {
				let ForIfCaluse {
					pattern,
					expression: sequence,
					conditions,
				} = next_layer;
				quote! {
					core::iter::IntoIterator::into_iter(#sequence).filter_map(|#pattern| {
						(true #(&& (#conditions))*).then(|| #current_output)
					})
					.flatten()
				}
			});

			tokens.extend(output)
		}
	}
	struct Mapping(syn::Expr);

	impl Parse for Mapping {
		fn parse(input: ParseStream) -> syn::Result<Self> {
			Ok(Self(input.parse()?))
		}
	}

	impl ToTokens for Mapping {
		fn to_tokens(
			&self,
			tokens: &mut TokenStream2,
		) {
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
		fn to_tokens(
			&self,
			tokens: &mut TokenStream2,
		) {
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
		fn to_tokens(
			&self,
			tokens: &mut TokenStream2,
		) {
			self.0.to_tokens(tokens);
		}
	}
}
