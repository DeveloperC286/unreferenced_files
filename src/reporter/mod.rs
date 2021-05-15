use std::collections::HashSet;

use crate::model::file_path_variants::FilePathVariants;

pub fn print(unreferenced_files: HashSet<FilePathVariants>, print_full_path: bool) {
    let mut sorted_unreferenced_files: Vec<&FilePathVariants> = unreferenced_files.iter().collect();
    sorted_unreferenced_files.sort();

    for unreferenced_file in sorted_unreferenced_files {
        if print_full_path {
            println!("{}", unreferenced_file.file_canonicalize_path);
        } else {
            println!("{}", unreferenced_file.file_relative_path);
        }
    }
}
