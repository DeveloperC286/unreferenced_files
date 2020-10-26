extern crate regex;

use std::collections::HashSet;
use std::fs::{read_dir, read_to_string};

use regex::Regex;
use structopt::StructOpt;

mod cli;

fn main() {
    let arguments = cli::Arguments::from_args();

    let files: HashSet<String> = get_files_in_directory(arguments.from);
    let referenced_files: HashSet<String> =
        get_files_referenced_in_directory(&files, arguments.search);

    for unreferenced_file in files.difference(&referenced_files) {
        println!("{}", unreferenced_file);
    }
}

fn get_files_referenced_in_directory(files: &HashSet<String>, path: String) -> HashSet<String> {
    let mut files_referenced = HashSet::new();

    for entry in read_dir(path).unwrap() {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    let content = read_to_string(path).unwrap();

                    for file in files {
                        let file_regex = Regex::new(file).unwrap();

                        if file_regex.is_match(&content) {
                            files_referenced.insert(file.to_string());
                        }
                    }
                } else {
                    files_referenced.extend(get_files_referenced_in_directory(
                        files,
                        path.display().to_string(),
                    ));
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    files_referenced
}

fn get_files_in_directory(path: String) -> HashSet<String> {
    let mut files = HashSet::new();

    for entry in read_dir(path).unwrap() {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    files.insert(path.display().to_string());
                } else {
                    files.extend(get_files_in_directory(path.display().to_string()));
                }
            }
            Err(error) => {
                println!("{:?}", error);
            }
        }
    }

    files
}