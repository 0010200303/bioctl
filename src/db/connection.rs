use rusqlite::{Connection, Result};

pub fn get_connection(path: &str) -> Result<Connection> {
    if path == ":memory:" {
        return Ok(Connection::open_in_memory()?);
    }
    Ok(Connection::open(path)?)
}
