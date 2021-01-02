use std::collections::HashSet;

use crate::model::file_path_variants::FilePathVariants;

pub fn print(unreferenced_files: HashSet<FilePathVariants>) {
    for unreferenced_file in unreferenced_files {
        println!("{}", unreferenced_file.file_relative_path);
    }
}
