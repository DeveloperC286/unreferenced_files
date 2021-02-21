use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;

pub type FileContent = String;

#[derive(Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct RawFile {
    pub file_path_variants: FilePathVariants,
    pub file_content: FileContent,
}

impl RawFile {
    pub fn new(path: PathBuf) -> Option<RawFile> {
        if let Some(file_content) = crate::file_utilities::get_file_content(&path) {
            Some(RawFile {
                file_path_variants: crate::model::file_path_variants::FilePathVariants::new(path),
                file_content,
            })
        } else {
            None
        }
    }
}

pub fn get_raw_files(paths: Vec<PathBuf>, ignore_file_regexes: Vec<String>) -> HashSet<RawFile> {
    let compiled_ignore_file_regexes = crate::regex_utilities::get_regexes(ignore_file_regexes);
    let mut raw_files = HashSet::new();

    for path in paths {
        if path.is_dir() {
            raw_files.extend(get_raw_files_in_directory(
                &path,
                &compiled_ignore_file_regexes,
            ));
        } else if let Some(raw_file) =
            get_raw_files_in_file(path.to_path_buf(), &compiled_ignore_file_regexes)
        {
            raw_files.insert(raw_file);
        }
    }

    raw_files
}

fn get_raw_files_in_directory(path: &Path, ignore_file_regexes: &[Regex]) -> HashSet<RawFile> {
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
                    if let Some(raw_file) = get_raw_files_in_file(path, ignore_file_regexes) {
                        raw_files.insert(raw_file);
                    }
                } else {
                    raw_files.extend(get_raw_files_in_directory(
                        path.as_path(),
                        ignore_file_regexes,
                    ));
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

fn get_raw_files_in_file(path: PathBuf, ignore_file_regexes: &[Regex]) -> Option<RawFile> {
    if path.is_file() {
        if let Some(raw_file) = RawFile::new(path) {
            let file_canonicalize_path = &raw_file.file_path_variants.file_canonicalize_path;

            if crate::regex_utilities::does_not_match_any(
                &file_canonicalize_path,
                &*ignore_file_regexes,
            ) {
                trace!(
                    "Adding {:?} to the files searching.",
                    raw_file.file_path_variants.file_relative_path
                );
                return Some(raw_file);
            } else {
                debug!(
                    "Ignoring the file {:?} and not searching it.",
                    raw_file.file_path_variants.file_relative_path
                );
            }
        }
    } else {
        error!("{:?} is not a file.", path);
        exit(crate::ERROR_EXIT_CODE);
    }

    None
}
