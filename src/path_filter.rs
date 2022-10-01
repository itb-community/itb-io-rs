use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use lazy_static::lazy_static;
use path_absolutize::Absolutize;

pub struct PathFilter {}

lazy_static! {
    static ref SAVE_DATA_DIR: Mutex<Option<PathBuf>> = Mutex::new(Option::None);
}

impl PathFilter {
    pub fn is_whitelisted<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
        let normalized_path = path.as_ref().absolutize()?;

        let result = normalized_path.starts_with(PathFilter::game_directory()?)
            || normalized_path.starts_with(PathFilter::save_data_directory()?);

        Ok(result)
    }

    fn game_directory() -> std::io::Result<PathBuf> {
        let cwd = std::env::current_dir()?;
        let result_cow = cwd.absolutize()?;
        match result_cow {
            Cow::Borrowed(result) => Ok(result.to_path_buf()),
            Cow::Owned(result) => Ok(result)
        }
    }

    pub fn save_data_directory() -> std::io::Result<PathBuf> {
        let mut it = SAVE_DATA_DIR.lock().unwrap();
        if it.is_some() {
            Ok(it.as_ref().unwrap().to_path_buf())
        } else {
            if let Some(user_dirs) = directories::UserDirs::new() {
                let mut candidates = vec![];

                // Windows user documents storage
                if let Some(document_dir) = user_dirs.document_dir() {
                    candidates.push(document_dir.join("My Games/Into The Breach"));
                }

                // Linux via Steam's Proton wrapper
                candidates.push(PathBuf::from("./../../steamapps/compatdata/590380/pfx/"));

                // Installation directory fallback
                candidates.push(PathBuf::from("./user"));

                let first_valid_candidate = candidates.into_iter()
                    .find(|it| PathFilter::is_save_data_location_valid(it))
                    .ok_or(Error::new(ErrorKind::Other, "Could not find a valid save data location"))?;
                let save_data_dir_cow = first_valid_candidate.absolutize()?;
                let save_data_dir = match save_data_dir_cow {
                    Cow::Borrowed(save_data_dir) => save_data_dir.to_path_buf(),
                    Cow::Owned(save_dat_dir) => save_dat_dir
                };

                Ok(it.insert(save_data_dir).to_path_buf())
            } else {
                Err(Error::new(ErrorKind::Other, "Couldn't retrieve valid home directory path from the operating system"))
            }
        }
    }

    fn is_save_data_location_valid<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().join("io_test.txt").exists()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::path_filter::PathFilter;

    #[test]
    fn test() {
        let path = PathBuf::from(".");
        let result = PathFilter::is_save_data_location_valid(&path);

        assert!(!result);
    }

    #[test]
    fn test2() {
        let path = PathBuf::from("E:/Users/Tomek/Documents/My Games/Into the Breach");
        let result = PathFilter::is_save_data_location_valid(&path);

        assert!(result);
    }

    #[test]
    fn test3() {
        let maybe_dir = PathFilter::save_data_directory();

        if maybe_dir.is_err() {
            panic!("{}", maybe_dir.err().unwrap());
        }

        let dir = maybe_dir.unwrap();
        println!("save data dir: {:?}", dir)
    }

    #[test]
    fn test4() {
        let path = PathBuf::from("E:/Users/Tomek/Documents/My Games/Into the Breach");

        for component in path.components().into_iter() {
            println!("{:?}", component)
        }
    }

    #[test]
    fn test5() {
        let path = PathBuf::from("Users/Tomek/Documents/My Games/Into the Breach");

        for component in path.components().into_iter() {
            println!("{:?}", component)
        }
    }

    #[test]
    fn test6() {
        let path = PathBuf::from("./Users/Tomek/Documents/My Games/Into the Breach");

        for component in path.components().into_iter() {
            println!("{:?}", component)
        }
    }
}