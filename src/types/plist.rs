//! Herein find the ab-[serde]-ity of bridging to plist (and docs of format)
//!
//! If you'd like to skip the steps, go look at [self::notificationcenterui::NotificationCenterUI] et al
#![doc = include_str!("../../doc/plist_info.md")]

use nested_binary_plist::NestedBinaryPlist;
use nskeyed_archiver_formatted_plist::NSKeyedArchiverFormattedPlist;
pub mod notificationcenterui_plist;
