use std::path::{Path, PathBuf};

pub(crate) fn to_pathbufs<T: AsRef<str>>(paths: &[T]) -> Result<Vec<PathBuf>, ()> {
    fn to_pathbuf<T: AsRef<str>>(path: T) -> Result<PathBuf, ()> {
        let path = Path::new(path.as_ref());

        if !path.exists() {
            error!("{:?} does not exist.", path);
            return Err(());
        }

        Ok(path.to_path_buf())
    }

    let mut pathbufs = vec![];

    for path in paths {
        let pathbuf = to_pathbuf(path)?;
        pathbufs.push(pathbuf);
    }

    Ok(pathbufs)
}
