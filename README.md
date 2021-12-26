# **R**ust **F**ile **M**anager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://travis-ci.com/dmtrshat/rfm.svg?branch=main)](https://travis-ci.com/dmtrshat/rfm)
[![Crates.io Status](https://img.shields.io/crates/v/rfm.svg)](https://crates.io/crates/rfm)
[![Docs](https://docs.rs/rfm/badge.svg)](https://docs.rs/rfm)

> File manager, only inside Rust 🦀

**`rfm`** is a convenient and intuitive way to interact with files on OS. Function naming is similar to Unix commands _(`ls`, `mkdir`, `mv`, `rm`, `cp`, `touch` etc.)_, so you can easily figure out how to use it.

## Installation

Add the latest rfm version to your `Cargo.toml`:

```toml
[dependencies]
rfm = "X.Y.Z" #replace it with the current version
```

## Usage

For example:

```rust
extern crate rfm;
use std::{
    io::Result,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let dir_from_1 = Path::new("./foo").to_path_buf();
    let dir_from_2 = Path::new("./bar").to_path_buf();

    let dir_to = Path::new("./baz").to_path_buf();

    // check dir for existing files/dirs
    if rfm::ls(&dir_to)?.len() > 0 {
        // clean dir
        rfm::clean(&vec![&dir_to])?;
    }

    let need_to_move: Vec<&PathBuf> = vec![&dir_from_1, &dir_from_2];
    // Move some files/dirs
    rfm::mv(&need_to_move, &dir_to)?;

    Ok(())
}
```

## Functions:

| Function | Description |
| ----------------- | ----------------------------------------------------------------------------------------------------- |
| [rfm::ls]() | Read the directory/directories and return the content. `dir` - takes the path to the directory whose contents you want to retrieve. |
| [rfm::clean]() |  Clears the directory/directories of all child files and directories on the passed path. `paths` - takes a list of paths of what you want to clean. |
| [rfm::mkdir]() | Creates a directory/directories on the passed path. **Note**, the function creates all missing directories if they occur in the passed parameter. `dir_paths` - takes a list of paths of what you want to create. |
| [rfm::touch]() | Creates a file/files in the passed path. `file_paths` - takes a list of paths of what you want to create. |
| [rfm::cp]() | Copies files and directories, including nested files and directories. `from` - takes a list of paths of what you want to copy. `to` - destination path. |
| [rfm::mv]() | Moves files and directories, including nested files and directories. `from` - takes a list of paths of what you want to copy. `to` - destination path. |
| [rfm::rm]() | Deletes files/directories (including nested files/directories). `from` - takes a list of paths of what you want to delete. |
| [rfm::extract]() | Extracts all files from the directory, including nested files. `from` - takes a list of paths of where you want to extract files from. `to` - destination path. |
| [rfm::get_size]() | Returns the size of a file or directory in bytes, `path` - the path to the directory/file whose size you want to get. |

## License

[MIT](LICENSE)

---
Stay Rusty 🦀

Made by [Dmitry Shatokhin](https://github.com/dmtrshat) with love ❤️
