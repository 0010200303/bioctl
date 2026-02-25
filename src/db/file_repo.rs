use rusqlite::{Connection, Result, params};
use crate::domain::file::File;

pub fn insert_file(connection: &Connection, path: &str, created_at: i64) -> Result<Option<i64>> {
    match connection.query_one(
        "INSERT OR IGNORE INTO files (path, created_at) VALUES (?1, ?2) RETURNING id",
        params![path, created_at],
        |row| row.get(0),
    ) {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn delete_file(connection: &Connection, file_id: i64) -> Result<Option<String>> {
    match connection.query_one(
        "DELETE FROM files WHERE id = ?1 RETURNING path",
        params![file_id],
        |row| row.get(0),
    ) {
        Ok(name) => Ok(Some(name)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn fetch_file(connection: &Connection, file_id: i64) -> Result<Option<File>> {
    match connection.query_one(
        "SELECT id, path, created_at FROM files WHERE id = ?1",
        params![file_id],
        |row| {
            Ok(File {
                id: row.get(0)?,
                path: row.get(1)?,
                created_at: row.get(2)?,
            })
        },
    ) {
        Ok(file) => Ok(Some(file)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn fetch_files(connection: &Connection) -> Result<Vec<File>> {
    let mut statement = connection.prepare(
        "SELECT id, path, created_at FROM files",
    )?;
    let files = statement
        .query_map([], |row| {
            Ok(File {
                id: row.get(0)?,
                path: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<File>, _>>()?;
    Ok(files)
}

pub fn find_file_by_path(connection: &Connection, path: &str) -> Result<Option<File>> {
    match connection.query_one(
        "SELECT id, path, created_at FROM files WHERE path = ?1",
        params![path],
        |row| {
            Ok(File {
                id: row.get(0)?,
                path: row.get(1)?,
                created_at: row.get(2)?,
            })
        },
    ) {
        Ok(file) => Ok(Some(file)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
