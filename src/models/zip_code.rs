use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ZipCode {
    #[serde(rename = "href")]
    href: Url,

    #[serde(rename = "nr")]
    code: String,

    #[serde(rename = "navn")]
    name: Option<String>,
}

impl ZipCode {
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

    pub fn new(href: &Url, code: &str, name: Option<&str>) -> Self {
        Self {
            href: href.to_owned(),
            code: code.to_string(),
            name: name.map(String::from),
        }
    }
}
