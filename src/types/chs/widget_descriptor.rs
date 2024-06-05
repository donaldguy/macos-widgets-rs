use enumflags2::{bitflags, BitFlags};
use plist_structs_derive::FromPlist;
use serde_derive::Deserialize;
use serde_repr::Deserialize_repr;

use super::intent::reference::CHSIntentReference;

/// Speculated meanings of values in supported_size_classes
#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum CHSWidgetFamilyMask {
    // low bit (1) appears to be unused (always 0)
    WidgetSmall = 1 << 1,  // 2
    WidgetMedium = 1 << 2, // 4
    WidgetLarge = 1 << 3,  // 8
    WidgetXL = 1 << 4,     // 16

    // bits 5 [32] - 9 [512] appear currently unused
    // or used by the watchOS "accessory" / "complication" types
    // but not synced to macOS?

    // EXCEPT:
    AppleNewsSpecialCase = 1 << 6,
    // It appears (as 0b0000100001001110) in 2 copies (different platforms)
    // of the com.apple.news "today" widget
    // what does it do? ¯\_(ツ)_/¯. Maybe something with notification summaries?
    // maybe gets a higher refresh rate limit?

    // It may be a mistake to assume that ^^low-bit meanings remain
    // same when anything below here is also set
    // and indeed to assume same across SDK versions.

    // but at least appearing alone (or alone together),
    // I'm confident in these 2:
    LockScreenSingle = 1 << 10, // 1024
    LockScreenDouble = 1 << 11, // 2048

    /// lots less sure about this one. of my 552, only 70 have set.
    /// with low bits kinda all over the place.
    ShortcutsAndSpotlight = 1 << 12, // 4096
}
// 3 high bits (in u16) also appear currently unused

/// Guessed based on rare use of 2, by photos and find my
#[derive(Clone, Copy, Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CHSWidgetVisibilityKey {
    NoRestriction = 0,
    _NotCurrentlyUsed = 1,
    RequiresUnlockOnStandy = 2,
}

#[derive(Clone, Debug, Deserialize, FromPlist)]
#[serde(rename_all = "camelCase")]
pub struct CHSWidgetDescriptor {
    // $classes = 420 * {"CHSMutableWidgetDescriptor", "CHSWidgetDescriptor",  "CHSBaseDescriptor", "NSObject"]
    //            133 * {"CHSWidgetDescriptor", "CHSBaseDescriptor", "NSObject"]
    display_name: Option<String>,       // from CHSBaseDescriptor
    kind: String,                       // from CHSWidgetDescriptor
    widget_description: Option<String>, // (surprisingly) from CHSBaseDescriptor

    #[serde(rename = "nativeCBI")]
    native_container_bundle_identifier: Option<String>, // from CHSBaseDescriptor
    extension_identity: super::extension_identity::CHSExtensionIdentity,

    // from CHSBaseDescriptor
    /// My counts break down as:
    ///  46             platform: 1,
    ///  56             platform: 2,
    /// 451             platform: 3,
    ///
    /// There is decent evidence that 1 means macOS (w/ SDK version 14.X)
    /// But 2 and 3 both seem to point toward iOS & I can't tell if:
    /// - possibly iPhone only vs iPhone+iPad.
    /// - simply different generations of API?
    platform: u8, // from CHSBaseDescriptor

    sdk_version: String, // from CHSBaseDescriptor
    //  10             sdk_version: "14.2",
    //  13             sdk_version: "14.4",
    //  23             sdk_version: "14.5",
    //  19             sdk_version: "16.4",
    //  22             sdk_version: "17.0",
    // 148             sdk_version: "17.2",
    // 188             sdk_version: "17.4",
    // 130             sdk_version: "17.5",
    #[serde(rename = "backgroundStyle")]
    preferred_background_style: u8, // CHSBaseDescriptor; 536 = 0, 1 =1 (com.apple.Batteries), 16 = 2
    background_removable: bool, // CHSWidgetDescriptor; 501 true, 52 false

    supports_interaction: bool, // CHSWidgetDescriptor; 551 true, 2 false
    supports_vibrant_content: bool, // CHSBaseDescriptor; 7 true, 546 false

    widget_visibility: CHSWidgetVisibilityKey,

    /// aka (in the, .tbd supportedFamilies)
    pub supported_size_classes: BitFlags<CHSWidgetFamilyMask>, // u16,

    /// here, as probably elsewhere, intent meaning ~Shortcuts.
    intent_type: Option<plist_structs::UnknownTypeValue>, // 312 present v 214 absent
    #[serde(rename = "defaultIntent2")]
    default_intent_reference: Option<CHSIntentReference>,
    intent_recommendations_container:
        Option<super::intent::recommendation::CHSIntentRecommendationsContainer>,
    /// this shows filled in for me only for the Safari extension Noir
    /// and once for Firefox Focus ?
    fetch_default_intent_completions: Option<String>,
    //---
    // Excluded fields:
    //---
    // # Cause consistent across all 553 on my mac:

    // // I imagine same info as _localized_locale one level up
    // locale_token: plist::Data, // from CHSBaseDescriptor

    // //currently always 1
    // version: plist::Integer,  // from CHSBaseDescriptor

    // // current always false?
    // #[serde(rename = "hiddenBySensitiveUI")]
    // hidden_by_sensitive_ui: bool,  // from CHSBaseDescriptor
    // enables_multiple_tap_targets: bool, // from CHSWidgetDescriptor

    //---
    // # Cause not much used:
    // // only set, for me twice, both com.apple.chrono.event-service.gamed
    // // so I'm gonna assume not accesible to third parties?
    // event_mach_service_name: Option<String>,
    //---
    // # Cause in the ChronoServices.tbd, but absent across the board:
    // - defaultIntentPorvider
    // - didMigrateToRecommendationsContainer

    // of "disfavoredLocations" from the .tbd:
    // there is sometimes a key in the plists called "unsupLoca",
    // that points at an NSDictionary, but both keys & values
    // point at integers of unclear-to-me meaning (values are all(?) 8)
    // it makes nskeyedarchiver_converter spuriously detect circular references
}
