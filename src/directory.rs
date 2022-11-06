use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::file::File;
use crate::path_filter::PathFilter;

#[derive(Debug)]
pub struct Directory {
    pub(crate) path: PathBuf,
}

fn normalize(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string().replace("\\", "/") + "/"
}

impl Directory {
    pub fn path(&self) -> String {
        // Have directories report their path with a trailing slash, since that's sometimes
        // convenient when working with paths in Lua.
        normalize(&self.path)
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn parent(&self) -> std::io::Result<Option<Directory>> {
        let maybe_dir = self.path.parent()
            .map(|parent| Directory::from(parent));

        if let Some(dir) = maybe_dir {
            if PathFilter::is_whitelisted(&dir.path)? {
                Ok(Option::Some(dir))
            } else {
                Err(Error::new(ErrorKind::Other, "Parent is not an allowed directory"))
            }
        } else {
            Ok(None)
        }
    }

    pub fn relativize<P: AsRef<Path>>(&self, path: P) -> Option<String> {
        pathdiff::diff_paths(path, &self.path)
            .map(|path| normalize(&path))
    }

    pub fn files(&self) -> std::io::Result<Vec<File>> {
        if self.exists() {
            let mut result = Vec::new();

            for entry in WalkDir::new(&self.path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(true)
                .into_iter()
            {
                let entry = entry?;
                if entry.file_type().is_file() {
                    result.push(File::from(entry.path()));
                }
            }

            Ok(result)
        } else {
            Err(Error::new(ErrorKind::Other, "Directory doesn't exist"))
        }
    }

    pub fn directories(&self) -> std::io::Result<Vec<Directory>> {
        if self.exists() {
            let mut result = Vec::new();

            for entry in WalkDir::new(&self.path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(true)
                .into_iter()
            {
                let entry = entry?;
                if entry.file_type().is_dir() {
                    result.push(Directory::from(entry.path()));
                }
            }

            Ok(result)
        } else {
            Err(Error::new(ErrorKind::Other, "Directory doesn't exist"))
        }
    }

    pub fn make_directories(&self) -> std::io::Result<()> {
        if PathFilter::is_whitelisted(&self.path)? {
            std::fs::create_dir_all(&self.path)
        } else {
            Err(Error::new(ErrorKind::Other, "Path does not point to an allowed directory"))
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn delete(&self) -> std::io::Result<()> {
        if self.exists() {
            std::fs::remove_dir_all(&self.path)
        } else {
            Ok(())
        }
    }
}

impl<P: AsRef<Path>> From<P> for Directory where PathBuf: From<P> {
    fn from(path: P) -> Self {
        Directory {
            path: PathBuf::from(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::directory::Directory;

    #[test]
    fn path_should_be_reported_with_trailing_slash() {
        let dir = Directory::from("asd");

        assert_eq!("asd/", dir.path());
        assert_eq!("asd", dir.path.to_str().unwrap())
    }

    #[test]
    fn relativize_should_remove_common_path() {
        let dir = Directory::from("some/path");
        let maybe_relative = dir.relativize("some/path/test");

        assert_eq!("test", maybe_relative.unwrap());
    }

    #[test]
    fn relativize_should_add_shorthands() {
        let dir = Directory::from("some/path/test");
        let maybe_relative = dir.relativize("some");

        assert_eq!("../..", maybe_relative.unwrap());
    }
}
