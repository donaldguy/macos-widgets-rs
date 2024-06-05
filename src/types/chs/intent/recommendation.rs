use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CHSIntentRecommendation {
    intent2: super::reference::CHSIntentReference,
    desc: String,
}

#[derive(Clone, Deserialize)]
pub struct CHSIntentRecommendationsContainer {
    recommendations: Vec<CHSIntentRecommendation>,
    schema: Option<plist_structs::BinaryData>,
}

impl std::fmt::Debug for CHSIntentRecommendationsContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(data) = &self.schema {
            f.debug_struct("CHSIntentRecommendationsContainer")
                .field("schema", &format!("[{} bytes]", data.as_ref().len()))
                .field("recommendations", &self.recommendations)
                .finish()
        } else {
            self.recommendations.fmt(f)
        }
    }
}
