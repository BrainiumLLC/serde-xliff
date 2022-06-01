use regex::Captures;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::{
    fs::File as StdFile,
    io::BufReader,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum XliffError {
    #[error("Could not open file at {path:?}: {source:?}")]
    CouldNotOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error(transparent)]
    DeserializeError(#[from] serde_xml_rs::Error),
}

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
    pub trans_unit: Vec<TransUnit>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct File {
    pub original: String,
    pub source_language: String,
    pub target_language: String,
    pub datatype: String,
    pub body: Body,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Xliff {
    pub version: String,
    pub xmln: String,
    pub file: File,
}

impl Xliff {
    pub fn translation_units(&self) -> &[TransUnit] {
        &self.file.body.trans_unit
    }

    pub fn from_source_dir_and_localization(
        path: impl AsRef<Path>,
        localization: &str,
    ) -> Result<Self, XliffError> {
        let path = path.as_ref();

        let file_path = path.join(&localization).join("strings.xliff");
        let f = StdFile::open(&file_path).map_err(|source| XliffError::CouldNotOpenFile {
            path: file_path,
            source,
        })?;
        let xliff = serde_xml_rs::de::from_reader(BufReader::new(f))?;
        Ok(xliff)
    }
}
