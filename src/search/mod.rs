use crate::indexing::indexing::FileOrDir;

use rusqlite::{params, Connection};

pub fn search(param: &String) -> Vec<FileOrDir> {
    let conn = Connection::open("eva.db").unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT path,
                name,
                is_folder,
                size_bytes,
                accessed_time,
                modified_time,
                created_time,
                readonly,
                hidden,
                system,
                folder_size_bytes,
                num_files,
                num_subfolders 
            FROM results WHERE name LIKE ?1",
        )
        .unwrap();

    let folder_iter = stmt
        .query_map(params![format!("%{}%", param)], |row| {
            Ok(FileOrDir {
                name: row.get(0)?,
                path: row.get(1)?,
                is_dir: row.get(2)?,
                created: row.get(3)?,
                accessed: row.get(4)?,
                modified: row.get(5)?,
                size: row.get(6)?,
                read_only: row.get(7)?,
                hidden: row.get(8)?,
                system: row.get(9)?,
                folder_size: row.get(10)?,
                num_files: row.get(11)?,
                num_folders: row.get(12)?,
            })
        })
        .unwrap();

    // create a vector of FileOrDir structs
    let results = folder_iter
        .map(|folder| folder.unwrap())
        .collect::<Vec<FileOrDir>>();

    return results;
}
