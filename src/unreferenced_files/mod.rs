use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;
use crate::model::raw_file::RawFile;

pub fn get_unreferenced_files(
    searching_for: HashSet<FilePathVariants>,
    searching: HashSet<RawFile>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_stem: bool,
) -> HashSet<FilePathVariants> {
    let searching_for_regex_map = crate::regex_utilities::get_regex_map(
        &searching_for,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    get_unreferenced_files_in_directory(
        searching_for,
        searching_for_regex_map,
        searching,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    )
}

fn get_unreferenced_files_in_directory(
    mut searching_for: HashSet<FilePathVariants>,
    searching_for_regex_map: HashMap<String, Regex>,
    searching: HashSet<RawFile>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_stem: bool,
) -> HashSet<FilePathVariants> {
    for raw_file in searching {
        if searching_for.is_empty() {
            return searching_for;
        }

        info!(
            "Searching the file {:?}.",
            raw_file.file_path_variants.file_relative_path
        );

        'files: for unreferenced_file in searching_for.clone() {
            if unreferenced_file.file_canonicalize_path
                == raw_file.file_path_variants.file_canonicalize_path
            {
                warn!(
                    "Not searching {:?} for {:?} as they are the same file.",
                    raw_file.file_path_variants.file_relative_path,
                    unreferenced_file.file_relative_path
                );
                continue 'files;
            }

            if search_for_relative_path
                && crate::regex_utilities::contains(
                    &raw_file,
                    &unreferenced_file.file_relative_path,
                    &searching_for_regex_map,
                )
            {
                searching_for.remove(&unreferenced_file);
                continue 'files;
            }

            if search_for_file_name
                && crate::regex_utilities::contains(
                    &raw_file,
                    &unreferenced_file.file_name,
                    &searching_for_regex_map,
                )
            {
                searching_for.remove(&unreferenced_file);
                continue 'files;
            }

            if search_for_file_stem
                && crate::regex_utilities::contains(
                    &raw_file,
                    &unreferenced_file.file_stem,
                    &searching_for_regex_map,
                )
            {
                searching_for.remove(&unreferenced_file);
                continue 'files;
            }
        }
    }

    searching_for
}

#[cfg(test)]
#[macro_use]
mod tests;
