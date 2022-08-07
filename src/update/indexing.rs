use std::{collections::HashMap, path::PathBuf, time::SystemTime};
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[derive(Debug)]
pub enum FileOrDir {
    Name(String),
    Dir(PathBuf),
    IsDir(bool),
    Created(SystemTime),
    Accessed(SystemTime),
    Modified(SystemTime),
    Size(u64),
    ReadOnly(bool),
    Hidden(bool),
    System(String),
    FolderSize(u64),
    NumFiles(u64),
    NumFolders(u64),
}

pub fn index_folder(folder: &str) -> Vec<HashMap<&str, FileOrDir>> {
    let mut files = Vec::new();
    let walker = WalkDir::new(folder).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let mut file = HashMap::new();
        let item = entry.as_ref().unwrap();

        // Populate hashmap
        file.insert(
            "name",
            FileOrDir::Name(String::from(item.file_name().to_string_lossy())),
        );
        file.insert("path", FileOrDir::Dir(item.path().to_path_buf()));

        let meta = item.metadata().unwrap();
        file.insert("is_folder", FileOrDir::IsDir(meta.is_dir()));
        file.insert(
            "accessed_time",
            FileOrDir::Accessed(meta.accessed().unwrap()),
        );
        file.insert(
            "modified_time",
            FileOrDir::Modified(meta.modified().unwrap()),
        );
        file.insert("created_time", FileOrDir::Created(meta.created().unwrap()));
        file.insert("size_bytes", FileOrDir::Size(meta.len()));
        file.insert(
            "readonly",
            FileOrDir::ReadOnly(meta.permissions().readonly()),
        );
        file.insert("hidden", FileOrDir::Hidden(false));

        // TODO: figure out how to include these metrics
        file.insert("system", FileOrDir::System("unknown".to_string()));
        file.insert("folder_size_bytes", FileOrDir::FolderSize(0));
        file.insert("num_files", FileOrDir::NumFiles(0));
        file.insert("num_subfolders", FileOrDir::NumFolders(0));

        // add hashmap to vector
        files.push(file);
    }
    files
}
