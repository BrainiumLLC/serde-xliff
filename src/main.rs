mod xliff;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs::File as StdFile, io::BufReader};
use thiserror::Error;
use xliff::Xliff;

#[derive(Debug, Error)]
pub enum XliffError {
    #[error("path {0:?} contained invalid utf-8")]
    InvalidUtf8(PathBuf),
}

#[derive(Debug, Default)]
struct StringFiles {
    files: HashMap<String, Xliff>,
}

fn from_source_dir(path: impl AsRef<Path>) -> Result<StringFiles, XliffError> {
    let path = path.as_ref();
    let mut files = StringFiles::default();
    for localization in std::fs::read_dir(&path).unwrap() {
        if let Ok(entry) = localization {
            assert!(entry.file_type().unwrap().is_dir());
            let directory = entry.file_name();
            let f = StdFile::open(path.join(&directory).join("strings.xliff"))
                .expect("Could not open file");
            let xliff: Xliff = serde_xml_rs::de::from_reader(BufReader::new(f))
                .expect("Could not create Xliff object");
            files.files.insert(
                directory
                    .to_str()
                    .expect("Could not convert directory to string")
                    .to_string(),
                xliff,
            );
        }
    }
    Ok(files)
}

fn main() {
    let files = from_source_dir("strings");
    println!("{:#?}", files);
}
