use super::*;

const SEARCH_FOR_RELATIVE_PATH: bool = true;
const SEARCH_FOR_FILE_NAME: bool = false;
const SEARCH_FOR_FILE_STEM: bool = false;

#[test]
fn test_relative_path_searching_for_found() {
    // Given
    let searching_for = [FilePathVariants {
        file_canonicalize_path: "/tmp/folder/file1.txt".to_string(),
        file_relative_path: "./folder/file1.txt".to_string(),
        file_name: "file1.txt".to_string(),
        file_stem: "file1".to_string(),
    }]
    .iter()
    .cloned()
    .collect();

    let mut raw_files = HashSet::new();
    raw_files.insert(RawFile {
        file_path_variants: FilePathVariants {
            file_canonicalize_path: "/tmp/test.java".to_string(),
            file_relative_path: "./test.java".to_string(),
            file_name: "test.java".to_string(),
            file_stem: "test".to_string(),
        },
        file_content: "@test\npublic void testImporting() {\n  import(\"./folder/file1.txt\");\n}"
            .to_string(),
    });
    let searching = RawFiles { raw_files };

    // When
    let unreferenced_files = get_unreferenced_files(
        searching_for,
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(FOUND, unreferenced_files);
}

#[test]
fn test_relative_path_searching_for_not_found() {
    // Given
    let searching_for = [FilePathVariants {
        file_canonicalize_path: "/tmp/folder/file1.txt".to_string(),
        file_relative_path: "./folder/file1.txt".to_string(),
        file_name: "file1.txt".to_string(),
        file_stem: "file1".to_string(),
    }]
    .iter()
    .cloned()
    .collect();

    let mut raw_files = HashSet::new();
    raw_files.insert(RawFile {
        file_path_variants: FilePathVariants {
            file_canonicalize_path: "/tmp/test.java".to_string(),
            file_relative_path: "./test.java".to_string(),
            file_name: "test.java".to_string(),
            file_stem: "test".to_string(),
        },
        file_content: "@test\npublic void testImporting() {\n  import(\"file1.txt\");\n}"
            .to_string(),
    });
    let searching = RawFiles { raw_files };

    // When
    let unreferenced_files = get_unreferenced_files(
        searching_for,
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(NOT_FOUND, unreferenced_files);
}
