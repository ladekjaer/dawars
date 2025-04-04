use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Municipality {
    #[serde(rename = "href")]
    href: Url,

    #[serde(rename = "kode")]
    code: String,

    #[serde(rename = "navn")]
    name: Option<String>,
}

impl Municipality {
    pub fn href(&self) -> &Url {
        &self.href
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn name(&self) -> Option<&str> {
        match &self.name {
            None => None,
            Some(name) => Some(&name),
        }
    }

    pub fn new(href: Url, code: &str, name: Option<&str>) -> Self {
        Self {
            href,
            code: code.to_string(),
            name: name.map(|name| name.to_string()),
        }
    }
}