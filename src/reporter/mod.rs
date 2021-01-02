use std::collections::HashSet;
use std::path::PathBuf;

pub fn print(unreferenced_files: HashSet<PathBuf>) {
    for unreferenced_file in unreferenced_files {
        println!("{}", unreferenced_file.display());
    }
}
