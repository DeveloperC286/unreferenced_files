use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

pub fn get_path(path: &str) -> &Path {
    let path = Path::new(path);

    if !path.exists() {
        error!("{:?} does not exist.", path);
        exit(crate::ERROR_EXIT_CODE);
    }

    if !path.is_dir() {
        error!("{:?} is not a directory.", path);
        exit(crate::ERROR_EXIT_CODE);
    }

    path
}

pub fn get_file_name(path: &Path) -> &str {
    match path.file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(str) => str,
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

pub fn get_relative_path(path: &Path) -> String {
    path.display()
        .to_string()
        .trim_start_matches("./")
        .to_string()
}

pub fn get_file_content(path: &Path) -> String {
    match std::fs::read_to_string(path) {
        Ok(file_content) => file_content,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

pub fn get_directory_entries(path: &Path) -> std::fs::ReadDir {
    match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

pub fn get_files_in_directory(path: &Path) -> HashSet<PathBuf> {
    let mut files = HashSet::new();

    trace!("Searching the directory {:?} for files.", path.display());
    for dir_entry in get_directory_entries(path) {
        match dir_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    info!("Adding the file {:?} to the found files.", path.display());
                    files.insert(path);
                } else {
                    files.extend(get_files_in_directory(path.as_path()));
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

pub fn is_same_path(path: &Path, path2: &Path) -> bool {
    match path.canonicalize() {
        Ok(canonicalized_path) => match path2.canonicalize() {
            Ok(canonicalized_path2) => {
                return canonicalized_path == canonicalized_path2;
            }
            Err(_) => {
                error!("Unable to canonicalize {:?}.", path2);
            }
        },
        Err(_) => {
            error!("Unable to canonicalize {:?}.", path);
        }
    }

    false
}
