use std::collections::{HashMap, HashSet};

use regex::Regex;

use crate::model::file_path_variants::FilePathVariants;
use crate::utilities::regex::get_regex;

pub(crate) struct FilePathVariantsRegexes {
    file_path_variants_regexes: HashMap<String, Regex>,
}

impl FilePathVariantsRegexes {
    pub(crate) fn new(
        files: &HashSet<FilePathVariants>,
        search_for_relative_path: bool,
        search_for_file_name: bool,
        search_for_file_steam: bool,
    ) -> FilePathVariantsRegexes {
        let mut file_path_variants_regexes = HashMap::new();

        for file in files {
            if search_for_relative_path {
                let relative_path_regex = get_regex(&file.file_relative_path);
                file_path_variants_regexes
                    .insert(file.file_relative_path.clone(), relative_path_regex);
            }

            if search_for_file_name {
                let file_name_regex = get_regex(&file.file_name);
                file_path_variants_regexes.insert(file.file_name.clone(), file_name_regex);
            }

            if search_for_file_steam {
                let file_stem_regex = get_regex(&file.file_stem);
                file_path_variants_regexes.insert(file.file_stem.clone(), file_stem_regex);
            }
        }

        FilePathVariantsRegexes {
            file_path_variants_regexes,
        }
    }

    pub(crate) fn get(&self, file_path: &str) -> &Regex {
        self.file_path_variants_regexes.get(file_path).unwrap()
    }
}
