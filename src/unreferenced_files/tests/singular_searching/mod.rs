use super::*;

const SEARCH_FOR_RELATIVE_PATH: bool = true;
const SEARCH_FOR_FILE_NAME: bool = true;
const SEARCH_FOR_FILE_STEM: bool = true;

#[test]
fn test_singular_searching_for_found() {
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
    let searching = [RawFile {
        file_path_variants: FilePathVariants {
            file_canonicalize_path: "/tmp/test.java".to_string(),
            file_relative_path: "./test.java".to_string(),
            file_name: "test.java".to_string(),
            file_stem: "test".to_string(),
        },
        file_content: "@test\npublic void testImporting() {\n  import(\"./file1.txt\");\n}"
            .to_string(),
    }]
    .iter()
    .cloned()
    .collect();

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
fn test_singular_searching_for_not_found() {
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
    let searching = [RawFile {
        file_path_variants: FilePathVariants {
            file_canonicalize_path: "/tmp/test.java".to_string(),
            file_relative_path: "./test.java".to_string(),
            file_name: "test.java".to_string(),
            file_stem: "test".to_string(),
        },
        file_content: "@test\npublic void testImporting() {\n  import(\"./file2.txt\");\n}"
            .to_string(),
    }]
    .iter()
    .cloned()
    .collect();

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

#[test]
fn test_multiple_searching_for_found() {
    // Given
    let searching_for = [
        FilePathVariants {
            file_canonicalize_path: "/tmp/folder/file1.txt".to_string(),
            file_relative_path: "./folder/file1.txt".to_string(),
            file_name: "file1.txt".to_string(),
            file_stem: "file1".to_string(),
        },
        FilePathVariants {
            file_canonicalize_path: "/tmp/folder/file2.txt".to_string(),
            file_relative_path: "./folder/file2.txt".to_string(),
            file_name: "file2.txt".to_string(),
            file_stem: "file2".to_string(),
        },
    ]
    .iter()
    .cloned()
    .collect();
    let searching = [RawFile {
        file_path_variants: FilePathVariants {
            file_canonicalize_path: "/tmp/test.java".to_string(),
            file_relative_path: "./test.java".to_string(),
            file_name: "test.java".to_string(),
            file_stem: "test".to_string(),
        },
        file_content: "@test\npublic void testImporting() {\n  import(\"./file2.txt\");\n  import(\"./file1.txt\");\n}"
            .to_string(),
    }]
        .iter()
        .cloned()
        .collect();

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
fn test_multiple_searching_for_not_found() {
    // Given
    let searching_for = [
        FilePathVariants {
            file_canonicalize_path: "/tmp/folder/file1.txt".to_string(),
            file_relative_path: "./folder/file1.txt".to_string(),
            file_name: "file1.txt".to_string(),
            file_stem: "file1".to_string(),
        },
        FilePathVariants {
            file_canonicalize_path: "/tmp/folder/file2.txt".to_string(),
            file_relative_path: "./folder/file2.txt".to_string(),
            file_name: "file2.txt".to_string(),
            file_stem: "file2".to_string(),
        },
    ]
    .iter()
    .cloned()
    .collect();
    let searching = [RawFile {
        file_path_variants: FilePathVariants {
            file_canonicalize_path: "/tmp/test.java".to_string(),
            file_relative_path: "./test.java".to_string(),
            file_name: "test.java".to_string(),
            file_stem: "test".to_string(),
        },
        file_content: "@test\npublic void testImporting() {\n  import(\"./file3.txt\");\n  import(\"./file4.txt\");\n}"
            .to_string(),
    }]
        .iter()
        .cloned()
        .collect();

    // When
    let unreferenced_files = get_unreferenced_files(
        searching_for,
        searching,
        SEARCH_FOR_RELATIVE_PATH,
        SEARCH_FOR_FILE_NAME,
        SEARCH_FOR_FILE_STEM,
    );

    // Then
    assert_sorted_unreferenced_files_snapshot!(MULTIPLE_NOT_FOUND, unreferenced_files);
}
