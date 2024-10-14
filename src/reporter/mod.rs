use std::collections::HashSet;

use crate::file_path_variants::FilePathVariants;

pub(crate) fn print(unreferenced_files: HashSet<FilePathVariants>, print_full_path: bool) {
    let mut sorted_unreferenced_files: Vec<&FilePathVariants> = unreferenced_files.iter().collect();
    sorted_unreferenced_files.sort();

    for unreferenced_file in sorted_unreferenced_files {
        unreferenced_file.print(print_full_path);
    }
}
