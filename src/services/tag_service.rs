use anyhow::Result;
use rusqlite::Connection;
use crate::db::tag_repo;
use crate::domain::tag::Tag;

pub fn create_tag(connection: &Connection, tag_name: &str) -> Result<Option<i64>> {
    Ok(tag_repo::insert_tag(connection, tag_name)?)
}

pub fn delete_tag(connection: &Connection, tag_id: i64) -> Result<Option<String>> {
    Ok(tag_repo::delete_tag(connection, tag_id)?)
}

pub fn get_tag(connection: &Connection, tag_id: i64) -> Result<Option<Tag>> {
    Ok(tag_repo::fetch_tag(connection, tag_id)?)
}

pub fn fetch_tags(connection: &Connection) -> Result<Vec<Tag>> {
    Ok(tag_repo::fetch_tags(connection)?)
}

pub fn find_tag_by_name(connection: &Connection, tag_name: &str) -> Result<Option<Tag>> {
    Ok(tag_repo::find_tag_by_name(connection, tag_name)?)
}
