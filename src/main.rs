#[macro_use]
extern crate log;
extern crate regex;

use std::collections::HashSet;
use std::fs::{read_dir, read_to_string, ReadDir};
use std::path::{Path, PathBuf};
use std::process::exit;

use regex::Regex;
use structopt::StructOpt;

mod cli;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    env_logger::init();
    let arguments = cli::Arguments::from_args();
    info!("{:?}", arguments);

    let files = get_files_in_directory(get_path(&arguments.from));
    let referenced_files = get_files_referenced_in_directory(&files, get_path(&arguments.search));
    print_unreferenced_files(files, referenced_files);
}

fn print_unreferenced_files(files: HashSet<PathBuf>, referenced_files: HashSet<PathBuf>) {
    for unreferenced_file in files.difference(&referenced_files) {
        println!("{}", unreferenced_file.display());
    }
}

fn get_files_referenced_in_directory(files: &HashSet<PathBuf>, path: &Path) -> HashSet<PathBuf> {
    let mut files_referenced = HashSet::new();

    for entry in read_dir(path).unwrap() {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    let searching = path.display().to_string();
                    info!("Searching the file '{}'.", searching);
                    let content = read_to_string(path).unwrap();

                    for file in files {
                        let searching_for = file.display().to_string();

                        if searching_for.len() > 2 {
                            let trimmed_searching_for = &searching_for[2..];
                            let file_regex = Regex::new(trimmed_searching_for).unwrap();

                            if file_regex.is_match(&content) {
                                trace!(
                                    "Found the text '{}' inside the file '{}'.",
                                    trimmed_searching_for,
                                    searching
                                );
                                files_referenced.insert(file.clone());
                            }
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

fn get_directory_entries(path: &Path) -> ReadDir {
    match read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            error!("{:?}", error);
            exit(ERROR_EXIT_CODE);
        }
    }
}

fn get_path(path: &str) -> &Path {
    let path = Path::new(path);

    if !path.exists() {
        error!("{:?} does not exist.", path);
        exit(ERROR_EXIT_CODE);
    }

    if !path.is_dir() {
        error!("{:?} is not a directory.", path);
        exit(ERROR_EXIT_CODE);
    }

    path
}

fn get_files_in_directory(path: &Path) -> HashSet<PathBuf> {
    let mut files = HashSet::new();

    info!("Searching the directory '{}' for files.", path.display());
    for dir_entry in get_directory_entries(path) {
        match dir_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    trace!("Adding the file '{}' to the found files.", path.display());
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
