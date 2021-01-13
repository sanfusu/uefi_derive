use guid::Guid;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

pub(crate) fn protocol_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();
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
                let guid_args: Guid = args.value().parse().unwrap();
                let data1 = guid_args.data1;
                let data2 = guid_args.data2;
                let data3 = guid_args.data3;
                let data4 = guid_args.data4;
                let impl_guid = quote! {
                    impl #generics #ident #generics {
                        pub fn guid()->guid::Guid{
                            guid::Guid {
                                data1:#data1,
                                data2:#data2,
                                data3:#data3,
                                data4: [#(#data4,)*]
                            }
                        }
                    }
                };
                output.extend(impl_guid);
            }
        }
    }
    output
}
