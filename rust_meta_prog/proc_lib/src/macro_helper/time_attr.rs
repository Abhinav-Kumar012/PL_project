pub(crate) mod time {
    use proc_macro2::{TokenStream as TokenStream2};
    use quote::{ToTokens,quote};
    use syn::{ItemFn, parse::{Parse, ParseStream}};

    pub(crate) struct fn_attrs(ItemFn);

    impl Parse for fn_attrs {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse()?))
        }
    }

    impl ToTokens for fn_attrs {
        fn to_tokens(&self, tokens: &mut TokenStream2) {
            let itemfn = &self.0;
            let (sig,visiblity,block) = (&itemfn.sig,&itemfn.vis,&itemfn.block);
            let fn_name = &itemfn.sig.ident;
            let mod_fn = quote! {
                #visiblity #sig {
                    let __start = std::time::Instant::now();
                    let __res = (|| #block)();
                    let __duration = __start.elapsed();
                    println!("fn {} ran for {:?}",stringify!(#fn_name),__duration);
                    __res
                }
            };
            tokens.extend(mod_fn);
        }
    }

} 