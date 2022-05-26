use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TransUnit {
    pub id: String,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Body {
    trans_unit: Vec<TransUnit>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct File {
    original: String,
    source_language: String,
    target_language: String,
    datatype: String,
    body: Body,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Xliff {
    version: String,
    xmln: String,
    file: File,
}

impl Xliff {
    pub fn translation_units(&self) -> &[TransUnit] {
        &self.file.body.trans_unit
    }
}
