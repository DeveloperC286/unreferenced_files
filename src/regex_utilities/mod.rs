use std::process::exit;

use regex::Regex;

pub fn get_regex(regex_string: &str) -> Regex {
    match Regex::new(regex_string) {
        Ok(regex) => regex,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}
