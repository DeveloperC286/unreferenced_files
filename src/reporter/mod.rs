use std::collections::HashSet;

use crate::model::file_path_variants::FilePathVariants;

pub fn print(unreferenced_files: HashSet<FilePathVariants>) {
    let mut sorted_unreferenced_files: Vec<&FilePathVariants> = unreferenced_files.iter().collect();
    sorted_unreferenced_files.sort();

    for unreferenced_file in sorted_unreferenced_files {
        println!("{}", unreferenced_file.file_relative_path);
    }
}
