use rusqlite::{Connection, Result};

pub fn init_schema(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS files (
            id                  INTEGER PRIMARY KEY,
            path                TEXT NOT NULL UNIQUE,
            created_at          INTEGER
        );

        CREATE TABLE IF NOT EXISTS groups (
            id                  INTEGER PRIMARY KEY,
            name                TEXT NOT NULL,
            created_at          INTEGER,
            description         TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_groups_name ON groups(name);

        -- Group <-> File associations
        CREATE TABLE IF NOT EXISTS group_files (
            group_id           INTEGER NOT NULL,
            file_id             INTEGER NOT NULL,
            PRIMARY KEY         (group_id, file_id),
            FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
            FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
        );

        -- Group <-> Group Hierachy (circles prevented in api)
        CREATE TABLE IF NOT EXISTS group_children (
            parent_group_id     INTEGER NOT NULL,
            child_group_id      INTEGER NOT NULL,
            PRIMARY KEY         (parent_group_id, child_group_id),
            FOREIGN KEY (parent_group_id) REFERENCES groups(id) ON DELETE CASCADE,
            FOREIGN KEY (child_group_id) REFERENCES groups(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS tags (
            id                  INTEGER PRIMARY KEY,
            name                TEXT NOT NULL COLLATE NOCASE UNIQUE
        );

        CREATE TABLE IF NOT EXISTS group_tags (
            group_id            INTEGER NOT NULL,
            tag_id              INTEGER NOT NULL,
            PRIMARY KEY         (group_id, tag_id),
            FOREIGN KEY (group_id) REFERENCES groups(id) on DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) on DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_group_tags_tag ON group_tags(tag_id);
        ",
    )?;
    Ok(())

    // CREATE TABLE IF NOT EXISTS group_metadata (
    //     group_id            INTEGER NOT NULL,
    //     key                 TEXT NOT NULL,
    //     value               TEXT NOT NULL,
    //     PRIMARY KEY         (group_id, key),
    //     FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
    // );
    // CREATE INDEX IF NOT EXISTS idx_group_metadata_key ON group_metadata(key);
}

pub fn reset_schema(connection: &Connection) -> Result<()> {
    connection.execute_batch(
        "
        PRAGMA foreign_keys = OFF;

        DROP TABLE IF EXISTS files;
        DROP TABLE IF EXISTS groups;
        DROP TABLE IF EXISTS group_files;
        DROP TABLE IF EXISTS group_children;

        DROP TABLE IF EXISTS tags;
        DROP TABLE IF EXISTS group_tags;

        PRAGMA foreign_keys = ON;
        ",
    )?;
    init_schema(connection)?;

    Ok(())
}
