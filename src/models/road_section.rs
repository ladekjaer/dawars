use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RoadSection {
    #[serde(rename = "id")]
    uuid: Uuid,

    #[serde(rename = "href")]
    href: Url,

    #[serde(rename = "kommunekode")]
    municipality_code: String,

    #[serde(rename = "kode")]
    code: String,

    #[serde(rename = "darstatus")]
    status: u8,
}

impl RoadSection {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn href(&self) -> &Url {
        &self.href
    }

    pub fn municipality_code(&self) -> &str {
        &self.municipality_code
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn status(&self) -> u8 {
        self.status
    }
}