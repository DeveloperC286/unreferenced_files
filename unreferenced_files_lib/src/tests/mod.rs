use std::collections::HashSet;

use crate::file_path_variants::FilePathVariants;
use crate::search::raw_file::RawFile;
use crate::search::Search;
use crate::search_for::SearchFor;

const FOUND: &str = "found";
const NOT_FOUND: &str = "not_found";
const MULTIPLE_NOT_FOUND: &str = "multiple_not_found";

lazy_static! {
    static ref SEARCH: FilePathVariants = FilePathVariants {
        file_canonicalize_path: "/tmp/search.java".to_string(),
        file_relative_path: "./search.java".to_string(),
        file_name: "search.java".to_string(),
        file_stem: "search".to_string(),
    };
    static ref FILE1_TXT: FilePathVariants = FilePathVariants {
        file_canonicalize_path: "/tmp/folder/file1.txt".to_string(),
        file_relative_path: "./folder/file1.txt".to_string(),
        file_name: "file1.txt".to_string(),
        file_stem: "file1".to_string(),
    };
    static ref FILE2_TXT: FilePathVariants = FilePathVariants {
        file_canonicalize_path: "/tmp/folder/file2.txt".to_string(),
        file_relative_path: "./folder/file2.txt".to_string(),
        file_name: "file2.txt".to_string(),
        file_stem: "file2".to_string(),
    };
    static ref UNREFERENCED_FILE1: SearchFor = {
        let mut search_for = HashSet::new();
        search_for.insert(FILE1_TXT.clone());
        SearchFor { search_for }
    };
    static ref UNREFERENCED_FILE1_AND_FILE2: SearchFor = {
        let mut search_for = HashSet::new();
        search_for.insert(FILE1_TXT.clone());
        search_for.insert(FILE2_TXT.clone());
        SearchFor { search_for }
    };
}

macro_rules! raw_file_with_content {
    ($file_content:expr) => {
        Search {
            raw_files: {
                let mut raw_files = HashSet::new();
                raw_files.insert(RawFile {
                    file_path_variants: SEARCH.clone(),
                    file_content: $file_content.to_string(),
                });
                raw_files
            },
        }
    };
}

macro_rules! raw_files_with_content {
    ($file1_content:expr, $file2_content:expr) => {
        Search {
            raw_files: {
                let mut raw_files = HashSet::new();
                raw_files.insert(RawFile {
                    file_path_variants: SEARCH.clone(),
                    file_content: $file1_content.to_string(),
                });
                raw_files.insert(RawFile {
                    file_path_variants: SEARCH.clone(),
                    file_content: $file2_content.to_string(),
                });
                raw_files
            },
        }
    };
}

macro_rules! assert_unreferenced_files_snapshot {
    ($snapshot_name:expr, $search_for:expr) => {
        let mut sorted_search_for: Vec<_> = $search_for.iter().collect();
        sorted_search_for.sort();
        insta::assert_debug_snapshot!($snapshot_name, sorted_search_for);
    };
}

mod file_name;
mod file_stem;
mod multiple_searching;
mod relative_path;
mod singular_searching;
