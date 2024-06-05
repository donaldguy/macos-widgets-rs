use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "CompatibilityVersion")]
#[non_exhaustive]
pub enum WidgetPlacement {
    _0,
    V1(v1::WidgetPlacement),
}

pub mod v1 {
    use plist_structs_derive::FromPlist;

    use super::*;

    #[derive(Clone, Debug, Deserialize, FromPlist, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct WidgetPlacement {
        numbered_displays: Vec<self::display::NumberedDisplays>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Item {
        identifier: uuid::Uuid,
        column: u8,
        row: u8,
        size: self::item::Size,

        /// while one presumes this may also determine what ends up on top (though/because the system rejects overlap usally),
        /// this number de-facto seems to be the order in which widgets were inserted or last clicked-and-dragged (highest number most recent, 0 longest ago)
        z_order: u8,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Group {
        origin: self::group::Origin,
        items: Vec<Item>,
    }

    pub mod display {
        #[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct NumberedDisplays {
            number: i8,
            resolutions: Vec<Display>,
        }

        #[derive(Clone, Debug, serde_derive::Deserialize, serde_derive::Serialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct Display {
            size: self::resolution::Size,
            groups: Vec<super::Group>,
        }

        mod resolution {
            use crate::types::widget::layout::ScreenSize as CrateScreenSize;

            #[derive(Clone, serde_derive::Deserialize, serde_derive::Serialize)]
            #[serde(from = "(f64,f64)")]
            #[serde(into = "(f64,f64)")]
            pub(super) struct Size {
                ss: CrateScreenSize,
            }

            impl std::fmt::Debug for Size {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.ss.fmt(f)
                }
            }

            impl From<(f64, f64)> for Size {
                fn from(value: (f64, f64)) -> Self {
                    let (width, height) = value;
                    Self {
                        ss: CrateScreenSize {
                            width: width as u16,
                            height: height as u16,
                        },
                    }
                }
            }

            impl Into<(f64, f64)> for Size {
                fn into(self) -> (f64, f64) {
                    (self.ss.width as f64, self.ss.height as f64)
                }
            }
        }
    }

    mod group {
        use crate::types::widget::layout::GroupOrigin as CrateGroupOrigin;

        #[derive(Clone, serde_derive::Deserialize, serde_derive::Serialize)]
        #[serde(from = "(f64,f64)")]
        #[serde(into = "(f64,f64)")]
        pub(super) struct Origin {
            go: CrateGroupOrigin,
        }

        impl std::fmt::Debug for Origin {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.go.fmt(f)
            }
        }

        impl From<(f64, f64)> for Origin {
            fn from(value: (f64, f64)) -> Self {
                let (f_left, f_top) = value;

                Self {
                    go: CrateGroupOrigin {
                        left: f_left as u16,
                        top: f_top as u16,
                    },
                }
            }
        }

        impl Into<(f64, f64)> for Origin {
            fn into(self) -> (f64, f64) {
                (self.go.left as f64, self.go.top as f64)
            }
        }
    }

    mod item {
        use crate::types::widget::WidgetSize as CrateWidgetSize;
        use std::str::FromStr;

        #[derive(Clone)]
        pub(super) struct Size(CrateWidgetSize);

        impl serde::ser::Serialize for Size {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut d = plist_structs::Dictionary::new();
                d.insert(
                    self.0.to_string(),
                    plist_structs::UnknownTypeValue::Dictionary(plist_structs::Dictionary::new()),
                );

                plist_structs::Dictionary::serialize(&d, serializer)
            }
        }

        impl<'de> serde::de::Deserialize<'de> for Size {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                match plist_structs::Dictionary::deserialize(deserializer) {
                    Err(e) => Err(serde::de::Error::custom(format!(
                        "malformed widget size: {:?}",
                        e
                    ))),
                    Ok(dict) => {
                        let k = dict.keys();
                        if k.len() != 1 {
                            Err(serde::de::Error::invalid_length(k.len(), &"1"))
                        } else {
                            let k0 = k.last().unwrap();
                            if let Some(plist_structs::UnknownTypeValue::Dictionary(v0)) =
                                dict.get(k0)
                            {
                                if !v0.is_empty() {
                                    Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Other(&format!(
                                            "non-empty dict: {:?}",
                                            v0
                                        )),
                                        &"empty dict",
                                    ))
                                } else {
                                    match CrateWidgetSize::from_str(k0) {
                                        Ok(s) => Ok(Self(s)),
                                        Err(invalid) => {
                                            if let CrateWidgetSize::Invalid(s) = invalid {
                                                Err(serde::de::Error::invalid_value(
                                            serde::de::Unexpected::Str(s.as_str()),
                                            &"one of 'Small', 'Medium', 'Large', or 'Extra Large'",
                                        ))
                                            } else {
                                                panic!(
                                                    "Somehow a valid size was returned in an Err"
                                                )
                                            }
                                        }
                                    }
                                }
                            } else {
                                Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Other(&format!("{:?}", dict.get(k0))),
                                    &"empty dict",
                                ))
                            }
                        }
                    }
                }
            }
        }

        impl std::fmt::Debug for Size {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{:?}", self.0))
            }
        }

        impl From<CrateWidgetSize> for Size {
            fn from(value: CrateWidgetSize) -> Self {
                Self(value)
            }
        }

        impl From<Size> for CrateWidgetSize {
            fn from(value: Size) -> Self {
                value.0
            }
        }
    }
}
