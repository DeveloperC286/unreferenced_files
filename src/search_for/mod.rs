use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::file_path_variants::FilePathVariants;
use crate::file_path_variants_regexes::FilePathVariantsRegexes;
use crate::filters::Filters;
use crate::search::Search;

#[cfg(not(test))]
pub struct SearchFor {
    search_for: HashSet<FilePathVariants>,
}

// For unit testing.
#[cfg(test)]
#[derive(Clone)]
pub struct SearchFor {
    pub(crate) search_for: HashSet<FilePathVariants>,
}

impl SearchFor {
    pub fn new<T: AsRef<str>>(paths: &[T], filters: Filters) -> Result<SearchFor> {
        fn get_file_path_variants_in_directory(
            path: &Path,
            filters: &Filters,
        ) -> Result<HashSet<FilePathVariants>> {
            let mut files_path_variants = HashSet::new();
            debug!(
                "Searching the directory {:?} for files to search for.",
                path.display()
            );

            let entries = std::fs::read_dir(path)?;
            for dir_entry in entries {
                let dir_entry = dir_entry?;
                let path = dir_entry.path();

                if path.is_file() {
                    if let Some(file_path_variants) = get_file_path_variants(path, filters) {
                        files_path_variants.insert(file_path_variants);
                    }
                } else {
                    files_path_variants.extend(get_file_path_variants_in_directory(
                        path.as_path(),
                        filters,
                    )?);
                }
            }

            Ok(files_path_variants)
        }

        fn get_file_path_variants(path: PathBuf, filters: &Filters) -> Option<FilePathVariants> {
            let file_path_variants = FilePathVariants::new(path).unwrap();

            if filters.should_ignore(&file_path_variants.file_canonicalize_path) {
                debug!(
                    "Ignoring the file {:?} and not searching for it.",
                    file_path_variants.file_relative_path
                );
            } else {
                debug!(
                    "Adding {:?} to the files searching for.",
                    file_path_variants.file_relative_path
                );
                return Some(file_path_variants);
            }

            None
        }

        let mut search_for = HashSet::new();

        let pathbufs = crate::utilities::to_pathbufs(paths)?;
        for pathbuf in pathbufs {
            if pathbuf.is_file() {
                if let Some(file_path_variants) = get_file_path_variants(pathbuf, &filters) {
                    search_for.insert(file_path_variants);
                }
            } else {
                search_for.extend(get_file_path_variants_in_directory(&pathbuf, &filters)?);
            }
        }

        info!("Found {} files to search for references.", search_for.len());
        Ok(SearchFor { search_for })
    }

    pub fn get_unreferenced_files(
        &self,
        searching: Search,
        search_for_relative_path: bool,
        search_for_file_name: bool,
        search_for_file_stem: bool,
    ) -> HashSet<FilePathVariants> {
        let file_path_variants_regexes = FilePathVariantsRegexes::new(
            &self.search_for,
            search_for_relative_path,
            search_for_file_name,
            search_for_file_stem,
        )
        .unwrap();

        let mut unreferenced_files = self.search_for.clone();

        for search in searching.raw_files {
            if !unreferenced_files.is_empty() {
                debug!(
                    "Searching the file {:?}.",
                    search.file_path_variants.file_relative_path
                );

                unreferenced_files.retain(|unreferenced_file| {
                    if unreferenced_file != &search.file_path_variants {
                        if search_for_relative_path
                            && search.is_match(
                                file_path_variants_regexes
                                    .get(&unreferenced_file.file_relative_path),
                            )
                        {
                            return false;
                        }

                        if search_for_file_name
                            && search.is_match(
                                file_path_variants_regexes.get(&unreferenced_file.file_name),
                            )
                        {
                            return false;
                        }

                        if search_for_file_stem
                            && search.is_match(
                                file_path_variants_regexes.get(&unreferenced_file.file_stem),
                            )
                        {
                            return false;
                        }
                    }

                    true
                });
            }
        }

        unreferenced_files
    }
}
