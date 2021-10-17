use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::model::filters::Filters;
use crate::model::search::raw_file::RawFile;

#[cfg(not(test))]
mod raw_file;

#[cfg(test)]
pub(crate) mod raw_file;

pub(crate) struct Search {
    pub(crate) raw_files: HashSet<RawFile>,
}

impl Search {
    pub(crate) fn new(paths: Vec<PathBuf>, filters: Filters) -> Self {
        fn get_raw_files_in_directory(path: &Path, filters: &Filters) -> HashSet<RawFile> {
            let mut raw_files = HashSet::new();
            trace!(
                "Searching the directory {:?} for files to search.",
                path.display()
            );

            match std::fs::read_dir(path) {
                Ok(entries) => {
                    for dir_entry in entries {
                        match dir_entry {
                            Ok(dir_entry) => {
                                let path = dir_entry.path();

                                if path.is_file() {
                                    if let Some(raw_file) = get_raw_file(path, filters) {
                                        raw_files.insert(raw_file);
                                    }
                                } else {
                                    raw_files.extend(get_raw_files_in_directory(
                                        path.as_path(),
                                        filters,
                                    ));
                                }
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                exit(crate::ERROR_EXIT_CODE);
                            }
                        }
                    }
                }
                Err(_) => {
                    error!(
                        "Unable to read the directory entries for the path {:?}.",
                        path
                    );
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
            raw_files
        }

        fn get_raw_file(path: PathBuf, filters: &Filters) -> Option<RawFile> {
            if path.is_file() {
                if let Some(raw_file) = RawFile::new(path) {
                    if filters.should_ignore(&raw_file.file_path_variants.file_canonicalize_path) {
                        debug!("Ignoring the file {:?} and not searching it.", raw_file);
                    } else {
                        trace!("Adding {:?} to the files searching.", raw_file);
                        return Some(raw_file);
                    }
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
                raw_files.extend(get_raw_files_in_directory(&path, &filters));
            } else if let Some(raw_file) = get_raw_file(path.to_path_buf(), &filters) {
                raw_files.insert(raw_file);
            }
        }

        Search { raw_files }
    }
}
