use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::model::raw_file::RawFile;

pub struct RawFiles {
    pub raw_files: HashSet<RawFile>,
}

impl RawFiles {
    pub fn new(paths: Vec<PathBuf>) -> Self {
        fn get_raw_files_in_directory(path: &Path) -> HashSet<RawFile> {
            let mut raw_files = HashSet::new();
            trace!(
                "Searching the directory {:?} for files to search.",
                path.display()
            );

            for dir_entry in crate::file_utilities::get_directory_entries(path) {
                match dir_entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();

                        if path.is_file() {
                            if let Some(raw_file) = get_raw_files_in_file(path) {
                                raw_files.insert(raw_file);
                            }
                        } else {
                            raw_files.extend(get_raw_files_in_directory(path.as_path()));
                        }
                    }
                    Err(error) => {
                        error!("{:?}", error);
                        exit(crate::ERROR_EXIT_CODE);
                    }
                }
            }

            raw_files
        }

        fn get_raw_files_in_file(path: PathBuf) -> Option<RawFile> {
            if path.is_file() {
                if let Some(raw_file) = RawFile::new(path) {
                    trace!(
                        "Adding {:?} to the files searching.",
                        raw_file.file_path_variants.file_relative_path
                    );
                    return Some(raw_file);
                }
            } else {
                error!("{:?} is not a file.", path);
                exit(crate::ERROR_EXIT_CODE);
            }

            None
        }

        let mut raw_files = HashSet::new();

        for path in paths {
            if path.is_dir() {
                raw_files.extend(get_raw_files_in_directory(&path));
            } else if let Some(raw_file) = get_raw_files_in_file(path.to_path_buf()) {
                raw_files.insert(raw_file);
            }
        }

        RawFiles { raw_files }
    }
}
