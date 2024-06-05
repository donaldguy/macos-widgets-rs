pub use plist::{Data as BinaryData, Dictionary, Error, Value as UnknownTypeValue};

mod traits;

pub use traits::FromPlist;
pub use traits::{Into, TryInto};

mod nested_binary_plist;
mod nskeyed_archiver_formatted_plist;

pub use nested_binary_plist::NestedBinaryPlist;
pub use nskeyed_archiver_formatted_plist::NSKeyedArchiverFormattedPlist;
