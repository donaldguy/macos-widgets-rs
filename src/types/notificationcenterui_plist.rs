use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

use dirs::home_dir;
use plist_structs::FromPlist;
use plist_structs_derive::FromPlist;
use serde_derive::Deserialize;

pub mod widgets;

#[derive(Clone, Debug, Deserialize, FromPlist)]
pub struct NotificationCenterUIPlist {
    /// In my experience its "0"; but I'm guessing it might be "0", the [`FMFontStyle`](https://developer.apple.com/documentation/applicationservices/fmfontstyle)
    #[serde(rename = "fontStyle")]
    _font_style: i16,

    /// a semi-mysterous float of some sort, e.g. as I write this `738152425.610449`. An order too low to be a unix epoch time.
    #[serde(rename = "last-analytics-stamp")]
    _last_analytics_stamp: f64,

    // apple was really like: but "what if all of the cases"
    // (well, camel, kebab, and Pascal [for widgets(v1).DesktopWidgetPlacementStorage],
    // that snake case is represented is a matter of assumption about e.g. "widgets")
    pub widgets: widgets::WidgetsDict,
}

impl NotificationCenterUIPlist {
    fn from_file_at<P: AsRef<std::path::Path>>(
        p: P,
    ) -> std::result::Result<Self, plist_structs::Error> {
        FromPlist::from_file(p)
    }

    pub fn from_file() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let path = Self::plist_path()?;

        match Self::from_file_at(path) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    const BUNDLE_ID: &'static str = "com.apple.notificationcenterui";
    ///`$HOME/Library/Containers/com.apple.notificationcenterui/Data`
    fn container_dir() -> Result<PathBuf> {
        match home_dir() {
            Some(home) => Ok(home.join(format!("Library/Containers/{}/Data", Self::BUNDLE_ID))),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Could not determine user home directory",
            )),
        }
    }

    /// `${[container_dir()]}/Library/Preferences/com.apple.notificationcenterui.plist`
    pub fn plist_path() -> Result<PathBuf> {
        Ok(Self::container_dir()?.join(format!("Library/Preferences/{}.plist", Self::BUNDLE_ID)))
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn can_load_from_tests_file() -> std::result::Result<(), plist_structs::Error> {
        let ncui = NotificationCenterUIPlist::from_file_at(
            crate::test_files::notificationcenterui_plist(),
        )?;

        assert_eq!(ncui._font_style, 0);
        assert_eq!(ncui._last_analytics_stamp, 738152425.610449);
        assert_matches!(ncui.widgets, self::widgets::WidgetsDict::V1(_));
        Ok(())
    }

    #[cfg(all(target_os = "macos", test, feature = "smoketest-live-system"))]
    mod live_system_smoke {
        use super::super::*;

        #[test]
        fn ncui_container_exists() {
            assert!(NotificationCenterUIPlist::container_dir().unwrap().exists())
        }

        #[test]
        fn ncui_plist_exists() {
            assert!(NotificationCenterUIPlist::plist_path().unwrap().exists())
        }

        #[test]
        fn ncui_plist_parseable() {
            let _ = NotificationCenterUIPlist::from_file().unwrap();
        }
    }
}
