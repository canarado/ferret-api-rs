use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use std::path::PathBuf;
use std::env;

#[derive(Debug, Serialize)]
pub struct Ferret {
    id: String,
    path: String
}

pub fn add_ferret_to_db(id: Uuid, path: String) -> Result<()> {
    let db_path = PathBuf::from(env::var("SQLITE_DB_FILE").unwrap());
    let conn = Connection::open(db_path).unwrap();

    conn.execute("
        INSERT OR IGNORE INTO ferret(id, path) VALUES(?, ?)
    ", (&id.to_string(), &path))?;

    Ok(())
}

pub fn get_random_ferret() -> Result<Ferret> {
    let db_path = PathBuf::from(env::var("SQLITE_DB_FILE").unwrap());
    let conn = Connection::open(db_path).unwrap();

    let res = conn.query_row("SELECT * FROM ferret ORDER BY RANDOM() LIMIT 1", [], |row|
        Ok(Ferret { id: row.get(0)?, path: row.get(1)?})
    )?;

    Ok(res)
}

// pub fn get_ferret_by_id(id: String) -> Result<()> {
//     let conn = Connection::open(".\\private\\database.db").unwrap();

//     let mut stmt = conn.prepare("SELECT id, path FROM ferret WHERE id=?")?;

//     //let res = stmt.execute([&id])?;
//     let res = stmt.query_row([&id],|row|
//         Ok(Ferret { id: row.get(0)?, path: row.get(1)?}
//     ))?;

//     Ok(())
// }

pub fn prepare_database() -> Result<()> {
    let db_path = PathBuf::from(env::var("SQLITE_DB_FILE").unwrap());
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ferret(
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL
            )",
            ()
    )?;

    Ok(())
}