use crate::model::file_path_variants::FilePathVariants;
use crate::model::raw_file::RawFile;

use super::*;

const FOUND: &str = "found";
const NOT_FOUND: &str = "not_found";
const MULTIPLE_NOT_FOUND: &str = "multiple_not_found";

macro_rules! assert_sorted_unreferenced_files_snapshot {
    ($snapshot_name:expr, $unreferenced_files:expr) => {
        let mut sorted_unreferenced_files: Vec<_> = $unreferenced_files.into_iter().collect();
        sorted_unreferenced_files.sort();
        insta::assert_debug_snapshot!($snapshot_name, sorted_unreferenced_files);
    };
}

mod file_name;
mod file_stem;
mod multiple_searching;
mod relative_path;
mod singular_searching;
