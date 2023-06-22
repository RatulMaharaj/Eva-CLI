use rusqlite::{params, Connection, Result};

pub fn create_tables() -> Result<()> {
    let conn = Connection::open("eva.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders_to_index (
             id INTEGER PRIMARY KEY,
             folder TEXT NOT NULL UNIQUE
         );",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS results (
             id INTEGER PRIMARY KEY,
             path TEXT,
             name TEXT,
             is_folder NUMERIC,
             size_bytes NUMERIC,
             accessed_time DATETIME,
             modified_time DATETIME,
             created_time DATETIME,
             readonly NUMERIC,
             hidden NUMERIC,
             system TEXT,
             folder_size_bytes NUMERIC,
             num_files NUMERIC,
             num_subfolders NUMERIC,
             folderId INTEGER,
             CONSTRAINT fk_folders_to_index
                FOREIGN KEY (folderId) 
                REFERENCES folders_to_index(id)
                ON DELETE CASCADE
         );",
        params![],
    )?;

    conn.close().ok();

    Ok(())
}
