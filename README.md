# **R**ust **F**ile **M**anager

> File manager, only inside Rust 🦀

**`rfm`** is a convenient and intuitive way to interact with files on OS. Function naming is similar to Unix commands _(`ls`, `mkdir`, `mv`, `rm`, `cp`, `touch` etc.)_, so you can easily figure out how to use it.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rfm = "0.5.0"
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
    let dir_from_1 = Path::new("./tests/testing/mv").to_path_buf();
    let dir_from_2 = Path::new("./tests/testing/cp").to_path_buf();

    let dir_to = Path::new("./tests/testing/mkdir").to_path_buf();

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
| [rfm::mkdir]() | Creates a directory/directories on the passed path **Note**, the function creates all missing directories if they occur in the passed parameter. `dir_paths` - takes a list of paths of what you want to create. |
| [rfm::touch]() | Creates a file/files in the passed path. `file_paths` - takes a list of paths of what you want to create. |
| [rfm::cp]() | Copies files and directories, including nested files and directories. `from` - takes a list of paths of what you want to copy. `to` - destination path. |
| [rfm::file::mv]() | Moves files and directories, including nested files and directories. `from` - takes a list of paths of what you want to copy. `to` - destination path. |
| [rfm::file::rm]() | Deletes files/directories (including nested files/directories). `from` - takes a list of paths of what you want to delete. |

## Contributing

1. Fork it!
2. Create your feature branch: `git checkout -b my-new-feature`
3. Commit your changes: `git commit -am "Add some feature"`
4. Push to the branch: `git push origin my-new-feature`
5. Submit a pull request
6. Stay Rusty 🦀

## License

[MIT](LICENSE)

Made by [Dmitry Shatokhin](https://github.com/dmtrshat) with love ❤️
