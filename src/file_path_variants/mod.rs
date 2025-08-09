use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

#[derive(Debug, Clone, Eq)]
pub struct FilePathVariants {
    pub(crate) file_canonicalize_path: String,
    pub(crate) file_relative_path: String,
    pub(crate) file_name: String,
    pub(crate) file_stem: String,
}

impl FilePathVariants {
    pub(crate) fn new(path: PathBuf) -> Result<FilePathVariants> {
        fn get_file_canonicalize_path(path: &Path) -> Result<String> {
            let canonicalized_path = path.canonicalize()?;
            Ok(canonicalized_path.display().to_string())
        }

        fn get_file_relative_path(path: &Path) -> String {
            path.display()
                .to_string()
                .trim_start_matches("./")
                .to_string()
        }

        fn get_file_name(path: &Path) -> Result<String> {
            match path.file_name() {
                Some(file_name) => match file_name.to_str() {
                    Some(str) => Ok(str.to_string()),
                    None => {
                        bail!(format!("Can not convert {:?} to str.", file_name));
                    }
                },
                None => {
                    bail!(format!("{:?} has no file name.", path.display()));
                }
            }
        }

        fn get_file_stem(path: &Path) -> Result<String> {
            match path.file_stem() {
                Some(file_stem) => match file_stem.to_str() {
                    Some(str) => Ok(str.to_string()),
                    None => {
                        bail!(format!("Can not convert {:?} to str.", file_stem));
                    }
                },
                None => {
                    bail!(format!("{:?} has no file steam.", path.display()));
                }
            }
        }

        let file_canonicalize_path = get_file_canonicalize_path(&path)?;
        let file_name = get_file_name(&path)?;
        let file_stem = get_file_stem(&path)?;

        Ok(FilePathVariants {
            file_canonicalize_path,
            file_relative_path: get_file_relative_path(&path),
            file_name,
            file_stem,
        })
    }

    pub fn print(&self, print_full_path: bool) {
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
        Some(self.cmp(other))
    }
}

impl Ord for FilePathVariants {
    #[inline]
    fn cmp(&self, other: &FilePathVariants) -> Ordering {
        self.file_canonicalize_path
            .cmp(&other.file_canonicalize_path)
    }
}
