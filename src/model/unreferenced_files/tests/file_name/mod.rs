use super::*;

const SEARCH_FOR_RELATIVE_PATH: bool = false;
const SEARCH_FOR_FILE_NAME: bool = true;
const SEARCH_FOR_FILE_STEM: bool = false;

#[test]
fn test_file_name_searching_for_relative_path_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1.clone();
    let searching = raw_file_with_content!(
        "@test\npublic void testImporting() {\n  import(\"./folder/file1.txt\");\n}"
    );

    // When
    unreferenced_files.remove_referenced_files(
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(FOUND, unreferenced_files);
}

#[test]
fn test_file_name_searching_for_file_name_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1.clone();
    let searching =
        raw_file_with_content!("@test\npublic void testImporting() {\n  import(\"file1.txt\");\n}");

    // When
    unreferenced_files.remove_referenced_files(
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(FOUND, unreferenced_files);
}

#[test]
fn test_file_name_searching_for_not_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1.clone();
    let searching =
        raw_file_with_content!("@test\npublic void testImporting() {\n  import(\"file1\");\n}");

    // When
    unreferenced_files.remove_referenced_files(
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(NOT_FOUND, unreferenced_files);
}
