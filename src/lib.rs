use std::convert::TryInto;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};
#[macro_use]
extern crate quote;

#[proc_macro_derive(Protocol, attributes(guid))]
pub fn protocol_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident;
    let generics = ast.generics;
    let mut output = quote! {};
    if let Some(guid_helper_attr) = ast
        .attrs
        .iter()
        .find(|attr| attr.path.to_token_stream().to_string() == "guid")
    {
        if let syn::Meta::NameValue(meta) = guid_helper_attr.parse_meta().unwrap() {
            if let syn::Lit::Str(args) = meta.lit {
                let guid_args: guid::Guid = args.value().try_into().unwrap();
                
                let impl_guid = quote! {
                    impl #generics #ident #generics {
                        pub fn guid()->guid::Guid{
                            #guid_args
                        }
                    }
                };
                output.extend(impl_guid.to_token_stream());
            }
        }
    }
    output.into()
}
