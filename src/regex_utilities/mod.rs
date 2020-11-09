use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::process::exit;

use regex::Regex;

use crate::file_utilities;

pub fn get_regex_map(
    files: &HashSet<PathBuf>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_steam: bool,
) -> HashMap<String, Regex> {
    let mut regex_map = HashMap::new();

    for file in files {
        if search_for_relative_path {
            let relative_path = file_utilities::get_relative_path(file);
            let relative_path_regex = get_regex(&relative_path);
            regex_map.insert(relative_path, relative_path_regex);
        }

        if search_for_file_name {
            let file_name = file_utilities::get_file_name(file).to_string();
            let file_name_regex = get_regex(&file_name);
            regex_map.insert(file_name, file_name_regex);
        }

        if search_for_file_steam {
            let file_stem = file_utilities::get_file_stem(file).to_string();
            let file_stem_regex = get_regex(&file_stem);
            regex_map.insert(file_stem, file_stem_regex);
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
