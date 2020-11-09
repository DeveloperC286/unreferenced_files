use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::exit;

use regex::Regex;

use crate::{file_content, file_utilities, regex_utilities};

pub fn print(from: &str, search: &str, search_for_relative_path: bool, search_for_file_name: bool) {
    let files = file_utilities::get_files_in_directory(file_utilities::get_path(from));
    let regex_map =
        regex_utilities::get_regex_map(&files, search_for_relative_path, search_for_file_name);
    let unreferenced_files = get_unreferenced_files_in_directory(
        &files,
        file_utilities::get_path(search),
        &regex_map,
        search_for_relative_path,
        search_for_file_name,
    );
    print_unreferenced_files(unreferenced_files);
}

fn get_unreferenced_files_in_directory(
    files: &HashSet<PathBuf>,
    path: &Path,
    regex_map: &HashMap<String, Regex>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
) -> HashSet<PathBuf> {
    let mut unreferenced_files = files.clone();

    for entry in file_utilities::get_directory_entries(path) {
        match entry {
            Ok(dir_entry) => {
                let searching = dir_entry.path();

                if searching.is_file() {
                    let file_content = file_utilities::get_file_content(&searching);
                    info!("Searching the file {:?}.", searching);

                    'files: for unreferenced_file in unreferenced_files.clone() {
                        if file_utilities::is_same_path(&unreferenced_file, &searching) {
                            warn!(
                                "Not searching {:?} for {:?} as they are the same file.",
                                searching, unreferenced_file
                            );
                            continue 'files;
                        }

                        if search_for_relative_path
                            && file_content::contains(
                                &file_content,
                                &file_utilities::get_relative_path(&*unreferenced_file),
                                &searching,
                                regex_map,
                            )
                        {
                            unreferenced_files.remove(&*unreferenced_file);
                            continue 'files;
                        }

                        if search_for_file_name
                            && file_content::contains(
                                &file_content,
                                file_utilities::get_file_name(&*unreferenced_file),
                                &searching,
                                regex_map,
                            )
                        {
                            unreferenced_files.remove(&*unreferenced_file);
                            continue 'files;
                        }
                    }
                } else {
                    let child_directories_unreferenced_files = get_unreferenced_files_in_directory(
                        &unreferenced_files,
                        searching.as_path(),
                        regex_map,
                        search_for_relative_path,
                        search_for_file_name,
                    );
                    unreferenced_files = unreferenced_files
                        .intersection(&child_directories_unreferenced_files)
                        .cloned()
                        .collect();
                }
            }
            Err(error) => {
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    unreferenced_files
}

fn print_unreferenced_files(unreferenced_files: HashSet<PathBuf>) {
    for unreferenced_file in unreferenced_files {
        println!("{}", unreferenced_file.display());
    }
}
