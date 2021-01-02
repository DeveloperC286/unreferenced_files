use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::model::file_path_variants::FilePathVariants;

pub type FileContent = String;

#[derive(Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct RawFile {
    pub file_path_variants: FilePathVariants,
    pub file_content: FileContent,
}

impl RawFile {
    pub fn new(path: PathBuf) -> Self {
        let file_content = crate::file_utilities::get_file_content(&path);
        RawFile {
            file_path_variants: crate::model::file_path_variants::FilePathVariants::new(path),
            file_content,
        }
    }
}

pub fn get_raw_files_in_directory(path: &Path) -> HashSet<RawFile> {
    let mut files = HashSet::new();
    trace!("Searching the directory {:?} for files.", path.display());
    for dir_entry in crate::file_utilities::get_directory_entries(path) {
        match dir_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    info!("Adding the file {:?} to the found files.", path.display());
                    files.insert(RawFile::new(path));
                } else {
                    files.extend(get_raw_files_in_directory(path.as_path()));
                }
            }
            Err(error) => {
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    files
}
