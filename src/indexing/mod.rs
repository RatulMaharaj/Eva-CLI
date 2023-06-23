pub mod indexing;
use indexing::index_folder;
use rusqlite::{params, Connection};
use std::collections::HashMap;

use self::indexing::FileOrDir;

pub fn update() {
    let conn = Connection::open("eva.db").unwrap();
    println!("Updating indexes. Please be patient.");

    // fetch list of folders to index from database
    let folder_list = fetch_folders(&conn);

    // loop over folders and index each one
    for (id, folder) in folder_list.iter() {
        // drop existing results for the given id
        drop_entries(&conn, id);
        let _res = index_folder(folder);
        // insert results into database
        let conn = Connection::open("eva.db").unwrap();

        for file in _res.iter() {
            //  print each file item
            add_entry(&conn, file, id);
        }
    }

    println!("Updating of indexes complete.");
}

fn add_entry(conn: &Connection, entry: &FileOrDir, id: &i32) {
    match conn.execute(
        "INSERT INTO results(path, name, is_folder, size_bytes, accessed_time, modified_time, created_time, readonly, hidden, system, folder_size_bytes, num_files, num_subfolders, folderId) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![entry.path, entry.name, entry.is_dir, entry.size, entry.accessed, entry.modified, entry.created, entry.read_only, entry.hidden, entry.system, entry.folder_size, entry.num_files, entry.num_folders, id],
    ) {
        Ok(_updated) => (),
        Err(err) => println!("Update Failed! {}", err),
    }
}

fn drop_entries(conn: &Connection, folder_id: &i32) {
    conn.execute(
        "DELETE FROM results WHERE folderId = ?1",
        params![folder_id],
    )
    .unwrap();
}

pub fn add_folder(folder: &String) {
    let conn = Connection::open("eva.db").unwrap();
    println!("Adding folder \"{}\" to index list.", folder);
    let mut folder_list = get_folders(&conn);

    if !folder_list.contains(&folder) {
        conn.execute(
            "INSERT INTO folders_to_index (folder) values (?1)",
            &[folder],
        )
        .unwrap();
        println!("Added folder {} to index list!", &folder);
        folder_list = get_folders(&conn);
    } else {
        println!("Folder already in index list.")
    }

    println!("The current index list is: {:#?}", folder_list);
}

pub fn get_folders(conn: &Connection) -> Vec<String> {
    // fetch folders from database
    let folders = fetch_folders(&conn);

    // convert to vector
    let mut folder_list = Vec::new();
    for (_id, folder) in folders.iter() {
        folder_list.push(folder.to_string());
    }

    // return vector of strings
    return folder_list;
}

pub fn ls_folders() {
    // Establish database connection
    let conn = Connection::open("eva.db").unwrap();
    // fetch folders in list format
    let folder_list = get_folders(&conn);
    // print the list of folders
    println!("The current index list is: {:#?}", folder_list);
}

pub fn remove_folder(folder: &String) {
    let conn = Connection::open("eva.db").unwrap();
    println!("Removing folder \"{}\" from index list.", folder);
    let folders = fetch_folders(&conn);

    // get the key of the folder to remove
    let mut folder_id = 0;
    for (id, f) in folders.iter() {
        if f == folder {
            folder_id = *id;
        }
    }

    println!("Folder id is: {}", folder_id);

    if folders.contains_key(&folder_id) {
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
        conn.execute("DELETE FROM folders_to_index WHERE id = ?1;", [folder_id])
            .unwrap();
        println!("Removed folder {} from index list!", &folder);
    } else {
        println!("Folder not in index list.")
    }

    let folder_list = get_folders(&conn);
    println!("The current index list is:\n{:#?}", folder_list);
}

#[derive(Serialize, Deserialize, Debug)]
struct Folder {
    id: i32,
    folder: String,
}
pub fn fetch_folders(conn: &Connection) -> HashMap<i32, String> {
    let mut stmt = conn
        .prepare("SELECT id, folder FROM folders_to_index")
        .unwrap();

    let folder_iter = stmt
        .query_map(params![], |row| {
            Ok(Folder {
                id: row.get(0)?,
                folder: row.get(1)?,
            })
        })
        .unwrap();

    let mut folder_list = HashMap::new();

    for folder in folder_iter {
        let f = folder.unwrap();
        folder_list.insert(f.id, f.folder);
    }

    return folder_list;
}
