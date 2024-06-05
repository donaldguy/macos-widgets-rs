use super::Error;

pub trait FromPlist: Sized + serde::de::DeserializeOwned {
    fn is_from_plist(&self) -> bool {
        true
    }

    fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        plist::from_file(path)
    }
}

pub trait TryInto<T: FromPlist> {
    type Error;

    fn plist_try_into(self) -> Result<T, Self::Error>;
}

pub trait Into<T: FromPlist> {
    fn plist_into(self) -> T;
}

impl<T: FromPlist> TryInto<T> for &plist::Value {
    type Error = plist::Error;

    fn plist_try_into(self) -> Result<T, Self::Error> {
        plist::from_value(self)
    }
}

// impl<T> Into<T> for dyn TryInto<T, Error = Box<dyn std::error::Error>>
// where
//     T: FromPlist,
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

impl FromPlist for plist::Dictionary {}
