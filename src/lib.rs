mod xliff;

use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{fs::File as StdFile, io::BufReader};
use thiserror::Error;
use xliff::Xliff;

#[derive(Debug, Error)]
pub enum XliffError {
    #[error("path {0:?} contained invalid utf-8")]
    InvalidUtf8(PathBuf),
    #[error("Could not open file at {path:?}: {source:?}")]
    CouldNotOpenFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("could not read dir at {path:?}: {source:?}")]
    CouldNotReadDir {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error(transparent)]
    DeserializeError(#[from] serde_xml_rs::Error),
    #[error("directory {0:?} contained invalid utf-8")]
    InvalidFileName(OsString),
    #[error("invalid DirEntry at {path:?}: {source:?}")]
    InvalidDirEntry {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("expected directory at path: {0:?}")]
    FileIsNotDirectory(PathBuf),
}

#[derive(Debug, Default)]
pub struct StringFiles {
    pub files: HashMap<String, Xliff>,
}

impl StringFiles {
    pub fn from_source_dir(path: impl AsRef<Path>) -> Result<Self, XliffError> {
        let path = path.as_ref();
        let mut files = StringFiles::default();
        for localization in
            std::fs::read_dir(&path).map_err(|source| XliffError::CouldNotReadDir {
                path: path.to_owned(),
                source,
            })?
        {
            let entry = localization.map_err(|source| XliffError::InvalidDirEntry {
                path: path.to_owned(),
                source,
            })?;
            entry
                .file_type()
                .unwrap()
                .is_dir()
                .then(|| ())
                .ok_or_else(|| XliffError::FileIsNotDirectory(path.to_owned()))?;

            let directory = entry.file_name();
            let file_path = path.join(&directory).join("strings.xliff");
            let f = StdFile::open(&file_path).map_err(|source| XliffError::CouldNotOpenFile {
                path: file_path,
                source,
            })?;
            let xliff: Xliff = serde_xml_rs::de::from_reader(BufReader::new(f))?;
            files.files.insert(
                directory
                    .to_str()
                    .ok_or_else(|| XliffError::InvalidFileName(directory.clone()))?
                    .to_string(),
                xliff,
            );
        }
        Ok(files)
    }
}
