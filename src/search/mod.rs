use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

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
    pub fn new<T: AsRef<str>>(paths: &[T], filters: Filters) -> Result<Search> {
        fn get_raw_files_in_directory(path: &Path, filters: &Filters) -> Result<HashSet<RawFile>> {
            let mut raw_files = HashSet::new();
            debug!(
                "Searching the directory {:?} for files to search.",
                path.display()
            );

            let entries = std::fs::read_dir(path)?;

            for dir_entry in entries {
                let dir_entry = dir_entry?;
                let path = dir_entry.path();

                if path.is_file() {
                    if let Ok(raw_file) = get_raw_file(path, filters) {
                        raw_files.insert(raw_file);
                    }
                } else {
                    raw_files.extend(get_raw_files_in_directory(path.as_path(), filters)?);
                }
            }
            Ok(raw_files)
        }

        fn get_raw_file(path: PathBuf, filters: &Filters) -> Result<RawFile> {
            if path.is_file() {
                let raw_file = RawFile::new(path)?;

                if filters.should_ignore(&raw_file.file_path_variants.file_canonicalize_path) {
                    debug!("Ignoring the file {raw_file:?} and not searching it.");
                } else {
                    debug!("Adding {raw_file:?} to the files searching.");
                    return Ok(raw_file);
                }
            }

            bail!("Unable to get raw file.");
        }

        let mut raw_files = HashSet::new();
        let pathbufs = crate::utilities::to_pathbufs(paths)?;

        for pathbuf in pathbufs {
            if pathbuf.is_dir() {
                raw_files.extend(get_raw_files_in_directory(&pathbuf, &filters)?);
            } else if let Ok(raw_file) = get_raw_file(pathbuf, &filters) {
                raw_files.insert(raw_file);
            }
        }

        info!(
            "Loaded {} files to search within for references.",
            raw_files.len()
        );
        Ok(Search { raw_files })
    }
}
