use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::file::File;
use crate::path_filter::PathFilter;

#[derive(Debug)]
pub struct Directory {
    pub(crate) path: PathBuf,
}

impl Directory {
    pub fn path(&self) -> String {
        // Have directories report their path with a trailing slash, since that's sometimes
        // convenient when working with paths in Lua.
        self.path.to_str().unwrap().to_string().replace("\\", "/") + "/"
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

    pub fn file<P: AsRef<Path>>(&self, paths: Vec<P>) -> std::io::Result<File> {
        let path: PathBuf = paths.iter().collect();
        let path = self.path.join(path);

        if PathFilter::is_whitelisted(&path)? {
            Ok(File::from(path))
        } else {
            Err(Error::new(ErrorKind::Other, "File is not within an allowed directory"))
        }
    }

    pub fn directory<P: AsRef<Path>>(&self, paths: Vec<P>) -> std::io::Result<Directory> {
        let path: PathBuf = paths.iter().collect();
        let path = self.path.join(path);

        if PathFilter::is_whitelisted(&path)? {
            Ok(Directory::from(path))
        } else {
            Err(Error::new(ErrorKind::Other, "Path does not point to an allowed directory"))
        }
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

impl <P: AsRef<Path>> From<P> for Directory where PathBuf: From<P> {
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
    fn file_should_create_file_from_joined_paths() {
        let dir = Directory::from("asd");
        let f = dir.file(vec!["qwe", "zxc"]).unwrap();

        assert_eq!("asd/", dir.path());
        assert_eq!("asd/qwe/zxc", f.path());
    }
}
