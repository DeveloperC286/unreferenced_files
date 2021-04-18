use super::*;

const SEARCH_FOR_RELATIVE_PATH: bool = true;
const SEARCH_FOR_FILE_NAME: bool = true;
const SEARCH_FOR_FILE_STEM: bool = true;

#[test]
fn test_singular_searching_for_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1.clone();
    let searching = raw_files_with_content!(
        "@test\npublic void testImporting() {\n  import(\"./file1.txt\");\n}",
        "@test\npublic void testImporting() {\n  import(\"./file2.txt\");\n}"
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
fn test_singular_searching_for_not_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1.clone();
    let searching = raw_files_with_content!(
        "@test\npublic void testImporting() {\n  import(\"./file2.txt\");\n}",
        "@test\npublic void testImporting() {\n  import(\"./file3.txt\");\n}"
    );

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

#[test]
fn test_multiple_searching_for_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1_AND_FILE2.clone();
    let searching = raw_files_with_content!(
        "@test\npublic void testImporting() {\n  import(\"./file1.txt\");\n}",
        "@test\npublic void testImporting() {\n  import(\"./file2.txt\");\n}"
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
fn test_multiple_searching_for_not_found() {
    // Given
    let mut unreferenced_files = UNREFERENCED_FILE1_AND_FILE2.clone();
    let searching = raw_files_with_content!(
        "@test\npublic void testImporting() {\n  import(\"./file3.txt\");\n}",
        "@test\npublic void testImporting() {\n  import(\"./file4.txt\");\n}"
    );

    // When
    unreferenced_files.remove_referenced_files(
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(MULTIPLE_NOT_FOUND, unreferenced_files);
}
