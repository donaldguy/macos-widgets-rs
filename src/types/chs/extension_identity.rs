use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CHSExtensionIdentity {
    // $classes = ["CHSExtensionIdentity", "NSObject"]
    pub container_bundle_identifier: String,
    pub extension_bundle_identifier: String,
    pub device_identifier: Option<Uuid>,
}
