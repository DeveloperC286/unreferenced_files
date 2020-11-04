use std::process::exit;

use regex::Regex;

pub fn contains(file_content: &str, text_searching_for: &str, file_searching: &str) -> bool {
    match get_regex(text_searching_for).is_match(file_content) {
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

fn get_regex(text_to_find: &str) -> Regex {
    match Regex::new(text_to_find) {
        Ok(reg) => reg,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}
