use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::exit;

use regex::Regex;

use crate::{file_content, file_utilities, regex_utilities};

pub fn print(from: &str, search: &str) {
    let files = file_utilities::get_files_in_directory(file_utilities::get_path(from));
    let regex_map = regex_utilities::get_regex_map(&files);
    let referenced_files =
        get_files_referenced_in_directory(&files, file_utilities::get_path(search), &regex_map);
    print_unreferenced_files(files, &referenced_files);
}

fn get_files_referenced_in_directory(files: &HashSet<PathBuf>, path: &Path, regex_map: &HashMap<String, Regex>) -> HashSet<PathBuf> {
    let mut files_referenced = HashSet::new();

    for entry in file_utilities::get_directory_entries(path) {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    let file_searching = path.display().to_string();
                    let file_content = file_utilities::get_file_content(&path);
                    info!("Searching the file {:?}.", file_searching);

                    'files: for file in files {
                        if file_content::contains(
                            &file_content,
                            &file_utilities::get_relative_path(file),
                            &file_searching,
                            regex_map,
                        ) {
                            files_referenced.insert(file.clone());
                            continue 'files;
                        }

                        if file_content::contains(
                            &file_content,
                            file_utilities::get_file_name(file),
                            &file_searching,
                            regex_map,
                        ) {
                            files_referenced.insert(file.clone());
                            continue 'files;
                        }
                    }
                } else {
                    files_referenced
                        .extend(get_files_referenced_in_directory(files, path.as_path(), regex_map));
                }
            }
            Err(error) => {
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    files_referenced
}

fn print_unreferenced_files(files: HashSet<PathBuf>, referenced_files: &HashSet<PathBuf>) {
    for unreferenced_file in files.difference(referenced_files) {
        println!("{}", unreferenced_file.display());
    }
}
