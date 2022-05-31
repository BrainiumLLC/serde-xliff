use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug)]
pub struct ArgumentString {
    // TODO: `&'static str`?
    sections: Vec<String>,
}

impl From<String> for ArgumentString {
    fn from(string: String) -> Self {
        let sections = string.split("%").map(str::to_string).collect::<Vec<_>>();
        Self { sections }
    }
}

struct ArgumentStringVisitor;
impl<'de> Visitor<'de> for ArgumentStringVisitor {
    type Value = ArgumentString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with optional arguments marked by `%`")
    }
}

impl<'de> Deserialize<'de> for ArgumentString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ArgumentStringVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TransUnit {
    pub id: String,
    pub source: ArgumentString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Body {
    trans_unit: Vec<TransUnit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct File {
    original: String,
    source_language: String,
    target_language: String,
    datatype: String,
    body: Body,
}

#[derive(Debug, Deserialize)]
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
