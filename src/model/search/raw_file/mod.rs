use std::path::PathBuf;

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;

#[cfg(not(test))]
#[derive(Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct RawFile {
    pub(crate) file_path_variants: FilePathVariants,
    file_content: String,
}

// For unit testing.
#[cfg(test)]
#[derive(Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct RawFile {
    pub(crate) file_path_variants: FilePathVariants,
    pub(crate) file_content: String,
}

impl RawFile {
    pub(crate) fn new(path: PathBuf) -> Option<Self> {
        match std::fs::read_to_string(&path) {
            Ok(file_content) => Some(RawFile {
                file_path_variants: crate::model::file_path_variants::FilePathVariants::new(path),
                file_content,
            }),
            Err(error) => {
                warn!(
                    "Encountered {} while trying to read the file {:?}.",
                    error,
                    path.display()
                );
                None
            }
        }
    }

    pub(crate) fn is_match(&self, regex: &Regex) -> bool {
        regex.is_match(&self.file_content)
    }
}

impl std::fmt::Debug for RawFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.file_path_variants.file_relative_path)
    }
}
