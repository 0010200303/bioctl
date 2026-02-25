use crate::domain::group::Group;
use crate::domain::file::File;
use crate::domain::tag::Tag;
use rusqlite::{params, Connection, Result};

pub fn insert_group(
    connection: &Connection,
    group_name: &str,
    created_at: i64,
    description: &str,
) -> Result<Option<i64>> {
    match connection.query_one(
        "INSERT INTO groups (name, created_at, description) VALUES (?1, ?2, ?3) RETURNING id",
        params![group_name, created_at, description],
        |row| row.get(0),
    ) {
        Ok(id) => Ok(Some(id)),
        Err(e) => Err(e),
    }
}

pub fn delete_group(connection: &Connection, group_id: i64) -> Result<()> {
    connection.execute(
        "DELETE FROM groups WHERE id = ?1",
        params![group_id],
    )?;
    Ok(())
}

pub fn fetch_groups(connection: &Connection) -> Result<Vec<Group>> {
    let mut statement =
        connection.prepare("SELECT id, name, created_at, description FROM groups")?;
    let groups = statement
        .query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                description: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<Group>, _>>()?;
    Ok(groups)
}

pub fn find_group_id(connection: &Connection, group_name: &str) -> Result<Option<i64>> {
    match connection.query_one(
        "SELECT id FROM groups WHERE name = ?1",
        params![group_name],
        |row| row.get(0),
    ) {
        Ok(id) => Ok(Some(id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn insert_file(connection: &Connection, group_id: i64, file_id: i64) -> Result<()> {
    connection.execute(
        "
        INSERT OR IGNORE INTO group_files (group_id, file_id)
        VALUES (?1, ?2)
        ",
        params![group_id, file_id],
    )?;
    Ok(())
}

pub fn delete_file(connection: &Connection, group_id: i64, file_id: i64) -> Result<()> {
    connection.execute(
        "DELETE FROM group_files WHERE group_id = ?1 AND file_id = ?2",
        params![group_id, file_id],
    )?;
    Ok(())
}

pub fn fetch_group(connection: &Connection, group_id: i64) -> Result<Option<Group>> {
    match connection.query_one(
        "SELECT id, name, created_at, description FROM groups WHERE id = ?1",
        params![group_id],
        |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                description: row.get(3)?,
            })
        },
    ) {
        Ok(group) => Ok(Some(group)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn fetch_files(connection: &Connection, group_id: i64) -> Result<Vec<File>> {
    let mut statement = connection.prepare(
        "
        SELECT f.id, f.path, f.created_at
        FROM files AS f
        JOIN group_files AS gf ON f.id = gf.file_id
        WHERE gf.group_id = ?1
        ",
    )?;
    let files = statement
        .query_map(params![group_id], |row| {
            Ok(File {
                id: row.get(0)?,
                path: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<File>, _>>()?;
    Ok(files)
}

pub fn has_file(connection: &Connection, group_id: i64, file_id: i64) -> Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM group_files WHERE group_id = ?1 AND file_id = ?2)",
        params![group_id, file_id],
        |row| row.get(0),
    )
}

pub fn would_create_cycle(
    connection: &Connection,
    parent_group_id: i64,
    child_group_id: i64,
) -> Result<bool> {
    if parent_group_id == child_group_id {
        return Ok(true);
    }

    connection.query_row(
        "
        WITH RECURSIVE reachable(id) AS (
            SELECT gc.child_group_id
            FROM group_children AS gc
            WHERE gc.parent_group_id = ?1

            UNION ALL

            SELECT gc.child_group_id
            FROM group_children AS gc
            JOIN reachable AS r ON gc.parent_group_id = r.id
        )
        SELECT EXISTS (
            SELECT 1 FROM reachable WHERE id = ?2
        );
        ",
        params![child_group_id, parent_group_id],
        |row| row.get(0),
    )
}

pub fn add_child(
    connection: &Connection,
    parent_group_id: i64,
    child_group_id: i64
) -> Result<()> {
    connection.execute(
        "
        INSERT OR IGNORE INTO group_children (parent_group_id, child_group_id)
        VALUES (?1, ?2)
        ",
        params![parent_group_id, child_group_id],
    )?;
    Ok(())
}

pub fn delete_child(
    connection: &Connection,
    parent_group_id: i64,
    child_group_id: i64,
) -> Result<()> {
    connection.execute(
        "DELETE FROM group_children WHERE parent_group_id = ?1 AND child_group_id = ?2",
        params![parent_group_id, child_group_id],
    )?;
    Ok(())
}

pub fn fetch_children(connection: &Connection, parent_group_id: i64) -> Result<Vec<Group>> {
    let mut statement = connection.prepare(
        "
        SELECT g.id, g.name, g.created_at, g.description
        FROM groups AS g
        JOIN group_children AS gc ON g.id = gc.child_group_id
        WHERE gc.parent_group_id = ?1
        ",
    )?;
    let groups = statement
        .query_map(params![parent_group_id], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                description: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<Group>, _>>()?;
    Ok(groups)
}

pub fn has_child(connection: &Connection, parent_group_id: i64, child_group_id: i64) -> Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM group_children WHERE parent_group_id = ?1 AND child_group_id = ?2)",
        params![parent_group_id, child_group_id],
        |row| row.get(0),
    )
}

pub fn add_tag(connection: &Connection, group_id: i64, tag_id: i64) -> Result<()> {
    connection.execute(
        "
        INSERT OR IGNORE INTO group_tags (group_id, tag_id)
        VALUES (?1, ?2)
        ",
        params![group_id, tag_id],
    )?;
    Ok(())
}

pub fn delete_tag(connection: &Connection, group_id: i64, tag_id: i64) -> Result<()> {
    connection.execute(
        "DELETE FROM group_tags WHERE group_id = ?1 AND tag_id = ?2",
        params![group_id, tag_id],
    )?;
    Ok(())
}

pub fn fetch_tags(connection: &Connection, group_id: i64) -> Result<Vec<Tag>> {
    let mut statement = connection.prepare(
        "
        SELECT t.id, t.name
        FROM tags AS t
        JOIN group_tags AS gt ON t.id = gt.tag_id
        WHERE gt.group_id = ?1
        "
    )?;
    let tags = statement
        .query_map(params![group_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<Tag>, _>>()?;
    Ok(tags)
}

pub fn has_tag(connection: &Connection, group_id: i64, tag_id: i64) -> Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM group_tags WHERE group_id = ?1 AND tag_id = ?2)",
        params![group_id, tag_id],
        |row| row.get(0),
    )
}
