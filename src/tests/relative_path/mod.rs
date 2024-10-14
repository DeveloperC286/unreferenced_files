use super::*;

const SEARCH_FOR_RELATIVE_PATH: bool = true;
const SEARCH_FOR_FILE_NAME: bool = false;
const SEARCH_FOR_FILE_STEM: bool = false;

#[test]
fn test_relative_path_searching_for_found() {
    // Given
    let search_for = UNREFERENCED_FILE1.clone();
    let searching = raw_file_with_content!(
        "@test\npublic void testImporting() {\n  import(\"./folder/file1.txt\");\n}"
    );

    // When
    let unreferenced_files = search_for.get_unreferenced_files(
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_unreferenced_files_snapshot!(FOUND, unreferenced_files);
}

#[test]
fn test_relative_path_searching_for_not_found() {
    // Given
    let search_for = UNREFERENCED_FILE1.clone();
    let searching =
        raw_file_with_content!("@test\npublic void testImporting() {\n  import(\"file1.txt\");\n}");

    // When
    let unreferenced_files = search_for.get_unreferenced_files(
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_unreferenced_files_snapshot!(NOT_FOUND, unreferenced_files);
}
