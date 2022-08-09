use rusqlite::{params, Connection};

#[derive(Debug)]
struct Folder {
    id: i32,
    folder: String,
}

pub fn fetch_folders(conn: &Connection) -> Vec<String> {
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

    let mut folder_list = Vec::new();

    for folder in folder_iter {
        let f = folder.unwrap();
        let _id = f.id;
        folder_list.push(f.folder);
    }

    return folder_list;
}

pub fn add_folder(folder: &String) {
    let conn = Connection::open("eva.db").unwrap();
    println!("Adding folder \"{}\" to index list.", folder);
    let mut folder_list = fetch_folders(&conn);

    if !folder_list.contains(&folder) {
        conn.execute(
            "INSERT INTO folders_to_index (folder) values (?1)",
            &[folder],
        )
        .unwrap();
        println!("Added folder {} to index list!", &folder);
        folder_list = fetch_folders(&conn);
    } else {
        println!("Folder already in index list.")
    }

    println!("The current index list is: {:?}", folder_list);
}

pub fn remove_folder(folder: &String) {
    let conn = Connection::open("eva.db").unwrap();
    println!("Removing folder \"{}\" from index list.", folder);
    let mut folder_list = fetch_folders(&conn);

    if folder_list.contains(&folder) {
        conn.execute(
            "DELETE FROM folders_to_index WHERE folder = (?1)",
            &[folder],
        )
        .unwrap();
        println!("Removed folder {} from index list!", &folder);
        folder_list = fetch_folders(&conn);
    } else {
        println!("Folder not in index list.")
    }

    println!("The current index list is: {:?}", folder_list);
}
