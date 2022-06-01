use regex::Captures;
use serde::{de::Visitor, Deserialize, Deserializer};

#[derive(Debug)]
pub struct ArgumentString {
    pub sections: Vec<String>,
    pub format_string: String,
}

impl From<&str> for ArgumentString {
    fn from(string: &str) -> Self {
        // this implementation behaves subtly differently than our old games when encountering the literal string %%
        let separator = regex::Regex::new(r"%([0-9]+)").unwrap();
        let sections = separator
            .split(&string)
            .map(str::to_string)
            .collect::<Vec<_>>();
        let format_string = separator
            .replace_all(&string, |caps: &Captures| format!("{{arg_{}}}", &caps[1]))
            .to_string();
        Self {
            sections,
            format_string,
        }
    }
}

struct ArgumentStringVisitor;
impl<'de> Visitor<'de> for ArgumentStringVisitor {
    type Value = ArgumentString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string with optional arguments marked by `%`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let argument_string = value.into();
        Ok(argument_string)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ArgumentString>,
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
