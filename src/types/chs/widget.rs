use serde_derive::Deserialize;
use serde_repr::Deserialize_repr;

/// these numbers seem to line up fairly consistently; and to match the order
/// in [`WidgetFamily`](https://developer.apple.com/documentation/widgetkit/widgetfamily)
/// I'm still mostly inclined to prefer the string in [super::plist::notificationcenterui::widgets::placement]
#[derive(Clone, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CHSWidgetFamily {
    SystemSmall = 1,
    SystemMedium = 2,
    SystemLarge = 3,
    SystemExtraLarge = 4,
    // then the ones for watchOS that we can only presume
    // and unless something changes in macOS, safely ignore
    // AccessoryCircular = 5
    // AccessoryCorner = 6
    // AccessoryRectangular = 7
    // AccessoryInline = 8
}

#[derive(Clone, Debug, Deserialize)]
pub struct CHSWidget {
    // $classes = ["CHSWidget", "NSObject"]
    /// The name of the widget
    kind: String,
    /// a numericly-encoded enum of size
    family: CHSWidgetFamily,
    #[serde(rename = "extensionIdentity")]
    extension_identity: super::extension_identity::CHSExtensionIdentity,
    intent2: Option<super::intent::reference::CHSIntentReference>,
}

impl crate::types::PlistDerivedStruct for CHSWidget {}
