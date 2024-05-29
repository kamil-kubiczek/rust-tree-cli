// todo
// 1. scan dirs
// 2. scan files
// 3. loop with depth through files
// 4. display as tree in cli

use std::{borrow::Borrow, ffi::OsString, fs, path::Path};

#[derive(Debug)]
struct FileTree {
    dirs: Vec<Directory>,
}

impl FileTree {
    fn display_tree(&self) {
        for dir in self.dirs.iter() {
            Directory::display_directory(dir, 1)
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: Box<OsString>,
    directories: Option<Vec<Directory>>,
    files: Option<Vec<OsString>>,
}

impl Directory {
    fn display_directory(dirs: &Directory, depth: i32) {
        let mut dirs_spacing = String::from("");
        for _ in 1..depth {
            dirs_spacing += "-";
        }

        let mut files_spacing = String::from("");
        for _ in 1..depth {
            files_spacing += "   ";
        }

        println!(
            "{} {}",
            dirs_spacing,
            dirs.name
                .clone()
                .into_string()
                .unwrap()
                .as_mut()
                .replace("\"", "")
        );

        if let Some(files) = dirs.files.borrow() {
            for file in files {
                println!(
                    "{} |{}",
                    files_spacing,
                    file.clone()
                        .into_string()
                        .unwrap()
                        .as_mut()
                        .replace("\"", "")
                );
            }
        }

        if let Some(dirs) = dirs.directories.borrow() {
            for dir in dirs {
                Directory::display_directory(dir, depth + 1);
            }
        }
    }
}

fn main() {
    // scan for dirs
    let path = Path::new("./");
    let tree = create_tree(&path);
    tree.display_tree()
}

fn create_tree(path: &Path) -> FileTree {
    let mut tree = FileTree { dirs: vec![] };

    if let Some(dirs) = find_directories(path) {
        tree.dirs = dirs;
    }

    tree
}

fn find_directories(path: &Path) -> Option<Vec<Directory>> {
    if let Ok(entries) = fs::read_dir(path) {
        let mut directories: Vec<Directory> = vec![];

        directories.push(Directory {
            name: Box::new(
                path.file_name()
                    .unwrap_or(OsString::from(".").as_os_str())
                    .to_os_string(),
            ),
            directories: None,
            files: None,
        });

        for entry in entries {
            match entry {
                Err(e) => {
                    println!("entry error {}", e);
                    return None;
                }
                Ok(entry) => {
                    let entry_path = entry.path();

                    if entry_path.is_dir() {
                        let nested_directories = find_directories(&entry_path).unwrap();
                        let files = find_files(&entry_path)?;

                        directories.push(Directory {
                            name: Box::new(entry_path.file_name().unwrap().to_owned()),
                            directories: Some(nested_directories),
                            files: Some(files),
                        })
                    } else if entry_path.is_file() {
                        if let Some(files) = directories[0].files.as_mut() {
                            files.push(entry_path.file_name().unwrap().to_os_string());
                        } else {
                            let last_index = directories.len() - 1;
                            if let Some(files) = directories[last_index].files.as_mut() {
                                files.push(entry_path.file_name().unwrap().to_os_string());
                            }
                        }
                    }
                }
            }
        }
        return Some(directories);
    } else {
        None
    }
}

fn find_files(path: &Path) -> Option<Vec<OsString>> {
    if path.is_dir() {
        let mut files = vec![];

        if let Ok(direcories_iter) = path.read_dir() {
            for dir in direcories_iter {
                match dir {
                    Ok(dir) => {
                        if dir.path().is_file() {
                            files.push(dir.path().file_name().unwrap().to_os_string())
                        }
                    }
                    Err(e) => println!("Error when finding folders in path {:?}", e),
                }
            }
        }

        Some(files)
    } else {
        None
    }
}
