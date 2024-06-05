#[derive(Clone, Debug)]
pub struct NestedBinaryPlist<T> {
    value: T,
}

mod de {
    use serde::de::{Deserialize, DeserializeOwned, Visitor};
    use std::marker::PhantomData;

    struct NestedPlistVisitor<T> {
        phantom: PhantomData<T>,
    }

    impl<T> NestedPlistVisitor<T> {
        fn new() -> Self {
            Self {
                phantom: PhantomData,
            }
        }
    }

    impl<'de, T: DeserializeOwned> Visitor<'de> for NestedPlistVisitor<T> {
        type Value = super::NestedBinaryPlist<T>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("A byte array from the plist crate (nested)")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match plist::from_bytes(v) {
                Err(e) => Err(E::custom(format!("Plist processing failed: {:?}", e))),
                Ok(value) => Ok(Self::Value { value }),
            }
        }
    }

    impl<'de, T: DeserializeOwned> Deserialize<'de> for super::NestedBinaryPlist<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_bytes(NestedPlistVisitor::new())
        }
    }
}

impl<T> AsRef<T> for NestedBinaryPlist<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for NestedBinaryPlist<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

use super::{Into, PlistDerivedStruct, TryInto};

impl<T> Into<T> for NestedBinaryPlist<T>
where
    T: PlistDerivedStruct,
{
    fn plist_into(self) -> T {
        self.value
    }
}

impl<I, T> TryInto<T> for NestedBinaryPlist<I>
where
    I: TryInto<T>,
    T: PlistDerivedStruct,
{
    type Error = I::Error;

    fn plist_try_into(self) -> Result<T, Self::Error> {
        self.value.plist_try_into()
    }
}
