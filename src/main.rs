use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
    fs,
    path::Path,
    vec,
};

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
    name: OsString,
    directories: Option<Vec<Directory>>,
    files: Option<Vec<OsString>>,
}

impl Directory {
    fn display_directory(dirs: &Directory, depth: i32) {
        let mut dirs_spacing = String::from("");
        for _ in 0..depth {
            dirs_spacing.insert_str(0, "───");
        }

        let mut tabs_spacing = String::from("");

        for _ in 0..depth {
            tabs_spacing.push_str("   ")
        }

        println!(
            "{}└──{}",
            tabs_spacing,
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
                    "{}└──{}",
                    tabs_spacing,
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

    // parent dir
    tree.dirs.push(Directory {
        name: path
            .file_name()
            .unwrap_or(OsStr::new("Error when reading root"))
            .to_owned(),
        directories: Some(vec![]),
        files: Some(vec![]),
    });

    if let Some(dirs) = find_directories(path) {
        tree.dirs[0].directories = Some(dirs);
    }

    if let Some(files) = find_files(path) {
        tree.dirs[0].files = Some(files);
    }

    tree
}

fn find_directories(path: &Path) -> Option<Vec<Directory>> {
    if let Ok(entries) = fs::read_dir(path) {
        let mut directories: Vec<Directory> = vec![];

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
                            name: entry_path.file_name().unwrap().to_owned(),
                            directories: Some(nested_directories),
                            files: Some(files),
                        });
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
                    Err(e) => println!("Error when finding files in path {:?}", e),
                }
            }
        }

        Some(files)
    } else {
        None
    }
}
