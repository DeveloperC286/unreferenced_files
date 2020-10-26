extern crate regex;

use std::collections::HashSet;

use regex::Regex;
use structopt::StructOpt;

mod cli;

fn main() {
    let arguments = cli::Arguments::from_args();

    let files: HashSet<String> = get_all_files_in(arguments.from);
    let files_referenced: HashSet<String> = get_all_files_referenced_in(&files, arguments.search);

    // Now diff files and files_referenced.
    for file_unreferenced in files.difference(&files_referenced) {
        println!("{}", file_unreferenced);
    }
}

fn get_all_files_referenced_in(files: &HashSet<String>, path: String) -> HashSet<String> {
    let mut files_referenced = HashSet::new();

    for entry in std::fs::read_dir(path).unwrap() {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    let content = std::fs::read_to_string(path).unwrap();

                    for file in files {
                        let file_regex = Regex::new(file).unwrap();

                        if file_regex.is_match(&content) {
                            files_referenced.insert(file.to_string());
                        }
                    }
                } else {
                    files_referenced.extend(get_all_files_referenced_in(files, path.display().to_string()));
                }
            }
            Err(error) => {
                println!("{:?}", error);
                std::process::exit(1);
            }
        }
    }

    return files_referenced;
}

fn get_all_files_in(path: String) -> HashSet<String> {
    let mut files = HashSet::new();

    for entry in std::fs::read_dir(path).unwrap() {
        match entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();

                if path.is_file() {
                    files.insert(path.display().to_string());
                } else {
                    files.extend(get_all_files_in(path.display().to_string()));
                }
            }
            Err(error) => {
                println!("{:?}", error);
                std::process::exit(1);
            }
        }
    }

    return files;
}