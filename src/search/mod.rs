use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::filters::Filters;
use crate::search::raw_file::RawFile;

#[cfg(not(test))]
mod raw_file;

#[cfg(test)]
pub(crate) mod raw_file;

pub struct Search {
    pub(crate) raw_files: HashSet<RawFile>,
}

impl Search {
    pub fn new<T: AsRef<str>>(paths: &[T], filters: Filters) -> Result<Search, ()> {
        fn get_raw_files_in_directory(
            path: &Path,
            filters: &Filters,
        ) -> Result<HashSet<RawFile>, ()> {
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
                                    if let Ok(raw_file) = get_raw_file(path, filters) {
                                        raw_files.insert(raw_file);
                                    }
                                } else {
                                    raw_files.extend(get_raw_files_in_directory(
                                        path.as_path(),
                                        filters,
                                    )?);
                                }
                            }
                            Err(error) => {
                                error!("{:?}", error);
                            }
                        }
                    }
                }
                Err(_) => {
                    error!(
                        "Unable to read the directory entries for the path {:?}.",
                        path
                    );
                    return Err(());
                }
            }
            Ok(raw_files)
        }

        fn get_raw_file(path: PathBuf, filters: &Filters) -> Result<RawFile, ()> {
            if path.is_file() {
                if let Ok(raw_file) = RawFile::new(path) {
                    if filters.should_ignore(&raw_file.file_path_variants.file_canonicalize_path) {
                        debug!("Ignoring the file {:?} and not searching it.", raw_file);
                    } else {
                        trace!("Adding {:?} to the files searching.", raw_file);
                        return Ok(raw_file);
                    }
                }
            } else {
                error!("{:?} is not a file.", path);
            }

            Err(())
        }

        let mut raw_files = HashSet::new();

        match crate::utilities::to_pathbufs(paths) {
            Ok(pathbufs) => {
                for pathbuf in pathbufs {
                    if pathbuf.is_dir() {
                        raw_files.extend(get_raw_files_in_directory(&pathbuf, &filters)?);
                    } else if let Ok(raw_file) = get_raw_file(pathbuf, &filters) {
                        raw_files.insert(raw_file);
                    }
                }
            }
            Err(_) => {
                return Err(());
            }
        }

        Ok(Search { raw_files })
    }
}
