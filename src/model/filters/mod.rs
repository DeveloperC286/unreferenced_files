use std::process::exit;

use regex::Regex;

pub struct Filters {
    filters: Vec<Regex>,
    filtering_on: FilteringOn,
}

enum FilteringOn {
    Only,
    Ignore,
    None,
}

impl Filters {
    pub fn new(only_search: Vec<String>, ignore_search: Vec<String>) -> Self {
        return match (only_search.len(), ignore_search.len()) {
            (0, 0) => Filters {
                filters: vec![],
                filtering_on: FilteringOn::None,
            },
            (_, 0) => Filters {
                filters: only_search
                    .iter()
                    .map(|regex| crate::regex_utilities::get_regex(regex))
                    .collect(),
                filtering_on: FilteringOn::Only,
            },
            (0, _) => Filters {
                filters: ignore_search
                    .iter()
                    .map(|regex| crate::regex_utilities::get_regex(regex))
                    .collect(),
                filtering_on: FilteringOn::Ignore,
            },
            _ => {
                error!("Only and ignore filters are mutually exclusive.");
                exit(crate::ERROR_EXIT_CODE);
            }
        };
    }

    pub fn is_filtered_out(&self, file_canonicalize_path: &str) -> bool {
        fn matches_any(checking: &str, regexes: &[Regex]) -> bool {
            for regex in regexes {
                if regex.is_match(checking) {
                    return false;
                }
            }

            true
        }

        match self.filtering_on {
            FilteringOn::Only => matches_any(file_canonicalize_path, &self.filters),
            FilteringOn::Ignore => !matches_any(file_canonicalize_path, &self.filters),
            FilteringOn::None => false,
        }
    }
}
