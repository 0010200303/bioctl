use clap::{Parser, Subcommand};
use rusqlite::Connection;
use anyhow::{Result, anyhow};
use crate::Config;
use crate::db::schema;
use crate::services::{file_service, group_service, tag_service};
use crate::domain::file::File;
use crate::domain::group::Group;
use crate::utils::print::{print_files, print_groups};

#[derive(Parser)]
#[command(name = "bioctl")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ResetDB {
        #[arg(long)]
        force: bool,
    },



    TrackFiles {
        paths: Vec<String>,
        #[arg(short, long, num_args = 1..)]
        groups: Option<Vec<i64>>,

        #[arg(short, long)]
        recursive: bool,
    },
    UntrackFile {
        file_id: i64,
    },
    GetFile {
        file_id: i64,
    },
    ListFiles,



    CreateGroup {
        group_name: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    DeleteGroup {
        group_id: i64,
    },
    GetGroup {
        group_id: i64,
    },
    ListGroups,

    GroupAddFile {
        group_id: i64,
        file_id: i64,
    },
    GroupRemoveFile {
        group_id: i64,
        file_id: i64,
    },
    GroupListFiles {
        group_id: i64,
    },
    GroupHasFile {
        group_id: i64,
        file_id: i64,
    },

    GroupAddChild {
        parent_group_id: i64,
        child_group_id: i64,
    },
    GroupRemoveChild {
        parent_group_id: i64,
        child_group_id: i64,
    },
    GroupListChildren {
        parent_group_id: i64,
    },
    GroupHasChild {
        parent_group_id: i64,
        child_group_id: i64,
    },

    GroupAddTag {
        group_id: i64,
        tag: String,
    },
    GroupRemoveTag {
        group_id: i64,
        tag: String,
    },
    GroupListTags {
        group_id: i64,
    },
    GroupHasTag {
        group_id: i64,
        tag: String,
    },
}



pub fn run(cli: Cli, connection: &mut Connection, config: &Config) -> Result<()> {
    let colors = config.colors();
    let color_reset = &colors.reset;
    let color_red = &colors.red;
    let color_green = &colors.green;
    let color_yellow = &colors.yellow;

    let get_file = |connection: &Connection, file_id: i64| -> Result<File> {
        file_service::get_file(connection, file_id)?
            .ok_or_else(|| anyhow!("{color_red}File with id {} not found{color_reset}", file_id))
    };

    let get_group = |connection: &Connection, group_id: i64| -> Result<Group> {
        group_service::get_group(&connection, group_id)?
            .ok_or_else(|| anyhow!("{color_red}Group with id {} not found{color_reset}", group_id))
    };

    match cli.command {
        Commands::ResetDB { force } => {
            if force == false {
                return Err(anyhow!("{color_red}Refusing to reset database without --force flag{color_reset}"));
            }
            schema::reset_schema(&connection)?;
            println!("{color_yellow}Reset entire db{color_reset}");
        }



        Commands::TrackFiles { paths, groups, recursive } => {
            let mut canonical_paths: Vec<String> = Vec::new();
            for path in &paths {
                let mut found = match file_service::collect_canonical_paths(&path, recursive) {
                    Ok(p) => p,
                    Err(e) => {
                        if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                            match io_error.kind() {
                                std::io::ErrorKind::NotFound => {
                                    return Err(anyhow!("{color_red}Path '{}' does not exist{color_reset}", path));
                                }
                                std::io::ErrorKind::InvalidInput => {
                                    return Err(anyhow!("{color_red}Path '{}' is not a directory or a file{color_reset}", path));
                                }
                                _ => {}
                            }
                        }
                        return Err(e);
                    }
                };
                canonical_paths.append(&mut found);
            }

            let mut file_ids = file_service::track_files(connection, &canonical_paths)?;
            for file_id in &file_ids {
                println!("{color_green}File is now tracked with id {}{color_reset}", file_id);
            }

            if let Some(group_list) = groups {
                if file_ids.len() != canonical_paths.len() {
                    file_ids.clear();

                    for canonical_path in &canonical_paths {
                        let file_id = file_service::find_file_by_path(&connection, canonical_path)?
                            .ok_or_else(|| anyhow!("{color_red}File could not be found{color_reset}"))?.id;
                        file_ids.push(file_id);
                    }
                }

                for &group_id in &group_list {
                    let group = get_group(&connection, group_id)?;

                    group_service::add_files(&connection, group_id, &file_ids)?;

                    for canonical_path in &canonical_paths {
                        println!("{color_green}Added file ({}) to group '{}'{color_reset}", canonical_path, group.name);
                    }
                }
                println!("{color_green}Tracked and added {} files to {} different groups{color_reset}", file_ids.len(), group_list.len());
            } else {
                println!("{color_green}Tracked {} files{color_reset}", file_ids.len());
            }
        }
        Commands::UntrackFile { file_id } => {
            if let Some(path) = file_service::untrack_file(&connection, file_id)? {
                println!("{color_green}File is no longer being tracked ({}){color_reset}", path);
            } else {
                return Err(anyhow!("{color_yellow}File with id {} not found{color_reset}", file_id));
            }
        }
        Commands::GetFile { file_id } => {
            let file = get_file(&connection, file_id)?;
            print_files(vec![file]);
        }
        Commands::ListFiles => {
            let files = file_service::list_files(connection)?;
            if files.is_empty() {
                println!("No files");
            } else {
                print_files(file_service::list_files(&connection)?);
            }
        }



        Commands::CreateGroup { group_name, description } => {
            let group_id = group_service::create_group(&connection, &group_name, description.as_deref())?
                .ok_or_else(|| anyhow!("{color_red}Could not create group '{}'{color_reset}", group_name))?;
            println!("{color_green}Group '{}'(id: {}) created{color_reset}", group_name, group_id);
        }
        Commands::DeleteGroup { group_id } => {
            let group = get_group(&connection, group_id)?;

            group_service::delete_group(&connection, group_id)?;
            println!("{color_green}Group '{}'(id: {}) deleted{color_reset}", group.name, group_id);
        }
        Commands::GetGroup { group_id } => {
            let group = get_group(&connection, group_id)?;
            print_groups(vec![group]);
        }
        Commands::ListGroups => {
            let groups = group_service::fetch_groups(&connection)?;
            if groups.is_empty() {
                println!("No groups");
            } else {
                print_groups(groups);
            }
        }

        Commands::GroupAddFile { group_id, file_id } => {
            let group = get_group(&connection, group_id)?;
            let file = get_file(&connection, file_id)?;

            group_service::add_file(&connection, group_id, file_id)?;
            println!("{color_green}Added file ({}) to group '{}'{color_reset}", file.path, group.name);
        }
        Commands::GroupRemoveFile { group_id, file_id } => {
            let group = get_group(&connection, group_id)?;
            let file = get_file(&connection, file_id)?;

            group_service::remove_file(&connection, group_id, file_id)?;
            println!("{color_green}Removed file ({}) from group '{}'{color_reset}", file.path, group.name);
        }
        Commands::GroupListFiles { group_id } => {
            let group = get_group(&connection, group_id)?;

            let files = group_service::fetch_files(&connection, group_id)?;
            if files.is_empty() {
                print!("No files");
            } else {
                println!("{} files in group '{}':", files.len(), group.name);
                print_files(files);
            }
        }
        Commands::GroupHasFile { group_id, file_id } => {
            let group = get_group(&connection, group_id)?;
            let file = get_file(&connection, file_id)?;

            if group_service::has_file(&connection, group_id, file_id)? {
                println!("{color_green}Group '{}' has file ({}){color_reset}", group.name, file.path);
            } else {
                println!("{color_yellow}Group '{}' does not have file ({}){color_reset}", group.name, file.path);
            }
        }

        Commands::GroupAddChild { parent_group_id, child_group_id } => {
            if parent_group_id == child_group_id {
                return Err(anyhow!("{color_red}Can not add group to itself{color_reset}"));
            }

            let parent_group = get_group(&connection, parent_group_id)?;
            let child_group = get_group(&connection, child_group_id)?;

            let cycle = group_service::would_create_cycle(&connection, parent_group_id, child_group_id)?;
            if cycle {
                return Err(anyhow!(
                    "{color_red}Adding child '{}' to parent '{}' would create a cycle{color_reset}",
                    child_group.name, parent_group.name,
                ));
            }

            group_service::add_child(&connection, parent_group_id, child_group_id)?;
            println!("{color_green}Added child '{}' to parent '{}'{color_reset}", child_group.name, parent_group.name);
        }
        Commands::GroupRemoveChild { parent_group_id, child_group_id } => {
            if parent_group_id == child_group_id {
                return Err(anyhow!("{color_red}Can not remove group from itself{color_reset}"))
            }
            
            let parent_group = get_group(&connection, parent_group_id)?;
            let child_group = get_group(&connection, child_group_id)?;

            if group_service::has_child(connection, parent_group_id, child_group_id)? == false {
                println!("{color_yellow}Group '{}' is not a child of group '{}'{color_reset}",
                    child_group.name,
                    parent_group.name
                );
                return Ok(());
            }

            group_service::remove_child(&connection, parent_group_id, child_group_id)?;
            println!("{color_green}Removed child '{}' from parent '{}'{color_reset}", child_group.name, parent_group.name);
        }
        Commands::GroupListChildren {parent_group_id} => {
            let parent_group = get_group(&connection, parent_group_id)?;

            let groups = group_service::fetch_children(&connection, parent_group_id)?;
            if groups.is_empty() {
                println!("No children");
            } else {
                println!("{} children in group '{}':", groups.len(), parent_group.name);
                print_groups(groups);
            }
        }
        Commands::GroupHasChild { parent_group_id, child_group_id } => {
            let parent_group = get_group(&connection, parent_group_id)?;
            let child_group = get_group(&connection, child_group_id)?;

            if group_service::has_child(&connection, parent_group_id, child_group_id)? {
                println!("{color_green}Group '{}' has child '{}'{color_reset}", parent_group.name, child_group.name);
            } else {
                println!("{color_yellow}Group '{}' does not have child '{}'{color_reset}", parent_group.name, child_group.name);
            }
        }

        Commands::GroupAddTag { group_id, tag } => {
            let group = get_group(&connection, group_id)?;
            let tag_id = match tag_service::find_tag_by_name(&connection, &tag)? {
                Some(tag) => tag.id,
                None => {
                    tag_service::create_tag(&connection, &tag)?
                        .ok_or_else(|| anyhow!("{color_red}Could not create Tag '{}'{color_reset}", tag))?
                }
            };

            group_service::add_tag(&connection, group_id, tag_id)?;
            println!("{color_green}Added tag '{}' to group '{}'{color_reset}", tag, group.name);
        }
        Commands::GroupRemoveTag {group_id, tag } => {
            let group = get_group(&connection, group_id)?;
            let _tag = tag_service::find_tag_by_name(&connection, &tag)?
                .ok_or_else(|| anyhow!("{color_red}Tag '{}' does not exist{color_reset}", tag))?;

            group_service::remove_tag(&connection, group_id, _tag.id)?;
            println!("{color_green}Removed tag '{}' from group '{}'{color_reset}", tag, group.name);
        }
        Commands::GroupListTags { group_id } => {
            let group = get_group(&connection, group_id)?;

            let tags = group_service::fetch_tags(&connection, group_id)?;
            if tags.is_empty() {
                println!("No tags");
            } else {
                let tag_names = tags.iter().map(|t| t.name.as_str()).collect::<Vec<&str>>();
                println!("{} tags in group '{}':", tags.len(), group.name);
                println!("{}", tag_names.join(", "));
            }
        }
        Commands::GroupHasTag { group_id, tag } => {
            let group = get_group(&connection, group_id)?;
            let tag_id = match tag_service::find_tag_by_name(&connection, &tag)? {
                Some(tag) => tag.id,
                None => {
                    tag_service::create_tag(&connection, &tag)?
                        .ok_or_else(|| anyhow!("{color_red}Could not create Tag '{}'{color_reset}", tag))?
                }
            };

            if group_service::has_tag(&connection, group_id, tag_id)? {
                println!("{color_green}Group '{}' has tag '{}'{color_reset}", group.name, tag);
            } else {
                println!("{color_yellow}Group '{}' does not have tag '{}'{color_reset}", group.name, tag);
            }
        }
    }
    Ok(())
}
