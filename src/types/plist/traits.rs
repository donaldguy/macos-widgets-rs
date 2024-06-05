pub trait PlistDerivedStruct: Sized + serde::de::DeserializeOwned {}

pub trait TryInto<T: PlistDerivedStruct> {
    type Error;

    fn plist_try_into(self) -> Result<T, Self::Error>;
}

pub trait Into<T: PlistDerivedStruct> {
    fn plist_into(self) -> T;
}

impl<T: PlistDerivedStruct> TryInto<T> for &plist::Value {
    type Error = plist::Error;

    fn plist_try_into(self) -> Result<T, Self::Error> {
        plist::from_value(self)
    }
}

// impl<T> Into<T> for dyn TryInto<T, Error = Box<dyn std::error::Error>>
// where
//     T: PlistDerivedStruct,
// {
//     fn into(self) -> T {
//         match self.try_into() {
//             Ok(t) => t,
//             Err(e) => panic!(
//                 "Conversion into {} failed: {:?}",
//                 std::any::type_name::<T>(),
//                 e
//             ),
//         }
//     }
// }

impl PlistDerivedStruct for plist::Dictionary {}
