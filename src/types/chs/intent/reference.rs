use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct CHSIntentReference {
    //$classes = ["CHSIntentReference", "NSObject"]
    #[serde(rename = "stableHash")]
    stable_hash: plist::Integer,

    idata: Option<plist::Value>,
    pcdata: Option<plist::Data>,
}

impl std::fmt::Debug for CHSIntentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        m.entry(&"stable_hash", &self.stable_hash);

        if let Some(i) = self.idata.as_ref() {
            match i {
                plist::Value::Data(d) => {
                    m.entry(&"idata", &format!("[{} bytes]", d.len()));
                }
                plist::Value::Dictionary(d) => {
                    if d.contains_key("NS.data") {
                        m.entry(
                            &"idata",
                            &format!(
                                "[NS.data: {} bytes]",
                                d.get("NS.data").unwrap().as_data().unwrap().len()
                            ),
                        );
                    } else {
                        m.entry(&"idata", &d);
                    }
                }
                v => {
                    m.entry(&"idata", &format!("[unexpected format: {:?}]", v));
                }
            }
        }

        if let Some(p) = self.pcdata.as_ref() {
            m.entry(&"pcdata", &format!("[{} bytes]", p.as_ref().len()));
        }

        m.finish()
    }
}
