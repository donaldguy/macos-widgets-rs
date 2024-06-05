//! slightly tweak the work from [nskeyedarchiver_converter] crate to [serde] better

use std::marker::PhantomData;

use nskeyedarchiver_converter::Converter;
use nskeyedarchiver_converter::ConverterError;
use serde_derive::Deserialize;

use super::FromPlist;

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct NSKeyedArchiverFormattedPlist<T>
where
    T: FromPlist,
{
    raw: plist::Value,
    #[serde(skip)]
    converted: Option<plist::Value>,
    #[serde(skip)]
    target_type: PhantomData<T>,
}

impl<T> NSKeyedArchiverFormattedPlist<T>
where
    T: FromPlist,
{
    pub fn try_decode(&mut self) -> Result<&plist::Value, ConverterError> {
        if let None = self.converted {
            let mut c = Converter::new(self.raw.clone())?;
            match c.decode() {
                Err(e) => return Err(e.into()),
                Ok(v) => {
                    self.converted = Some(v);
                }
            }
        }

        Ok(self.converted.as_ref().unwrap())
    }
}

use super::TryInto;

impl<T> TryInto<T> for NSKeyedArchiverFormattedPlist<T>
where
    T: FromPlist,
{
    type Error = Box<dyn std::error::Error>;

    fn plist_try_into(mut self) -> Result<T, Self::Error> {
        match self.try_decode() {
            Err(e) => Err(e.into()),
            Ok(val) => {
                match val {
                    plist::Value::Dictionary(d) => {
                        match plist::from_value::<T>(d.get("root").unwrap()) {
                            Err(e) => Err(e.into()),
                            Ok(t) => Ok(t),
                        }
                    }
                    _ => panic!("this"),
                    // plist::Value::Array(_) => todo!(),
                    // plist::Value::Boolean(_) => todo!(),
                    // plist::Value::Data(_) => todo!(),
                    // plist::Value::Date(_) => todo!(),
                    // plist::Value::Real(_) => todo!(),
                    // plist::Value::Integer(_) => todo!(),
                    // plist::Value::String(_) => todo!(),
                    // plist::Value::Uid(_) => todo!(),
                    // _ => todo!(),
                }
            }
        }
    }
}
