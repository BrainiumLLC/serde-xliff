use serde::{Deserialize, Serialize};
use std::{fs::File as StdFile, io::BufReader};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct Xliff {
    version: String,
    xmln: String,
    file: File,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct File {
    original: String,
    source_language: String,
    target_language: String,
    datatype: String,
    body: Body,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct Body {
    trans_unit: Vec<TransUnit>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
struct TransUnit {
    id: String,
    source: String,
}

fn main() {
    let f = StdFile::open("test.xliff").expect("Could not open file");
    let xliff: Xliff =
        serde_xml_rs::de::from_reader(BufReader::new(f)).expect("Could not create Xliff object");
    println!("{:#?}", xliff);
}
