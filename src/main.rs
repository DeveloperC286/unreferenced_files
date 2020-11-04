#[macro_use]
extern crate log;
extern crate regex;

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

use regex::Regex;
use structopt::StructOpt;

mod cli;
mod file_utilities;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    env_logger::init();
    let arguments = cli::Arguments::from_args();
    info!("{:?}", arguments);

    let files = get_files_in_directory(file_utilities::get_path(&arguments.from));
    let referenced_files = get_files_referenced_in_directory(&files, file_utilities::get_path(&arguments.search));
    print_unreferenced_files(files, &referenced_files);
}

fn print_unreferenced_files(files: HashSet<PathBuf>, referenced_files: &HashSet<PathBuf>) {
    for unreferenced_file in files.difference(referenced_files) {
        println!("{}", unreferenced_file.display());
    }
}

fn get_files_referenced_in_directory(files: &HashSet<PathBuf>, path: &Path) -> HashSet<PathBuf> {
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
                        if file_content_contains(
                            &file_content,
                            &file_utilities::get_relative_path(file),
                            &file_searching,
                        ) {
                            files_referenced.insert(file.clone());
                            continue 'files;
                        }

                        if file_content_contains(
                            &file_content,
                            file_utilities::get_file_name(file),
                            &file_searching,
                        ) {
                            files_referenced.insert(file.clone());
                            continue 'files;
                        }
                    }
                } else {
                    files_referenced
                        .extend(get_files_referenced_in_directory(files, path.as_path()));
                }
            }
            Err(error) => {
                error!("{:?}", error);
                exit(ERROR_EXIT_CODE);
            }
        }
    }

    files_referenced
}

fn file_content_contains(
    file_content: &str,
    text_searching_for: &str,
    file_searching: &str,
) -> bool {
    match get_regex(text_searching_for).is_match(file_content) {
        true => {
            trace!(
                "Found the text {:?} inside the file {:?}.",
                text_searching_for,
                file_searching
            );
            true
        }
        false => false,
    }
}

fn get_regex(text_to_find: &str) -> Regex {
    match Regex::new(text_to_find) {
        Ok(reg) => reg,
        Err(error) => {
            error!("{:?}", error);
            exit(ERROR_EXIT_CODE);
        }
    }
}

fn get_files_in_directory(path: &Path) -> HashSet<PathBuf> {
    let mut files = HashSet::new();

    info!("Searching the directory {:?} for files.", path.display());
    for dir_entry in file_utilities::get_directory_entries(path) {
        match dir_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    trace!("Adding the file {:?} to the found files.", path.display());
                    files.insert(path);
                } else {
                    files.extend(get_files_in_directory(path.as_path()));
                }
            }
            Err(error) => {
                error!("{:?}", error);
                exit(ERROR_EXIT_CODE);
            }
        }
    }

    files
}
