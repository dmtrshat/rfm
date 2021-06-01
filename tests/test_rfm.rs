extern crate rfm;
use std::{
    fs::{create_dir, remove_dir_all, remove_file, File},
    path::{Path, PathBuf},
};

static FILE_1: &str = "file-1.txt";
static FILE_2: &str = "file-2.txt";
static DIR_1: &str = "dir-1";
static DIR_2: &str = "dir-2";
static DIR_3: &str = "dir-3";
static DIR_FOO: &str = "foo";
static DIR_BAR: &str = "bar";
static DATA: &str = "./tests/testing/data";
static CLEAN: &str = "./tests/testing/clean";
static CP: &str = "./tests/testing/cp";
static LS: &str = "./tests/testing/ls";
static MKDIR: &str = "./tests/testing/mkdir";
static MV: &str = "./tests/testing/mv";
static RM: &str = "./tests/testing/rm";
static TOUCH: &str = "./tests/testing/touch";
static EXTRACT: &str = "./tests/testing/extract";

#[test]
fn test_ls() {
    let dir = std::path::Path::new(&DATA).to_path_buf();
    let empty_dir = std::path::Path::new(&LS).to_path_buf();

    if !empty_dir.exists() {
        create_dir(&empty_dir).ok();
    }

    let content_of_empty_dir = rfm::ls(&empty_dir).unwrap();
    let content_of_dir = rfm::ls(&dir).unwrap();

    assert!(
        !&content_of_dir.is_empty(),
        "ls of not empty dir should return list with content of dir"
    );
    assert!(
        &content_of_empty_dir.is_empty(),
        "ls of empty dir should return empty list"
    );
}

#[test]
fn test_cp() {
    let data_dir = Path::new(&DATA);
    let file_1 = data_dir.join(&FILE_1).to_path_buf();
    let file_2 = data_dir.join(&FILE_2).to_path_buf();
    let files = vec![&file_1, &file_2];
    let dir_1 = data_dir.join(&DIR_1).to_path_buf();
    let dir_2 = data_dir.join(&DIR_2).to_path_buf();
    let dirs = vec![&dir_1, &dir_2];

    let to_path = Path::new(&CP).to_path_buf();
    let expected_files: Vec<PathBuf> = vec![to_path.join(&FILE_1), to_path.join(&FILE_2)];
    let expected_dirs: Vec<PathBuf> = vec![to_path.join(&DIR_1), to_path.join(&DIR_2)];
    let expected_content_of_dirs: Vec<PathBuf> = vec![
        to_path.join(&DIR_1).join(&FILE_1),
        to_path.join(&DIR_1).join(&FILE_2),
        to_path.join(&DIR_2).join(&DIR_FOO),
        to_path.join(&DIR_2).join(&FILE_1),
        to_path.join(&DIR_2).join(&DIR_FOO).join(&DIR_BAR),
        to_path.join(&DIR_2).join(&DIR_FOO).join(&FILE_1),
        to_path
            .join(&DIR_2)
            .join(&DIR_FOO)
            .join(&DIR_BAR)
            .join(&FILE_1),
    ];

    remove_dir_all(&CP).ok();
    create_dir(&CP).ok();

    rfm::cp(&files, &to_path).ok();
    rfm::cp(&dirs, &to_path).ok();

    assert!(expected_files[0].exists(), "Copied file should exist");
    assert!(expected_files[1].exists(), "Copied file should exist");
    assert!(expected_dirs[0].exists(), "Copied dir should exist");

    assert!(
        expected_content_of_dirs[0].exists(),
        "Copied dirs content should exist"
    );
    assert!(
        expected_content_of_dirs[1].exists(),
        "Copied dirs content should exist"
    );
    assert!(
        expected_content_of_dirs[2].exists(),
        "Copied dirs content should exist"
    );
    assert!(
        expected_content_of_dirs[3].exists(),
        "Copied dirs content should exist"
    );
    assert!(
        expected_content_of_dirs[4].exists(),
        "Copied dirs content should exist"
    );
    assert!(
        expected_content_of_dirs[5].exists(),
        "Copied dirs content should exist"
    );
    assert!(
        expected_content_of_dirs[6].exists(),
        "Copied dirs content should exist"
    );
}

#[test]
fn test_rm() {
    let rm_dir = Path::new(&RM);
    let dir = rm_dir.join(&DIR_2).to_path_buf();
    let file = rm_dir.join(&FILE_1).to_path_buf();

    let elements = vec![&dir, &file];

    if !dir.exists() {
        create_dir(&dir).ok();
    }

    if !file.exists() {
        File::create(&file).ok();
    }

    rfm::rm(&elements).ok();
    assert!(!dir.exists(), "Deleted dir should not exist");
    assert!(!file.exists(), "Deleted file should not exist");
}

#[test]
fn test_mkdir() {
    let mkdir_dir = Path::new(&MKDIR);
    let dir_foo = mkdir_dir.join(&DIR_FOO);
    let few_dirs = mkdir_dir.join(&DIR_2).join(&DIR_FOO).join(&DIR_BAR);
    let dirs = vec![&dir_foo, &few_dirs];

    if few_dirs.exists() {
        remove_dir_all(&few_dirs).ok();
    }

    if dir_foo.exists() {
        remove_dir_all(&dir_foo).ok();
    }

    rfm::mkdir(&dirs).ok();
    assert!(dir_foo.exists(), "Created dir should exist");
    assert!(few_dirs.exists(), "Created dirs should exist");
}

#[test]
fn test_touch() {
    let touch_dir = Path::new(&TOUCH);
    let file_1 = touch_dir.join(&FILE_1);
    let file_2 = touch_dir.join(&FILE_2);
    let files = vec![&file_1, &file_2];

    if !touch_dir.exists() {
        create_dir(&touch_dir).ok();
    }

    if file_1.exists() {
        remove_file(&file_1).ok();
    }

    if file_2.exists() {
        remove_file(&file_2).ok();
    }

    rfm::touch(&files).ok();
    assert!(file_1.exists(), "Created file should exist");
    assert!(file_2.exists(), "Created file should exist");
}

#[test]
fn test_clean() {
    let clean_dir = Path::new(&CLEAN).to_path_buf();
    let file_1 = clean_dir.join(&FILE_1);
    let file_2 = clean_dir.join(&FILE_2);
    let dir_1 = clean_dir.join(&DIR_1);

    if !clean_dir.exists() {
        create_dir(&clean_dir).ok();
    }

    File::create(&file_1).ok();
    File::create(&file_2).ok();
    create_dir(&dir_1).ok();

    rfm::clean(&vec![&clean_dir]).ok();
    assert!(!file_1.exists(), "Cleaned dir should not exist any files");
    assert!(!file_2.exists(), "Cleaned dir should not exist any files");
    assert!(!dir_1.exists(), "Cleaned dir should not exist any dirs");
}

#[test]
fn test_mv() {
    let mv_dir = Path::new(&MV);
    let dir_1 = mv_dir.join(&DIR_1);
    let dir_2 = mv_dir.join(&DIR_2);
    let file_1 = mv_dir.join(&DIR_2).join(&FILE_1);

    let expected_dir = dir_1.join(&DIR_2);
    let expected_file = dir_1.join(&DIR_2).join(&FILE_1);

    let need_to_mv = vec![&dir_2];

    remove_dir_all(&mv_dir).ok();
    create_dir(&mv_dir).ok();

    for dir in [&dir_1, &dir_2].iter() {
        if !dir.exists() {
            create_dir(dir).ok();
        }
    }

    if !file_1.exists() {
        File::create(&file_1).ok();
    }

    rfm::mv(&need_to_mv, &dir_1).ok();
    assert!(
        expected_dir.exists(),
        "Files/dirs should exist at the endpoint"
    );
    assert!(
        expected_file.exists(),
        "Files/dirs should exist at the endpoint"
    );
    assert!(
        !dir_2.exists(),
        "The files/directories must not be at the starting point after the transfer"
    );
}

#[test]
fn test_extract() {
    let data_dir = Path::new(&DATA);
    let extract_dir = Path::new(&EXTRACT).to_path_buf();
    let dir_3 = data_dir.join(&DIR_3);
    let need_to_extract = vec![&dir_3];

    let expected_file_1 = extract_dir.join("file-1.txt");
    let expected_file_2 = extract_dir.join("file-2.txt");
    let expected_file_3 = extract_dir.join("file-3.txt");
    let expected_file_4 = extract_dir.join("file-4.txt");

    if extract_dir.exists() {
        remove_dir_all(&extract_dir).ok();
    }

    create_dir(&extract_dir).ok();

    rfm::extract(&need_to_extract, &extract_dir).ok();
    assert!(extract_dir.exists(), "Extract dir should exist");
    assert!(
        expected_file_1.exists(),
        "File should exist after extraction"
    );
    assert!(
        expected_file_2.exists(),
        "File should exist after extraction"
    );
    assert!(
        expected_file_3.exists(),
        "File should exist after extraction"
    );
    assert!(
        expected_file_4.exists(),
        "File should exist after extraction"
    );
}
