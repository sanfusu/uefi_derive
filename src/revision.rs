use std::str::FromStr;

use proc_macro2::TokenStream;
use uefi_raw::system_table::Revision;
use version::Version;

pub(crate) fn revision_proc(item: TokenStream) -> TokenStream {
    let rev_str = item.to_string();
    let revision: Revision = Revision::from_str(&rev_str).unwrap();
    let major = revision.major();
    let minor = revision.minor();
    quote! {
       Revision::new(#major, #minor)
    }
}
