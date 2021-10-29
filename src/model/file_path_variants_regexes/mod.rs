use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::model::file_path_variants::FilePathVariants;

pub(crate) struct FilePathVariantsRegexes {
    file_path_variants_regexes: HashMap<String, Regex>,
}

impl FilePathVariantsRegexes {
    pub(crate) fn new(
        files: &HashSet<FilePathVariants>,
        search_for_relative_path: bool,
        search_for_file_name: bool,
        search_for_file_steam: bool,
    ) -> Result<FilePathVariantsRegexes, ()> {
        let mut file_path_variants_regexes = HashMap::new();

        for file in files {
            if search_for_relative_path {
                match Regex::new(&file.file_relative_path) {
                    Ok(relative_path_regex) => {
                        file_path_variants_regexes
                            .insert(file.file_relative_path.clone(), relative_path_regex);
                    }
                    Err(_) => {
                        error!(
                            "Unable to create a regex from the file relative path {:?}.",
                            file.file_relative_path
                        );
                        return Err(());
                    }
                }
            }

            if search_for_file_name {
                match Regex::new(&file.file_name) {
                    Ok(file_name_regex) => {
                        file_path_variants_regexes.insert(file.file_name.clone(), file_name_regex);
                    }
                    Err(_) => {
                        error!(
                            "Unable to create a regex from the file name {:?}.",
                            file.file_relative_path
                        );
                        return Err(());
                    }
                }
            }

            if search_for_file_steam {
                match Regex::new(&file.file_stem) {
                    Ok(file_stem_regex) => {
                        file_path_variants_regexes.insert(file.file_stem.clone(), file_stem_regex);
                    }
                    Err(_) => {
                        error!(
                            "Unable to create a regex from the file stem {:?}.",
                            file.file_relative_path
                        );
                        return Err(());
                    }
                }
            }
        }

        Ok(FilePathVariantsRegexes {
            file_path_variants_regexes,
        })
    }

    pub(crate) fn get(&self, file_path: &str) -> &Regex {
        self.file_path_variants_regexes.get(file_path).unwrap()
    }
}
