use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;
use crate::model::raw_file::RawFile;

pub fn get_unreferenced_files(
    search_for: HashSet<FilePathVariants>,
    searching: HashSet<RawFile>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_stem: bool,
) -> HashSet<FilePathVariants> {
    let searching_for_regex_map = crate::regex_utilities::get_regex_map(
        &search_for,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    get_unreferenced_files_in_directory(
        search_for,
        searching_for_regex_map,
        searching,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    )
}

fn get_unreferenced_files_in_directory(
    mut search_for: HashSet<FilePathVariants>,
    search_for_regex_map: HashMap<String, Regex>,
    searching: HashSet<RawFile>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_stem: bool,
) -> HashSet<FilePathVariants> {
    for raw_file in searching {
        if search_for.is_empty() {
            return search_for;
        }

        info!(
            "Searching the file {:?}.",
            raw_file.file_path_variants.file_relative_path
        );

        search_for.retain(|unreferenced_file| {
            if unreferenced_file.file_canonicalize_path
                == raw_file.file_path_variants.file_canonicalize_path
            {
                warn!(
                    "Not searching {:?} for {:?} as they are the same file.",
                    raw_file.file_path_variants.file_relative_path,
                    unreferenced_file.file_relative_path
                );
                return true;
            }

            if search_for_relative_path
                && crate::regex_utilities::contains(
                    &raw_file,
                    &unreferenced_file.file_relative_path,
                    &search_for_regex_map,
                )
            {
                return false;
            }

            if search_for_file_name
                && crate::regex_utilities::contains(
                    &raw_file,
                    &unreferenced_file.file_name,
                    &search_for_regex_map,
                )
            {
                return false;
            }

            if search_for_file_stem
                && crate::regex_utilities::contains(
                    &raw_file,
                    &unreferenced_file.file_stem,
                    &search_for_regex_map,
                )
            {
                return false;
            }

            true
        });
    }

    search_for
}

#[cfg(test)]
#[macro_use]
mod tests;
