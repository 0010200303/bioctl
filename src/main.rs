mod cli;
mod db;
mod services;
mod utils;
mod domain;

use anyhow::Result;
use clap::Parser;
use crate::cli::{Cli, run};
use crate::db::{connection, schema};

pub struct Config {
    pub db_path: String,
    pub color: bool,
}

#[derive(Clone)]
pub struct Colors {
    pub reset: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
}

impl Config {
    pub fn from_env() -> Self {
        let _db_path  = std::env::var("BIOCTL_DB_PATH").unwrap_or("bioctl.db".into());
        let no_color = std::env::var_os("NO_COLOR").is_some();
        Self { db_path: _db_path, color: !no_color }
    }

    pub fn colors(&self) -> Colors {
        if self.color {
            Colors {
                reset: "\x1b[0m".into(),
                red: "\x1b[31m".into(),
                green: "\x1b[32m".into(),
                yellow: "\x1b[33m".into(),
            }
        } else {
            Colors {
                reset: "".into(),
                red: "".into(),
                green: "".into(),
                yellow: "".into(),
            }
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::from_env();

    let mut connection = connection::get_connection(&config.db_path)?;
    schema::init_schema(&connection)?;

    if let Err(error) = run(cli, &mut connection, &config) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
    Ok(())
}
