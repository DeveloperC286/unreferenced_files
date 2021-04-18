use crate::model::file_path_variants::FilePathVariants;
use crate::model::raw_file::RawFile;
use crate::model::raw_files::RawFiles;

use super::*;

const FOUND: &str = "found";
const NOT_FOUND: &str = "not_found";
const MULTIPLE_NOT_FOUND: &str = "multiple_not_found";

lazy_static! {
    static ref TEST_JAVA: FilePathVariants = FilePathVariants {
        file_canonicalize_path: "/tmp/test.java".to_string(),
        file_relative_path: "./test.java".to_string(),
        file_name: "test.java".to_string(),
        file_stem: "test".to_string(),
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
    static ref UNREFERENCED_FILE1: UnreferencedFiles = {
        let mut unreferenced_files = HashSet::new();
        unreferenced_files.insert(FILE1_TXT.clone());
        UnreferencedFiles { unreferenced_files }
    };
    static ref UNREFERENCED_FILE1_AND_FILE2: UnreferencedFiles = {
        let mut unreferenced_files = HashSet::new();
        unreferenced_files.insert(FILE1_TXT.clone());
        unreferenced_files.insert(FILE2_TXT.clone());
        UnreferencedFiles { unreferenced_files }
    };
}

macro_rules! raw_file_with_content {
    ($file_content:expr) => {
        RawFiles {
            raw_files: {
                let mut raw_files = HashSet::new();
                raw_files.insert(RawFile {
                    file_path_variants: TEST_JAVA.clone(),
                    file_content: $file_content.to_string(),
                });
                raw_files
            },
        }
    };
}

macro_rules! raw_files_with_content {
    ($file1_content:expr, $file2_content:expr) => {
        RawFiles {
            raw_files: {
                let mut raw_files = HashSet::new();
                raw_files.insert(RawFile {
                    file_path_variants: TEST_JAVA.clone(),
                    file_content: $file1_content.to_string(),
                });
                raw_files.insert(RawFile {
                    file_path_variants: TEST_JAVA.clone(),
                    file_content: $file2_content.to_string(),
                });
                raw_files
            },
        }
    };
}

macro_rules! assert_sorted_unreferenced_files_snapshot {
    ($snapshot_name:expr, $unreferenced_files:expr) => {
        let mut sorted_unreferenced_files: Vec<_> =
            $unreferenced_files.unreferenced_files.iter().collect();
        sorted_unreferenced_files.sort();
        insta::assert_debug_snapshot!($snapshot_name, sorted_unreferenced_files);
    };
}

mod file_name;
mod file_stem;
mod multiple_searching;
mod relative_path;
mod singular_searching;
