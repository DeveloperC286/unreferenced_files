use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Debug, Clone, Eq)]
pub(crate) struct FilePathVariants {
    pub(crate) file_canonicalize_path: String,
    pub(crate) file_relative_path: String,
    pub(crate) file_name: String,
    pub(crate) file_stem: String,
}

impl FilePathVariants {
    pub(crate) fn new(path: PathBuf) -> FilePathVariants {
        fn get_file_canonicalize_path(path: &Path) -> String {
            match path.canonicalize() {
                Ok(canonicalized_path) => canonicalized_path.display().to_string(),
                Err(error) => {
                    error!("{:?}", error);
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
        }

        fn get_file_relative_path(path: &Path) -> String {
            path.display()
                .to_string()
                .trim_start_matches("./")
                .to_string()
        }

        fn get_file_name(path: &Path) -> String {
            match path.file_name() {
                Some(file_name) => match file_name.to_str() {
                    Some(str) => str.to_string(),
                    None => {
                        error!("Can not convert {:?} to str.", file_name);
                        exit(crate::ERROR_EXIT_CODE);
                    }
                },
                None => {
                    error!("{:?} has no file name.", path.display());
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
        }

        fn get_file_stem(path: &Path) -> String {
            match path.file_stem() {
                Some(file_stem) => match file_stem.to_str() {
                    Some(str) => str.to_string(),
                    None => {
                        error!("Can not convert {:?} to str.", file_stem);
                        exit(crate::ERROR_EXIT_CODE);
                    }
                },
                None => {
                    error!("{:?} has no file steam.", path.display());
                    exit(crate::ERROR_EXIT_CODE);
                }
            }
        }

        FilePathVariants {
            file_canonicalize_path: get_file_canonicalize_path(&path),
            file_relative_path: get_file_relative_path(&path),
            file_name: get_file_name(&path),
            file_stem: get_file_stem(&path),
        }
    }

    pub(crate) fn print(&self, print_full_path: bool) {
        if print_full_path {
            println!("{}", self.file_canonicalize_path);
        } else {
            println!("{}", self.file_relative_path);
        }
    }
}

impl PartialEq for FilePathVariants {
    #[inline]
    fn eq(&self, other: &FilePathVariants) -> bool {
        self.file_canonicalize_path == other.file_canonicalize_path
    }
}

impl Hash for FilePathVariants {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.file_canonicalize_path.hash(hasher);
    }
}

impl PartialOrd for FilePathVariants {
    #[inline]
    fn partial_cmp(&self, other: &FilePathVariants) -> Option<Ordering> {
        self.file_canonicalize_path
            .partial_cmp(&other.file_canonicalize_path)
    }
}

impl Ord for FilePathVariants {
    #[inline]
    fn cmp(&self, other: &FilePathVariants) -> Ordering {
        self.file_canonicalize_path
            .cmp(&other.file_canonicalize_path)
    }
}
