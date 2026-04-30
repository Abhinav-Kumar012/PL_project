pub(crate) mod derive_macro_helper {
    use quote::{ToTokens, quote};
    use syn::{Error, Item::{self, Enum, Struct}, parse::{Parse, ParseStream}};
    use proc_macro2::{Span, TokenStream as TokenStream2};


    pub(crate) enum ItemInfo {
        StructItem(StructInfo),
        EnumItem(EnumInfo)
    }

    impl Parse for ItemInfo{
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let parsed_item = input.parse::<Item>()?;
            match parsed_item {
                Struct(s) => Ok(Self::StructItem(StructInfo(s))),
                Enum(e) => Ok(Self::EnumItem(EnumInfo(e))),
                _ => Err(Error::new(Span::call_site(),"works only with structs and enums")),
            }
        }
    }
    impl ToTokens for ItemInfo {
        fn to_tokens(&self, tokens: &mut TokenStream2) {
            let id = match &self {
                Self::EnumItem(e) => &e.0.ident,
                Self::StructItem(s) => &s.0.ident,
            };
            
            let token_main = match &self {
                Self::EnumItem(e) => quote! { #e },
                Self::StructItem(s) => quote! { #s },
            };

            let tk = quote! {
                impl MyDescribe for #id {
                    fn describe() -> String {
                        #token_main
                    }
                }
            };
            tokens.extend(tk);
        }
    }
    

    struct EnumInfo(syn::ItemEnum);

    impl Parse for EnumInfo {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse()?))
        }
    }

    impl ToTokens for EnumInfo {
        fn to_tokens(&self, tokens: &mut TokenStream2) {
            let id = &self.0.ident;
            let num_vars = &self.0.variants.len();
            let res = quote! { format!("{} has {} variants", stringify!(#id),#num_vars) };
            tokens.extend(res);
        }
    }



    struct StructInfo(syn::ItemStruct);

    impl Parse for StructInfo {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse()?))
        }
    }

    impl ToTokens for StructInfo {
        fn to_tokens(&self, tokens: &mut TokenStream2) {
            let id = &self.0.ident;
            let num_filelds = &self.0.fields.len();
            let res = quote! {format!("{} has {} fields", stringify!(#id),#num_filelds)};
            tokens.extend(res);
        }
    }

} 