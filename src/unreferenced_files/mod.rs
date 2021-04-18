use std::collections::HashSet;

use crate::model::file_path_variants::FilePathVariants;
use crate::model::file_path_variants_regexes::FilePathVariantsRegexes;
use crate::model::raw_file::RawFile;

pub fn get_unreferenced_files(
    search_for: HashSet<FilePathVariants>,
    searching: HashSet<RawFile>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_stem: bool,
) -> HashSet<FilePathVariants> {
    let file_path_variants_regexes = FilePathVariantsRegexes::new(
        &search_for,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    );

    get_unreferenced_files_in_directory(
        search_for,
        file_path_variants_regexes,
        searching,
        search_for_relative_path,
        search_for_file_name,
        search_for_file_stem,
    )
}

fn get_unreferenced_files_in_directory(
    mut search_for: HashSet<FilePathVariants>,
    file_path_variants_regexes: FilePathVariantsRegexes,
    searching: HashSet<RawFile>,
    search_for_relative_path: bool,
    search_for_file_name: bool,
    search_for_file_stem: bool,
) -> HashSet<FilePathVariants> {
    for raw_file in searching {
        if search_for.is_empty() {
            return search_for;
        }

        info!(
            "Searching the file {:?}.",
            raw_file.file_path_variants.file_relative_path
        );

        search_for.retain(|unreferenced_file| {
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

    search_for
}

#[cfg(test)]
#[macro_use]
mod tests;
