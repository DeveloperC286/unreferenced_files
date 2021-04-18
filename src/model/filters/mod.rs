use regex::Regex;

pub struct Filters {
    filters: Vec<Regex>,
    filtering_on: FilteringOn,
}

enum FilteringOn {
    Only,
    None,
}

impl Filters {
    pub fn new(only_search: Vec<String>) -> Self {
        if !only_search.is_empty() {
            Filters {
                filters: {
                    only_search
                        .iter()
                        .map(|regex| crate::regex_utilities::get_regex(regex))
                        .collect()
                },
                filtering_on: FilteringOn::Only,
            }
        } else {
            Filters {
                filters: vec![],
                filtering_on: FilteringOn::None,
            }
        }
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
            FilteringOn::None => false,
        }
    }
}
