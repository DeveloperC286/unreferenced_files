use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

pub fn contains(
    file_content: &str,
    text_searching_for: &str,
    searching: &Path,
    regex_map: &HashMap<String, Regex>,
) -> bool {
    match regex_map
        .get(text_searching_for)
        .unwrap()
        .is_match(file_content)
    {
        true => {
            trace!(
                "Found the text {:?} inside the file {:?}.",
                text_searching_for,
                searching
            );
            true
        }
        false => false,
    }
}
