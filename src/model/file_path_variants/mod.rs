use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Hash, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct FilePathVariants {
    pub file_canonicalize_path: String,
    pub file_relative_path: String,
    pub file_name: String,
    pub file_stem: String,
}

impl FilePathVariants {
    pub fn new(path: PathBuf) -> Self {
        FilePathVariants {
            file_canonicalize_path: get_file_canonicalize_path(&path),
            file_relative_path: get_file_relative_path(&path),
            file_name: get_file_name(&path),
            file_stem: get_file_stem(&path),
        }
    }
}

fn get_file_canonicalize_path(path: &Path) -> String {
    match path.canonicalize() {
        Ok(canonicalized_path) => canonicalized_path.display().to_string(),
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_file_relative_path(path: &Path) -> String {
    path.display()
        .to_string()
        .trim_start_matches("./")
        .to_string()
}

fn get_file_name(path: &Path) -> String {
    match path.file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(str) => str.to_string(),
            None => {
                error!("Can not convert {:?} to str.", file_name);
                exit(crate::ERROR_EXIT_CODE);
            }
        },
        None => {
            error!("{:?} has no file name.", path.display());
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_file_stem(path: &Path) -> String {
    match path.file_stem() {
        Some(file_stem) => match file_stem.to_str() {
            Some(str) => str.to_string(),
            None => {
                error!("Can not convert {:?} to str.", file_stem);
                exit(crate::ERROR_EXIT_CODE);
            }
        },
        None => {
            error!("{:?} has no file steam.", path.display());
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

pub fn get_file_path_variants(paths: Vec<PathBuf>) -> HashSet<FilePathVariants> {
    let mut files_path_variants = HashSet::new();

    for path in paths {
        if path.is_dir() {
            files_path_variants.extend(get_file_path_variants_in_directory(&path));
        } else if let Some(file_path_variants) = get_file_path_variants_in_file(path.to_path_buf())
        {
            files_path_variants.insert(file_path_variants);
        }
    }

    files_path_variants
}

fn get_file_path_variants_in_directory(path: &Path) -> HashSet<FilePathVariants> {
    let mut files_path_variants = HashSet::new();
    trace!(
        "Searching the directory {:?} for files to search for.",
        path.display()
    );

    for dir_entry in crate::file_utilities::get_directory_entries(path) {
        match dir_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    if let Some(file_path_variants) = get_file_path_variants_in_file(path) {
                        files_path_variants.insert(file_path_variants);
                    }
                } else {
                    files_path_variants.extend(get_file_path_variants_in_directory(path.as_path()));
                }
            }
            Err(error) => {
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    files_path_variants
}

fn get_file_path_variants_in_file(path: PathBuf) -> Option<FilePathVariants> {
    if path.is_file() {
        trace!("Adding {:?} to the files searching for.", path.display());
        return Some(FilePathVariants::new(path));
    } else {
        error!("{:?} is not a file.", path);
        exit(crate::ERROR_EXIT_CODE);
    }
}
