use serde::{Deserialize, Serialize};

/// According to the official documentation, none of the fields are optional. However, the
/// `discontinued` field (da. *nedlagt*) can be `null`. This can make sense due to JavaScript using
/// both `null` and `undefined`. The same applies to field `effective` (da. *ikrafttrædelse*).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct History {
    #[serde(rename = "oprettet")]
    created: String,

    #[serde(rename = "ændret")]
    modified: String,

    #[serde(rename = "ikrafttrædelse")]
    effective: Option<String>,

    #[serde(rename = "nedlagt")]
    discontinued: Option<String>,
}

impl History {
    pub fn created(&self) -> &str {
        &self.created
    }

    pub fn modified(&self) -> &str {
        &self.modified
    }

    pub fn effective(&self) -> Option<&str> {
        match &self.effective {
            None => None,
            Some(effective) => Some(&effective),
        }
    }

    pub fn discontinued(&self) -> Option<&str> {
        match &self.discontinued {
            None => None,
            Some(discontinued) => Some(&discontinued),
        }
    }

    pub fn new(created: &str, modified: &str, effective: Option<&str>, discontinued: Option<&str>) -> Self {
        Self {
            created: created.to_string(),
            modified: modified.to_string(),
            effective: effective.map(|e| e.to_string()),
            discontinued: discontinued.map(|e| e.to_string()),
        }
    }
}
