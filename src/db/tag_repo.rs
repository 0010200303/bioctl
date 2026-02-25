use rusqlite::{Connection, Result, params};
use crate::domain::tag::Tag;

pub fn insert_tag(connection: &Connection, tag_name: &str) -> Result<Option<i64>> {
    match connection.query_one(
        "INSERT OR IGNORE INTO tags (name) VALUES(?1) RETURNING id",
        params![tag_name],
        |row| row.get(0),
    ) {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn delete_tag(connection: &Connection, tag_id: i64) -> Result<Option<String>> {
    match connection.query_one(
        "DELETE FROM tags WHERE id = ?1 RETURNING name",
        params![tag_id],
        |row| row.get(0),
    ) {
        Ok(name) => Ok(Some(name)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn fetch_tag(connection: &Connection, tag_id: i64) -> Result<Option<Tag>> {
    match connection.query_one(
        "SELECT id, name from tags WHERE id = ?1",
        params![tag_id],
        |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        },
    ) {
        Ok(tag) => Ok(Some(tag)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn fetch_tags(connection: &Connection) -> Result<Vec<Tag>> {
    let mut statement = connection.prepare(
        "SELECT id, name FROM tags",
    )?;
    let tags = statement
        .query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<Tag>, _>>()?;
    Ok(tags)
}

pub fn find_tag_by_name(connection: &Connection, tag_name: &str) -> Result<Option<Tag>> {
    match connection.query_one(
        "SELECT id, name FROM tags WHERE name = ?1",
        params![tag_name],
        |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        },
    ) {
        Ok(tag) => Ok(Some(tag)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
