use std::borrow::Borrow;

use proc_macro;
use proc_macro::{TokenStream, TokenTree::*};

#[proc_macro_derive(FromPlist)]
pub fn derive_from_plist(ts: TokenStream) -> TokenStream {
    let ets = ts.clone();

    let mut struct_name: Option<String> = None;

    let mut i: usize = 0;
    for t in ts {
        if let (0, Ident(s)) = (i, t.borrow()) {
            match s.to_string().as_str() {
                "pub" => {
                    continue;
                }
                "struct" => {
                    i += 1;
                    continue;
                }
                _ => break,
            }
        } else if let (1, Ident(n)) = (i, t.borrow()) {
            struct_name = Some(n.to_string());
            continue;
        } else if let (2, Group(_)) = (i, t) {
            continue;
        }
    }

    if let Some(name) = struct_name {
        format!("impl plist_structs::FromPlist for {} {{}}", name)
            .parse()
            .unwrap()
    } else {
        panic!(
            "PlistStruct only applies to (traditional, not unit or tuple) structs, given: {:#?}",
            ets
        );
    }
}
