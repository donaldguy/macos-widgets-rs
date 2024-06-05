use serde_derive::Deserialize;

mod placement;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "vers")]
#[non_exhaustive]
pub enum WidgetsDict {
    /// a placeholder to semi-fragily work around lack of serde-rs/serde#745 completion
    /// whether this ever ends up useful will depend on if Apple (ever adds a new one and) keeps using whole numbers
    _0,
    /// current (and only?) version as of macOS Sanoma 14.5 (23F79)... but since the "vers" field is there
    V1(v1::WidgetsDict),
}

impl crate::types::PlistDerivedStruct for v1::InstantiatedWidgetContainer {}
impl crate::types::PlistDerivedStruct for v1::InstalledWidgetContainer {}
impl crate::types::PlistDerivedStruct for v1::WidgetsDict {}
impl crate::types::PlistDerivedStruct for WidgetsDict {}

pub mod v1 {
    use super::*;
    use crate::types::{
        plist::{NSKeyedArchiverFormattedPlist, NestedBinaryPlist, TryInto},
        CHSWidget, CHSWidgetDescriptor,
    };

    #[derive(Clone, Debug, Deserialize)]
    pub struct WidgetsDict {
        #[serde(rename = "DesktopWidgetPlacementStorage")]
        pub placement: NestedBinaryPlist<placement::v1::WidgetPlacement>,
        pub instances: Vec<NestedBinaryPlist<self::v1::InstantiatedWidgetContainer>>,
        pub widgets: Vec<NestedBinaryPlist<self::v1::InstalledWidgetContainer>>,
    }

    #[derive(Clone, Deserialize)]
    pub struct InstantiatedWidgetContainer {
        pub uuid: uuid::Uuid,
        pub widget: NestedBinaryPlist<NSKeyedArchiverFormattedPlist<CHSWidget>>,
    }

    impl std::fmt::Debug for InstantiatedWidgetContainer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let w: CHSWidget = self.widget.clone().to_owned().plist_try_into().unwrap();

            f.debug_tuple("Widget").field(&self.uuid).field(&w).finish()
        }
    }

    #[derive(Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InstalledWidgetContainer {
        pub encoded_descriptor:
            NestedBinaryPlist<NSKeyedArchiverFormattedPlist<CHSWidgetDescriptor>>,
        pub _localized_locale: String,
        pub _version: String,
        pub _mod_date: plist::Value,
    }

    impl std::fmt::Debug for InstalledWidgetContainer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let w: CHSWidgetDescriptor = self.encoded_descriptor.clone().plist_try_into().unwrap();
            w.fmt(f)

            // let mut w1 = self.encoded_descriptor.clone();
            // let w2 = w1.as_mut().try_decode().unwrap().as_dictionary();

            // w2.fmt(f)
        }
    }
}
