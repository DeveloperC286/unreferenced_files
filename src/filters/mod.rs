use anyhow::{bail, Context, Result};
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
    pub fn new(only_search: Vec<String>, ignore_search: Vec<String>) -> Result<Filters> {
        fn to_regexes(to_regexes: Vec<String>) -> Result<Vec<Regex>> {
            let mut regexes = vec![];

            for to_regex in to_regexes {
                let regex = Regex::new(&to_regex)
                    .context(format!("Unable to convert {:?} to a regex.", to_regex))?;
                regexes.push(regex);
            }

            Ok(regexes)
        }

        match (only_search.len(), ignore_search.len()) {
            (0, 0) => Ok(Filters {
                filters: vec![],
                filtering_on: FilteringOn::None,
            }),
            (_, 0) => {
                let filters =
                    to_regexes(only_search).context("Unable to create only search filters.")?;
                Ok(Filters {
                    filters,
                    filtering_on: FilteringOn::Only,
                })
            }

            (0, _) => {
                let filters =
                    to_regexes(ignore_search).context("Unable to create ignore search filters.")?;
                Ok(Filters {
                    filters,
                    filtering_on: FilteringOn::Ignore,
                })
            }
            _ => {
                bail!("Only and ignore filters are mutually exclusive.");
            }
        }
    }

    pub(crate) fn should_ignore(&self, file_canonicalize_path: &str) -> bool {
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
