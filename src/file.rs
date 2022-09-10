use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::directory::Directory;
use crate::path_filter::PathFilter;

#[derive(Debug)]
pub struct File {
    pub(crate) path: PathBuf
}

impl File {
    pub fn path(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    pub fn name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn name_without_extension(&self) -> String {
        self.path.file_stem().unwrap().to_str().unwrap().to_string()
    }

    pub fn extension(&self) -> Option<String> {
        self.path.extension()
            .map(|s| s.to_str().unwrap().to_string())
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

    pub fn read_to_byte_array(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.path)
    }

    pub fn read_to_string(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.path)
    }

    pub fn write_string(&self, content: String) -> std::io::Result<()> {
        std::fs::write(&self.path, content)
    }

    pub fn write_byte_array(&self, content: Vec<u8>) -> std::io::Result<()> {
        std::fs::write(&self.path, content)
    }

    pub fn copy(&self, destination: &Path) -> std::io::Result<()> {
        if PathFilter::is_whitelisted(&PathBuf::from(destination))? {
            std::fs::copy(&self.path, destination).map(|_| ())
        } else {
            Err(Error::new(ErrorKind::Other, "Destination is not within allowed directory"))
        }
    }

    pub fn move_file(&self, destination: &Path) -> std::io::Result<()> {
        if PathFilter::is_whitelisted(&PathBuf::from(destination))? {
            std::fs::rename(&self.path, destination)
        } else {
            Err(Error::new(ErrorKind::Other, "Destination is not within allowed directory"))
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn delete(&self) -> std::io::Result<()> {
        std::fs::remove_file(&self.path)
    }
}

impl From<&Path> for File {
    fn from(path: &Path) -> Self {
        File {
            path: path.to_path_buf()
        }
    }
}

impl From<PathBuf> for File {
    fn from(path: PathBuf) -> Self {
        File { path }
    }
}

impl <'a> From<Cow<'a, Path>> for File {
    fn from(path_cow: Cow<'a, Path>) -> Self {
        Self::from(PathBuf::from(path_cow))
    }
}
