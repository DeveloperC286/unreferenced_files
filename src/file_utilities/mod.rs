use std::path::Path;
use std::process::exit;

pub fn get_path(path: &str) -> &Path {
    let path = Path::new(path);

    if !path.exists() {
        error!("{:?} does not exist.", path);
        exit(crate::ERROR_EXIT_CODE);
    }

    path
}

pub fn get_file_content(path: &Path) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(file_content) => Some(file_content),
        Err(error) => {
            warn!(
                "Encountered {} while trying to read the file {}.",
                error,
                path.display()
            );
            None
        }
    }
}

pub fn get_directory_entries(path: &Path) -> std::fs::ReadDir {
    match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}
