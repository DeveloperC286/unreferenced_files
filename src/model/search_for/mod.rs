use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::model::file_path_variants::FilePathVariants;
use crate::model::file_path_variants_regexes::FilePathVariantsRegexes;
use crate::model::filters::Filters;
use crate::model::raw_files::RawFiles;

#[derive(Clone)]
pub struct SearchFor {
    search_for: HashSet<FilePathVariants>,
}

impl SearchFor {
    pub fn new(initial_search_for: Vec<PathBuf>, filters: Filters) -> Self {
        fn get_file_path_variants_in_directory(
            path: &Path,
            filters: &Filters,
        ) -> HashSet<FilePathVariants> {
            let mut files_path_variants = HashSet::new();
            trace!(
                "Searching the directory {:?} for files to search for.",
                path.display()
            );

            for dir_entry in crate::file_utilities::get_directory_entries(path) {
                match dir_entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();

                        if path.is_file() {
                            if let Some(file_path_variants) = get_file_path_variants(path, filters)
                            {
                                files_path_variants.insert(file_path_variants);
                            }
                        } else {
                            files_path_variants.extend(get_file_path_variants_in_directory(
                                path.as_path(),
                                filters,
                            ));
                        }
                    }
                    Err(error) => {
                        error!("{:?}", error);
                        exit(crate::ERROR_EXIT_CODE);
                    }
                }
            }

            files_path_variants
        }

        fn get_file_path_variants(path: PathBuf, filters: &Filters) -> Option<FilePathVariants> {
            if path.is_file() {
                let file_path_variants = FilePathVariants::new(path);

                if filters.should_ignore(&&file_path_variants.file_canonicalize_path) {
                    debug!(
                        "Ignoring the file {:?} and not searching for it.",
                        file_path_variants.file_relative_path
                    );
                } else {
                    trace!(
                        "Adding {:?} to the files searching for.",
                        file_path_variants.file_relative_path
                    );
                    return Some(file_path_variants);
                }
            } else {
                error!("{:?} is not a file.", path);
                exit(crate::ERROR_EXIT_CODE);
            }

            None
        }

        let mut search_for = HashSet::new();

        for path in initial_search_for {
            if path.is_file() {
                if let Some(file_path_variants) =
                    get_file_path_variants(path.to_path_buf(), &filters)
                {
                    search_for.insert(file_path_variants);
                }
            } else {
                search_for.extend(get_file_path_variants_in_directory(&path, &filters));
            }
        }

        SearchFor { search_for }
    }

    pub fn get_unreferenced_files(
        &self,
        search: RawFiles,
        search_for_relative_path: bool,
        search_for_file_name: bool,
        search_for_file_stem: bool,
    ) -> HashSet<FilePathVariants> {
        let file_path_variants_regexes = FilePathVariantsRegexes::new(
            &self.search_for,
            search_for_relative_path,
            search_for_file_name,
            search_for_file_stem,
        );

        let mut unreferenced_files = self.search_for.clone();

        for raw_file in search.raw_files {
            if !unreferenced_files.is_empty() {
                info!(
                    "Searching the file {:?}.",
                    raw_file.file_path_variants.file_relative_path
                );

                unreferenced_files.retain(|unreferenced_file| {
                    if unreferenced_file.file_canonicalize_path
                        == raw_file.file_path_variants.file_canonicalize_path
                    {
                        warn!(
                            "Not searching {:?} for {:?} as they are the same file.",
                            raw_file.file_path_variants.file_relative_path,
                            unreferenced_file.file_relative_path
                        );
                        return true;
                    }

                    if search_for_relative_path
                        && file_path_variants_regexes
                            .is_file_path_in_file(&unreferenced_file.file_relative_path, &raw_file)
                    {
                        return false;
                    }

                    if search_for_file_name
                        && file_path_variants_regexes
                            .is_file_path_in_file(&unreferenced_file.file_name, &raw_file)
                    {
                        return false;
                    }

                    if search_for_file_stem
                        && file_path_variants_regexes
                            .is_file_path_in_file(&unreferenced_file.file_stem, &raw_file)
                    {
                        return false;
                    }

                    true
                });
            }
        }

        unreferenced_files
    }
}

#[cfg(test)]
#[macro_use]
mod tests;
