use plist_structs::FromPlist;
use plist_structs_derive::FromPlist;
use serde_derive::Deserialize;

#[test]
fn test_derive_private_struct() {
    #[derive(FromPlist, Deserialize)]
    struct SomeStruct {}

    let s = SomeStruct {};

    assert!(FromPlist::is_from_plist(&s));
}

#[test]
fn test_derive_pub_struct() {
    #[derive(FromPlist, Deserialize)]
    pub struct SomeStruct {}

    let s = SomeStruct {};

    assert!(FromPlist::is_from_plist(&s));
}

#[test]
fn test_derive_scoped_pub_struct() {
    #[derive(FromPlist, Deserialize)]
    pub(self) struct SomeStruct {}

    let s = SomeStruct {};

    assert!(FromPlist::is_from_plist(&s));
}
