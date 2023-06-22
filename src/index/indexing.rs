use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[derive(Debug)]
pub struct FileOrDir {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub created: u64,
    pub accessed: u64,
    pub modified: u64,
    pub size: u64,
    pub read_only: bool,
    pub hidden: bool,
    pub system: String,
    pub folder_size: u64,
    pub num_files: u64,
    pub num_folders: u64,
}

pub fn index_folder(folder: &str) -> Vec<FileOrDir> {
    let mut files = Vec::new();
    let walker = WalkDir::new(folder).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        // dir item
        let item = entry.as_ref().unwrap();
        // extract meta
        let meta = item.metadata().unwrap();

        let file_dir_item = FileOrDir {
            name: String::from(item.file_name().to_string_lossy()),
            path: String::from(item.path().to_str().unwrap()),
            is_dir: meta.is_dir(),
            created: meta.accessed().unwrap().elapsed().unwrap().as_secs(),
            accessed: meta.created().unwrap().elapsed().unwrap().as_secs(),
            modified: meta.modified().unwrap().elapsed().unwrap().as_secs(),
            size: meta.len(),
            read_only: meta.permissions().readonly(),
            hidden: false,
            system: String::from("unknown"),
            folder_size: 0,
            num_files: 0,
            num_folders: 0,
        };

        // add hashmap to vector
        files.push(file_dir_item);
    }
    files
}
