use std::path::{Path, PathBuf};
use std::process::exit;

pub(crate) fn get_paths(paths: Vec<String>) -> Vec<PathBuf> {
    fn get_path(path: &str) -> PathBuf {
        let path = Path::new(path);

        if !path.exists() {
            error!("{:?} does not exist.", path);
            exit(crate::ERROR_EXIT_CODE);
        }

        path.to_path_buf()
    }

    paths.iter().map(|path| get_path(path)).collect()
}
