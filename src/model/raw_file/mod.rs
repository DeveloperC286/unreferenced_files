use std::path::PathBuf;

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;

pub type FileContent = String;

#[derive(Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct RawFile {
    pub file_path_variants: FilePathVariants,
    file_content: FileContent,
}

impl RawFile {
    pub fn new(path: PathBuf) -> Option<Self> {
        crate::utilities::file::get_file_content(&path).map(|file_content| RawFile {
            file_path_variants: crate::model::file_path_variants::FilePathVariants::new(path),
            file_content,
        })
    }

    pub fn is_match(&self, regex: &Regex) -> bool {
        regex.is_match(&self.file_content)
    }
}
