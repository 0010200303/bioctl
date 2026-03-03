use std::io::{Error, ErrorKind};
use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;
use crate::db::file_repo;
use crate::domain::file::File;
use crate::utils::time;

pub fn collect_canonical_paths(path: &str, recursive: bool) -> Result<Vec<String>> {
    let p = Path::new(path);
    if p.exists() == false {
        return Err(Error::new(ErrorKind::NotFound, "Path does not exist").into());
    }
    if p.is_file() {
        let p = Path::new(path);
        let canonical = p.canonicalize()?;
        return Ok(vec![canonical.to_string_lossy().into_owned()]);
    }
    else if p.is_dir() == false {
        return Err(Error::new(ErrorKind::InvalidInput, "Path is not a diretory or a file").into());
    }

    let mut paths = Vec::new();

    let mut dirs = vec![p.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry_result in dir.read_dir()? {
            let entry = entry_result?;
            let file_type = entry.file_type()?;

            if file_type.is_file() {
                let canonical = entry.path().canonicalize()?;
                let canonical_str = canonical.to_string_lossy().into_owned();
                paths.push(canonical_str);
            } else if file_type.is_dir() && recursive {
                dirs.push(entry.path());
            }
        }
    }
    Ok(paths)
}

pub fn track_file(connection: &Connection, path: &str) -> Result<Option<i64>> {
    Ok(file_repo::insert_file(connection, &path, time::now())?)
}

pub fn track_files(connection: &mut Connection, paths: &[String]) -> Result<Vec<i64>> {
    let mut file_ids = Vec::new();
    
    let transaction = connection.transaction()?;
    for path in paths {
        if let Some(file_id) = file_repo::insert_file(&transaction, path, time::now())? {
            file_ids.push(file_id);
        }
    }
    transaction.commit()?;

    Ok(file_ids)
}

pub fn untrack_file(connection: &Connection, file_id: i64) -> Result<Option<String>> {
    Ok(file_repo::delete_file(connection, file_id)?)
}

pub fn get_file(connection: &Connection, file_id: i64) -> Result<Option<File>> {
    Ok(file_repo::fetch_file(connection, file_id)?)
}

pub fn find_file_by_path(connection: &Connection, path: &str) -> Result<Option<File>> {
    Ok(file_repo::find_file_by_path(connection, &path)?)
}

pub fn list_files(connection: &Connection) -> Result<Vec<File>> {
    Ok(file_repo::fetch_files(connection)?)
}
