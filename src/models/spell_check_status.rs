use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum SpellCheckStatus {
    #[serde(rename = "Udløbet")]
    Expired,

    #[serde(rename = "Afvist")]
    Dismissed,

    #[serde(rename = "Godkendt")]
    Approved,

    #[serde(rename = "Ikke Kontrolleret")]
    NotChecked,
}