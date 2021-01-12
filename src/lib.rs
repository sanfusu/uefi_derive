use proc_macro::TokenStream;
use revision::revision_proc;
mod protocol;
mod revision;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Protocol, attributes(guid))]
pub fn protocol(input: TokenStream) -> TokenStream {
   protocol::protocol_derive(input.into()).into()
}

#[proc_macro]
pub fn rev(item: TokenStream) -> TokenStream {
    revision_proc(item.into()).into()
}
