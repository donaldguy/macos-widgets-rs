//! Here (if you're hackin') find structs and stuff

pub mod plist;
pub use plist::PlistDerivedStruct;

/// Structs corresponding to `PrivateFrameworks/ChronoServices.framework`
///
/// See also [crate::types::plist#widgetsinstanceswidget]
pub(self) mod chs {
    pub mod extension_identity;
    mod intent {
        pub mod recommendation;
        pub mod reference;
    }
    pub mod widget;
    pub mod widget_descriptor;
}

pub mod widget;

pub use chs::extension_identity::CHSExtensionIdentity;
pub use chs::widget::CHSWidget;
pub use chs::widget_descriptor::CHSWidgetDescriptor;
