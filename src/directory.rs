use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::file::File;
use crate::path_filter::PathFilter;

#[derive(Debug)]
pub struct Directory {
    pub(crate) path: PathBuf
}

impl Directory {
    pub fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
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

    pub fn files(&self) -> Vec<File> {
        let mut result = Vec::new();

        for entry in WalkDir::new(&self.path)
            .min_depth(1)
            .max_depth(1)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                result.push(File::from(entry.path()));
            }
        }

        result
    }

    pub fn directories(&self) -> Vec<Directory> {
        let mut result = Vec::new();

        for entry in WalkDir::new(&self.path)
            .min_depth(1)
            .max_depth(1)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                result.push(Directory::from(entry.path()));
            }
        }

        result
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn delete(&self) -> std::io::Result<()> {
        std::fs::remove_dir_all(&self.path)
    }
}

impl From<&Path> for Directory {
    fn from(path: &Path) -> Self {
        Directory {
            path: PathBuf::from(path)
        }
    }
}

impl From<PathBuf> for Directory {
    fn from(path: PathBuf) -> Self {
        Directory { path }
    }
}

impl <'a> From<Cow<'a, Path>> for Directory {
    fn from(path_cow: Cow<'a, Path>) -> Self {
        Self::from(PathBuf::from(path_cow))
    }
}
