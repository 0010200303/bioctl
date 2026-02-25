use chrono::DateTime;
use comfy_table::{Table, presets::ASCII_MARKDOWN};
use crate::domain::{file::File, group::Group};

const DATETIME_FORMAT: &str = "%d-%m-%Y %H:%M:%S";

pub fn print_files(files: Vec<File>) -> () {
    let mut table = Table::new();
    table.load_preset(ASCII_MARKDOWN)
        .set_header(vec!["ID", "created_at", "path"])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
    
    for file in files {
        let datetime = DateTime::from_timestamp(file.created_at, 0).unwrap();
        table.add_row(vec![file.id.to_string(), datetime.format(DATETIME_FORMAT).to_string(), file.path]);
    }
    println!("{table}");
}

pub fn print_groups(groups: Vec<Group>) -> () {
    let mut table = Table::new();
    table.load_preset(ASCII_MARKDOWN)
        .set_header(vec!["ID", "name", "created_at", "description"])
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    for group in groups {
        let datetime = DateTime::from_timestamp(group.created_at, 0).unwrap();
        table.add_row(vec![
            group.id.to_string(),
            group.name,
            datetime.format(DATETIME_FORMAT).to_string(),
            group.description
        ]);
    }
    println!("{table}");
}
