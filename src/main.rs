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
    let arguments = cli::Arguments::from_args();

    let files = get_files_in_directory(get_path(&arguments.from));
    let referenced_files = get_files_referenced_in_directory(&files, get_path(&arguments.search));

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
                    let content = read_to_string(path).unwrap();

                    for file in files {
                        let file_regex = Regex::new(&*file.display().to_string()).unwrap();

                        if file_regex.is_match(&content) {
                            files_referenced.insert(file.clone());
                        }
                    }
                } else {
                    files_referenced
                        .extend(get_files_referenced_in_directory(files, path.as_path()));
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    files_referenced
}

fn get_directory_entries(path: &Path) -> ReadDir {
    match read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            println!("{:?}", error);
            exit(ERROR_EXIT_CODE);
        }
    }
}

fn get_path(path: &str) -> &Path {
    Path::new(path)
}

fn get_files_in_directory(path: &Path) -> HashSet<PathBuf> {
    let mut files = HashSet::new();

    for dir_entry in get_directory_entries(path) {
        match dir_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    files.insert(path);
                } else {
                    files.extend(get_files_in_directory(path.as_path()));
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    files
}
