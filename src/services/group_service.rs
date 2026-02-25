use anyhow::Result;
use rusqlite::Connection;
use crate::db::group_repo;
use crate::domain::group::Group;
use crate::domain::file::File;
use crate::domain::tag::Tag;
use crate::utils::time;

pub fn create_group(connection: &Connection, group_name: &str, description: Option<&str>) -> Result<Option<i64>> {
    Ok(group_repo::insert_group(connection, group_name, time::now(), description.unwrap_or(""))?)
}

pub fn delete_group(connection: &Connection, group_id: i64) -> Result<()> {
    Ok(group_repo::delete_group(connection, group_id)?)
}

pub fn get_group(connection: &Connection, group_id: i64) -> Result<Option<Group>> {
    Ok(group_repo::fetch_group(connection, group_id)?)
}

pub fn fetch_groups(connection: &Connection) -> Result<Vec<Group>> {
    Ok(group_repo::fetch_groups(connection)?)
}

pub fn add_file(connection: &Connection, group_id: i64, file_id: i64) -> Result<()> {
    Ok(group_repo::insert_file(connection, group_id, file_id)?)
}

pub fn add_files(connection: &Connection, group_id: i64, file_ids: &[i64]) -> Result<()> {
    for &file_id in file_ids {
        add_file(connection, group_id, file_id)?;
    }
    Ok(())
}

pub fn remove_file(connection: &Connection, group_id: i64, file_id: i64) -> Result<()> {
    Ok(group_repo::delete_file(connection, group_id, file_id)?)
}

pub fn fetch_files(connection: &Connection, group_id: i64) -> Result<Vec<File>> {
    Ok(group_repo::fetch_files(connection, group_id)?)
}

pub fn has_file(connection: &Connection, group_id: i64, file_id: i64) -> Result<bool> {
    Ok(group_repo::has_file(connection, group_id, file_id)?)
}

pub fn would_create_cycle(connection: &Connection, parent_group_id: i64, child_group_id: i64) -> Result<bool> {
    Ok(group_repo::would_create_cycle(connection, parent_group_id, child_group_id)?)
}

pub fn add_child(connection: &Connection, parent_group_id: i64, child_group_id: i64) -> Result<()> {
    Ok(group_repo::add_child(connection, parent_group_id, child_group_id)?)
}

pub fn remove_child(connection: &Connection, parent_group_id: i64, child_group_id: i64) -> Result<()> {
    Ok(group_repo::delete_child(connection, parent_group_id, child_group_id)?)
}

pub fn fetch_children(connection: &Connection, parent_group_id: i64) -> Result<Vec<Group>> {
    Ok(group_repo::fetch_children(connection, parent_group_id)?)
}

pub fn has_child(connection: &Connection, parent_group_id: i64, child_group_id: i64) -> Result<bool> {
    Ok(group_repo::has_child(connection, parent_group_id, child_group_id)?)
}

pub fn add_tag(connection: &Connection, group_id: i64, tag_id: i64) -> Result<()> {
    Ok(group_repo::add_tag(connection, group_id, tag_id)?)
}

pub fn remove_tag(connection: &Connection, group_id: i64, tag_id: i64) -> Result<()> {
    Ok(group_repo::delete_tag(connection, group_id, tag_id)?)
}

pub fn fetch_tags(connection: &Connection, group_id: i64) -> Result<Vec<Tag>> {
    Ok(group_repo::fetch_tags(connection, group_id)?)
}

pub fn has_tag(connection: &Connection, group_id: i64, tag_id: i64) -> Result<bool> {
    Ok(group_repo::has_tag(&connection, group_id, tag_id)?)
}
