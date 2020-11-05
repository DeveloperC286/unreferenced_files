use std::collections::HashMap;

use regex::Regex;

pub fn contains(file_content: &str, text_searching_for: &str, file_searching: &str, regex_map: &HashMap<String, Regex>) -> bool {
    match regex_map.get(text_searching_for).unwrap().is_match(file_content) {
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