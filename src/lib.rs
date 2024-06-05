#![allow(dead_code)]
#![doc = include_str!("../README.md")]

pub mod types;

use types::NotificationCenterUIPlist;

pub fn load_and_print() -> Result<(), Box<dyn std::error::Error>> {
    let p = NotificationCenterUIPlist::from_file().unwrap();

    println!("{:#?}", p);

    Ok(())
}

#[cfg(test)]
pub mod test_files {
    use std::path::PathBuf;

    fn test_file(path: &str) -> PathBuf {
        let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");

        test_dir.join(path)
    }

    pub fn notificationcenterui_plist() -> PathBuf {
        test_file("static_notificationcenterui.plist")
    }
}
