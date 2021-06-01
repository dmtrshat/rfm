//! # rfm
//!
//! **`rfm`** is a convenient and intuitive way to interact with files on OS.
//! Function naming is similar to Unix commands
//! _(`ls`, `mkdir`, `mv`, `rm`, `cp`, `touch` etc.)_,
//! so you can easily figure out how to use it.
mod utils;
use std::{
    fs::{copy, create_dir, read_dir, remove_dir_all, remove_file},
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};

macro_rules! err {
    ($text:expr, $kind:expr) => {
        return Err(Error::new($kind, $text));
    };

    ($text:expr) => {
        err!($text, ErrorKind::Other)
    };
}

// ------------------------------------------------------------------------ //

/// Read the directory/directories and return the content.
/// `dir` - takes the path to the directory whose contents you want to
/// retrieve.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `dir` contains file or directory does not exist.
/// - Param `dir` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::ls;
///
///  let dir = std::path::Path::new("./dir1").to_path_buf();
///
///  let directory_contents = ls(&dir)?;
/// ```
pub fn ls(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut elements: Vec<PathBuf> = vec![];
    for entry in read_dir(dir)? {
        let dir = entry?;
        elements.push(dir.path())
    }
    Ok(elements)
}

/// Deletes files/directories (including nested files/directories).
/// `from` - takes a list of paths of what you want to delete.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `from` contains file or directory does not exist.
/// - Param `from` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::rm;
///
///  let dir = std::path::Path::new("./dir").to_path_buf();
///  let file = std::path::Path::new("./file.txt").to_path_buf();
///  let elements: Vec<&std::path::PathBuf> = vec![&dir, &file];
///
///  rm(&elements)?;
/// ```
pub fn rm(from: &Vec<&PathBuf>) -> Result<()> {
    if from.len() <= 0 {
        let err_msg = format!("from param is empty - {:?}", from);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    for i in from {
        if i.is_file() {
            remove_file(i)?;
        } else {
            remove_dir_all(i)?;
        }
    }

    Ok(())
}

/// Creates a file/files in the passed path.
/// `file_paths` - takes a list of paths of what you want to create.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `file_paths` contains file or directory does not exist.
/// - Param `file_paths` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::touch;
///
///  let file_1 = std::path::Path::new("./file-1.txt").to_path_buf();
///  let file_2 = std::path::Path::new("./file-2.txt").to_path_buf();
///  let files: Vec<&std::path::PathBuf> = vec![&file_1, &file_2];
///
///  touch(&files)?;
/// ```
pub fn touch(file_paths: &Vec<&PathBuf>) -> Result<()> {
    if file_paths.len() <= 0 {
        let err_msg = format!("file_paths param is empty - {:?}", file_paths);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    Ok(utils::create(file_paths, &utils::CreateType::File)?)
}

/// Creates a directory/directories on the passed path. **Note**, the function
/// creates all missing directories if they occur in the passed parameter.
/// `dir_paths` - takes a list of paths of what you want to create.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `dir_paths` contains file or directory does not exist.
/// - Param `dir_paths` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::mkdir;
///
///  let dir_1 = std::path::Path::new("./dir1").to_path_buf();
///  let dir_2 = std::path::Path::new("./dir2").to_path_buf();
///  let dirs: Vec<&std::path::PathBuf> = vec![&dir_1, &dir_2];
///
///  mkdir(&dirs)?;
/// ```
pub fn mkdir(dir_paths: &Vec<&PathBuf>) -> Result<()> {
    if dir_paths.len() <= 0 {
        let err_msg = format!("dir_paths param is empty - {:?}", dir_paths);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    Ok(utils::create(dir_paths, &utils::CreateType::Dir)?)
}

/// Copies files and directories, including nested files and directories.
/// `from` - takes a list of paths of what you want to copy.
/// `to` - destination path.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `from` contains file or directory does not exist.
/// - Param `from` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::cp;
///
///  let file = std::path::Path::new("./file.txt").to_path_buf();
///  let dir = std::path::Path::new("./dir").to_path_buf();
///  let elements: Vec<&std::path::PathBuf> = vec![&file, &dir];
///  let to = std::path::Path::new("./tests/expected_files").to_path_buf();
///
///  cp(&elements, &to)?;
/// ```
pub fn cp(from: &Vec<&PathBuf>, to: &PathBuf) -> Result<u64> {
    if from.len() <= 0 {
        let err_msg = format!("from param is empty - {:?}", from);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    if from.len() == 1 && from[0].is_file() {
        let file_name = &from[0].file_name().unwrap();
        let path_to = &to.join(file_name);

        Ok(copy(&from[0], &path_to)?)
    } else {
        for path in from {
            if path.is_file() {
                let from: Vec<&PathBuf> = vec![path];
                cp(&from, &to)?;
            } else {
                let dir_name = path.file_name().unwrap();
                let path_to = &to.join(dir_name);
                create_dir(&path_to)?;

                for i in ls(&path).unwrap() {
                    let from: Vec<&PathBuf> = vec![&i];
                    cp(&from, &path_to)?;
                }
            }
        }

        Ok(1)
    }
}

/// Moves files and directories, including nested files and directories.
/// `from` - takes a list of paths of what you want to copy.
/// `to` - destination path.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `from` contains file or directory does not exist.
/// - Param `from` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::mv;
///
///  let file = std::path::Path::new("./file.txt").to_path_buf();
///  let dir = std::path::Path::new("./dir").to_path_buf();
///  let elements: Vec<&std::path::PathBuf> = vec![&file, &dir];
///  let to = std::path::Path::new("./tests/expected_files").to_path_buf();
///
///  mv(&elements, &to)?;
/// ```
pub fn mv(from: &Vec<&PathBuf>, to: &PathBuf) -> Result<()> {
    if from.len() <= 0 {
        let err_msg = format!("from param is empty - {:?}", from);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    let copy = cp(&from, &to);
    Ok(match copy {
        Ok(_res) => rm(&from),
        Err(error) => panic!("Copying error: {:?}", error),
    }?)
}

/// Clears the directory/directories of all child files and directories on the
/// passed path.
/// `paths` - takes a list of paths of what you want to clean.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `paths` contains file or directory does not exist.
/// - Param `paths` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::clean;
///
///  let dir_1 = std::path::Path::new("./dir1").to_path_buf();
///  let dir_2 = std::path::Path::new("./dir2").to_path_buf();
///  let dirs: Vec<&std::path::PathBuf> = vec![&dir_1, &dir_2];
///
///  clean(&dirs)?;
/// ```
pub fn clean(paths: &Vec<&PathBuf>) -> Result<()> {
    if paths.len() <= 0 {
        let err_msg = format!("paths param is empty - {:?}", paths);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    if paths.len() == 1 {
        rm(&paths)?;
        let vec_of_paths = vec![paths[0]];
        Ok(mkdir(&vec_of_paths)?)
    } else {
        for i in paths {
            let elements: Vec<&PathBuf> = vec![i];
            clean(&elements)?;
        }
        Ok(())
    }
}

/// Extracts all files from the directory, including nested files.
/// `from` - takes a list of paths of where you want to extract files from.
/// `to` - destination path.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not
/// limited to just these case:
///
/// - Param `from` contains file or directory does not exist.
/// - Param `from` contains file or directory with invalid name.
/// - The current process does not have the permission to access to input
/// params.
///
/// # Example
///
/// ```rust,ignore
///  extern crate rfm;
///  use rfm::extract;
///
///  let dir = std::path::Path::new("./dir").to_path_buf();
///  let dir_2 = std::path::Path::new("./dir_2").to_path_buf();
///  let dirs: Vec<&std::path::PathBuf> = vec![&dir, &dir_2];
///  let to = std::path::Path::new("./tests/expected_files").to_path_buf();
///
///  extract(&dirs, &to)?;
/// ```
pub fn extract(from: &Vec<&PathBuf>, to: &PathBuf) -> Result<()> {
    if from.len() <= 0 {
        let err_msg = format!("from param is empty - {:?}", from);
        err!(err_msg, ErrorKind::InvalidInput);
    }

    for i in from {
        let paths = ls(i).ok().unwrap();

        if paths.len() > 0 {
            for p in paths {
                if p.is_file() {
                    let file = vec![&p];
                    cp(&file, &to).ok();
                } else {
                    let newfrom = vec![&p];
                    extract(&newfrom, &to).ok();
                }
            }
        }
    }

    Ok(())
}
