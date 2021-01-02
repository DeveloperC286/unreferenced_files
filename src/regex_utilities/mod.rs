use std::collections::{HashMap, HashSet};
use std::process::exit;

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;
use crate::model::raw_file::RawFile;

pub fn get_regex_map(
    files: &HashSet<FilePathVariants>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_steam: bool,
) -> HashMap<String, Regex> {
    let mut regex_map = HashMap::new();

    for file in files {
        if search_for_relative_path {
            let relative_path_regex = get_regex(&file.file_relative_path);
            regex_map.insert(file.file_relative_path.clone(), relative_path_regex);
        }

        if search_for_file_name {
            let file_name_regex = get_regex(&file.file_name);
            regex_map.insert(file.file_name.clone(), file_name_regex);
        }

        if search_for_file_steam {
            let file_stem_regex = get_regex(&file.file_stem);
            regex_map.insert(file.file_stem.clone(), file_stem_regex);
        }
    }

    regex_map
}

fn get_regex(text_to_find: &str) -> Regex {
    match Regex::new(text_to_find) {
        Ok(reg) => reg,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

pub fn contains(
    searching: &RawFile,
    text_searching_for: &str,
    regex_map: &HashMap<String, Regex>,
) -> bool {
    match regex_map
        .get(text_searching_for)
        .unwrap()
        .is_match(&searching.file_content)
    {
        true => {
            trace!(
                "Found the text {:?} inside the file {:?}.",
                text_searching_for,
                searching.file_path_variants.file_relative_path
            );
            true
        }
        false => false,
    }
}
