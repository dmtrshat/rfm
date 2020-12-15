use std::{
    fs::{create_dir, File},
    io::Result,
    path::{Path, PathBuf},
};

pub enum CreateType {
    Dir,
    File,
}

fn get_vec_of_nonexistent_dirs(path: &PathBuf) -> Vec<PathBuf> {
    let mut str_path = String::from(path.to_str().unwrap());
    let mut missing_dirs: Vec<PathBuf> = vec![];

    while !Path::new(&str_path).parent().unwrap().exists() {
        str_path = str_path.replace(
            Path::new(&str_path).file_name().unwrap().to_str().unwrap(),
            "",
        );
        missing_dirs.push(Path::new(&str_path.as_str()).to_path_buf());
    }

    missing_dirs.iter().rev().cloned().collect()
}

pub fn create(paths: &Vec<&PathBuf>, create_type: &CreateType) -> Result<()> {
    if paths.len() == 1 {
        match create_type {
            CreateType::Dir => {
                let missing_dirs: Vec<PathBuf> = get_vec_of_nonexistent_dirs(paths[0]);
                if missing_dirs.len() > 0 {
                    for dir in missing_dirs {
                        create_dir(&dir)?;
                    }
                }
                Ok(create_dir(paths[0])?)
            }
            CreateType::File => {
                File::create(&paths[0])?;
                Ok(())
            }
        }
    } else {
        for path in paths {
            let vec_of_path: Vec<&PathBuf> = vec![path];
            create(&vec_of_path, &create_type).ok();
        }
        Ok(())
    }
}
