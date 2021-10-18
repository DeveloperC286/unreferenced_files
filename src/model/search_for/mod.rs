use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::model::file_path_variants::FilePathVariants;
use crate::model::file_path_variants_regexes::FilePathVariantsRegexes;
use crate::model::filters::Filters;
use crate::model::search::Search;

#[cfg(not(test))]
pub(crate) struct SearchFor {
    search_for: HashSet<FilePathVariants>,
}

// For unit testing.
#[cfg(test)]
#[derive(Clone)]
pub(crate) struct SearchFor {
    pub(crate) search_for: HashSet<FilePathVariants>,
}

impl SearchFor {
    pub(crate) fn new<T: AsRef<str>>(paths: &[T], filters: Filters) -> Result<SearchFor, ()> {
        fn get_file_path_variants_in_directory(
            path: &Path,
            filters: &Filters,
        ) -> Result<HashSet<FilePathVariants>, ()> {
            let mut files_path_variants = HashSet::new();
            trace!(
                "Searching the directory {:?} for files to search for.",
                path.display()
            );

            match std::fs::read_dir(path) {
                Ok(entries) => {
                    for dir_entry in entries {
                        match dir_entry {
                            Ok(dir_entry) => {
                                let path = dir_entry.path();

                                if path.is_file() {
                                    if let Ok(file_path_variants) =
                                        get_file_path_variants(path, filters)
                                    {
                                        files_path_variants.insert(file_path_variants);
                                    }
                                } else {
                                    files_path_variants.extend(
                                        get_file_path_variants_in_directory(
                                            path.as_path(),
                                            filters,
                                        )?,
                                    );
                                }
                            }
                            Err(error) => {
                                error!("{:?}", error);
                                return Err(());
                            }
                        }
                    }
                }
                Err(error) => {
                    error!("{:?}", error);
                    return Err(());
                }
            }

            Ok(files_path_variants)
        }

        fn get_file_path_variants(
            path: PathBuf,
            filters: &Filters,
        ) -> Result<FilePathVariants, ()> {
            if path.is_file() {
                let file_path_variants = FilePathVariants::new(path).unwrap();

                if filters.should_ignore(&file_path_variants.file_canonicalize_path) {
                    debug!(
                        "Ignoring the file {:?} and not searching for it.",
                        file_path_variants.file_relative_path
                    );
                } else {
                    trace!(
                        "Adding {:?} to the files searching for.",
                        file_path_variants.file_relative_path
                    );
                    return Ok(file_path_variants);
                }
            } else {
                error!("{:?} is not a file.", path);
            }

            Err(())
        }

        let mut search_for = HashSet::new();

        match crate::model::utilities::to_pathbufs(paths) {
            Ok(pathbufs) => {
                for pathbuf in pathbufs {
                    if pathbuf.is_file() {
                        if let Ok(file_path_variants) = get_file_path_variants(pathbuf, &filters) {
                            search_for.insert(file_path_variants);
                        }
                    } else {
                        search_for.extend(get_file_path_variants_in_directory(&pathbuf, &filters)?);
                    }
                }
            }
            Err(_) => {
                return Err(());
            }
        }

        Ok(SearchFor { search_for })
    }

    pub(crate) fn get_unreferenced_files(
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
                info!(
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
