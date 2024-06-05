//! Herein find the ab-[serde]-ity of bridging to plist (and docs of format)
//!
//! If you'd like to skip the steps, go look at [self::notificationcenterui::NotificationCenterUI] et al
#![doc = include_str!("../../doc/plist_info.md")]

mod traits;
pub use traits::PlistDerivedStruct;
pub use traits::{Into, TryInto};

mod nested_binary_plist;
mod nskeyed_archiver_formatted_plist;

use nested_binary_plist::NestedBinaryPlist;
use nskeyed_archiver_formatted_plist::NSKeyedArchiverFormattedPlist;

pub mod notificationcenterui;
