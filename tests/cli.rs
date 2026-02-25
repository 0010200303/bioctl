use std::path::Path;

use assert_cmd::{assert::Assert, cargo, Command};
use predicates::{prelude::*, str::contains};
use tempfile::NamedTempFile;

struct TestContext {
    _tmp: NamedTempFile,
}

impl TestContext {
    fn new() -> Self {
        let tmp = NamedTempFile::new().expect("failed to create temp db file");
        Self { _tmp: tmp }
    }

    fn cmd(&self, args: &[&str]) -> Assert {
        let temp_db_path = self._tmp.path().to_str().unwrap().to_string();

        let mut cmd = Command::new(cargo::cargo_bin!());
        return cmd
            .env("BIOCTL_DB_PATH", temp_db_path)
            .env("NO_COLOR", "1")
            .args(args)
            .assert();
    }
}

pub fn get_canonical_path(path: &str) -> String {
    let p = Path::new(path);

    let canonical = p.canonicalize().unwrap();
    canonical.to_string_lossy().into_owned()
}

#[test]
fn reset_without_force() {
    let ctx = TestContext::new();

    ctx.cmd(&["reset-db"])
        .failure()
        .stderr(contains("Refusing to reset database without --force flag")
    );
}

#[test]
fn reset_clears_db() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created")
    );

    ctx.cmd(&["track-file", "tests/data/tust"])
        .success()
        .stdout(contains("File is now tracked with id 1")
    );

    ctx.cmd(&["reset-db", "--force"])
        .success()
        .stdout(contains("Reset entire db")
    );

    ctx.cmd(&["list-groups"])
        .success()
        .stdout(contains("No groups")
    );

    ctx.cmd(&["list-files"])
        .success()
        .stdout(contains("No files")
    );
}



#[test]
fn track_file_path_does_not_exist() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-file", "__pls_do_not_exist.file__"])
        .failure()
        .stderr(contains("Path '__pls_do_not_exist.file__' does not exist")
    );
}

#[test]
fn track_file_path_is_not_a_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-file", "tests"])
        .failure()
        .stderr(contains("Path 'tests' is not a file")
    );
}

#[test]
fn track_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-file", "tests/data/tust"])
        .success()
        .stdout(contains("File is now tracked with id 1")
    );

    ctx.cmd(&["get-file", "1"])
        .success()
        .stdout(predicate::str::contains(get_canonical_path("tests/data/tust"))
    );
}

#[test]
fn track_file_with_groups() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created")
    );

    ctx.cmd(&["create-group", "tust2"])
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created")
    );

    let path = get_canonical_path("tests/data/tust");
    ctx.cmd(&["track-file", "tests/data/tust", "-g", "1", "2"])
        .success()
        .stdout(
            contains("File is now tracked with id 1")
            .and(contains(format!("Added file ({path}) to group 'tust'")))
            .and(contains(format!("Added file ({path}) to group 'tust2'")))
        );
}

#[test]
fn track_files_path_does_not_exist() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-file", "__pls_do_not_exist_dir__"])
        .failure()
        .stderr(contains("Path '__pls_do_not_exist_dir__' does not exist")
    );
}

#[test]
fn track_files_path_is_not_a_dir() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-files", "tests/data/tust"])
        .failure()
        .stderr(contains("Path 'tests/data/tust' is not a directory")
    );
}

#[test]
fn track_files() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-files", "tests/data"])
        .success()
        .stdout(
        contains("File is now tracked with id 1")
            .and(contains("File is now tracked with id 2"))
            .and(contains("Tracked 2 files"))
        );

    ctx.cmd(&["list-files"])
        .success()
        .stdout(
            predicate::str::contains(get_canonical_path("tests/data/tust"))
            .and(predicate::str::contains(get_canonical_path("tests/data/tust2")))
    );
}

#[test]
fn track_files_recursive() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-files", "tests/data", "-r"])
        .success()
        .stdout(
            contains("File is now tracked with id 1")
            .and(contains("File is now tracked with id 2"))
            .and(contains("File is now tracked with id 3"))
            .and(contains("File is now tracked with id 4"))
    );

    ctx.cmd(&["list-files"])
        .success()
        .stdout(
            predicate::str::contains(get_canonical_path("tests/data/tust"))
            .and(predicate::str::contains(get_canonical_path("tests/data/tust2")))
            .and(predicate::str::contains(get_canonical_path("tests/data/nested/nested_tust")))
            .and(predicate::str::contains(get_canonical_path("tests/data/nested/nested_tust2")))
    );
}

#[test]
fn track_files_with_groups() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created")
    );

    ctx.cmd(&["create-group", "tust2"])
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created")
    );

    let path1 = get_canonical_path("tests/data/tust");
    let path2 = get_canonical_path("tests/data/tust2");
    let path3 = get_canonical_path("tests/data/nested/nested_tust");
    let path4 = get_canonical_path("tests/data/nested/nested_tust2");
    ctx.cmd(&["track-files", "tests/data", "-r", "-g", "1", "2"])
        .success()
        .stdout(
                contains("File is now tracked with id 1")
                .and(contains("File is now tracked with id 2"))
                .and(contains("File is now tracked with id 3"))
                .and(contains("File is now tracked with id 4"))
                .and(contains(format!("Added file ({path1}) to group 'tust'")))
                .and(contains(format!("Added file ({path2}) to group 'tust'")))
                .and(contains(format!("Added file ({path3}) to group 'tust'")))
                .and(contains(format!("Added file ({path4}) to group 'tust'")))
                .and(contains(format!("Added file ({path1}) to group 'tust2'")))
                .and(contains(format!("Added file ({path2}) to group 'tust2'")))
                .and(contains(format!("Added file ({path3}) to group 'tust2'")))
                .and(contains(format!("Added file ({path4}) to group 'tust2'")))
    );

    ctx.cmd(&["list-files"])
        .success()
        .stdout(
            predicate::str::contains(&path1)
            .and(predicate::str::contains(&path2))
            .and(predicate::str::contains(&path3))
            .and(predicate::str::contains(&path4))
    );

    ctx.cmd(&["group-list-files", "1"])
        .success()
        .stdout(
            predicate::str::contains(&path1)
            .and(predicate::str::contains(&path2))
            .and(predicate::str::contains(&path3))
            .and(predicate::str::contains(&path4))
    );

    ctx.cmd(&["group-list-files", "2"])
        .success()
        .stdout(
            predicate::str::contains(&path1)
            .and(predicate::str::contains(&path2))
            .and(predicate::str::contains(&path3))
            .and(predicate::str::contains(&path4))
    );
}

#[test]
fn untrack_file_id_not_found() {
    let ctx = TestContext::new();

    ctx.cmd(&["untrack-file", "2807"])
        .failure()
        .stderr(contains("File with id 2807 not found")
    );
}

#[test]
fn untrack_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    let path = get_canonical_path("tests/data/tust");
    ctx.cmd(&["track-file", "tests/data/tust", "-g", "1"])
        .success()
        .stdout(
            contains("File is now tracked with id 1")
            .and(contains(format!("Added file ({path}) to group 'tust'")))
    );

    ctx.cmd(&["untrack-file", "1"])
        .success()
        .stdout(contains(format!("File is no longer being tracked ({path})")));

    ctx.cmd(&["list-files"])
        .success()
        .stdout(contains("No files"));

    ctx.cmd(&["group-list-files", "1"])
        .success()
        .stdout(contains("No files"));
}

#[test]
fn get_file_id_not_found() {
    let ctx = TestContext::new();

    ctx.cmd(&["track-file", "tests/data/tust"])
        .success()
        .stdout(contains("File is now tracked with id 1"));

    ctx.cmd(&["get-file", "1"])
        .success()
        .stdout(contains(get_canonical_path("tests/data/tust")));
}

#[test]
fn list_files_empty() {
    let ctx = TestContext::new();

    ctx.cmd(&["list-files"])
        .success()
        .stdout(contains("No files"));
}

#[test]
fn list_files() {
    let ctx = TestContext::new();

    let path1 = get_canonical_path("tests/data/tust");
    let path2 = get_canonical_path("tests/data/tust2");
    let path3 = get_canonical_path("tests/data/nested/nested_tust");
    let path4 = get_canonical_path("tests/data/nested/nested_tust2");
    ctx.cmd(&["track-files", "tests/data", "-r"])
        .success()
        .stdout(
                contains("File is now tracked with id 1")
                .and(contains("File is now tracked with id 2"))
                .and(contains("File is now tracked with id 3"))
                .and(contains("File is now tracked with id 4"))
    );

    ctx.cmd(&["list-files"])
        .success()
        .stdout(
            predicate::str::contains(&path1)
            .and(predicate::str::contains(&path2))
            .and(predicate::str::contains(&path3))
            .and(predicate::str::contains(&path4))
    );
}



#[test]
fn create_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust", "-d", "this is just a tust group"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["get-group", "1"])
        .success()
        .stdout(
            contains("tust")
            .and(contains("this is just a tust group"))
    );
}

#[test]
fn delete_group_id_not_found() {
    let ctx = TestContext::new();

    ctx.cmd(&["delete-group", "1"])
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn delete_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["delete-group", "1"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) deleted"));
}

#[test]
fn get_group_id_not_found() {
    let ctx = TestContext::new();

    ctx.cmd(&["get-group", "1"])
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn get_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["get-group", "1"])
        .success()
        .stdout(contains("tust"));
}

#[test]
fn list_groups_empty() {
    let ctx = TestContext::new();

    ctx.cmd(&["list-groups"])
        .success()
        .stdout(contains("No groups"));
}

#[test]
fn list_groups() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust", "-d", "this is just a tust group"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"])
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["list-groups"])
        .success()
        .stdout(
            contains("tust")
            .and(contains("this is just a tust group"))
            .and(contains("tust2"))
    );
}

#[test]
fn group_add_file_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-add-file", "1", "1"])
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_add_file_no_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-add-file", "1", "1"])
        .failure()
        .stderr(contains("File with id 1 not found"));
}

#[test]
fn group_add_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"])
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["track-file", "tests/data/tust"])
        .success()
        .stdout(contains("File is now tracked with id 1")
    );

    let path = get_canonical_path("tests/data/tust");
    ctx.cmd(&["group-add-file", "1", "1"])
        .success()
        .stdout(contains(format!("Added file ({}) to group 'tust'", path)));
}

#[test]
fn group_remove_file_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-remove-file", "1", "1"])
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_remove_file_no_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-remove-file", "1", "1"]) 
        .failure()
        .stderr(contains("File with id 1 not found"));
}

#[test]
fn group_remove_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["track-file", "tests/data/tust", "-g", "1"]) 
        .success()
        .stdout(contains("File is now tracked with id 1"));

    let path = get_canonical_path("tests/data/tust");
    ctx.cmd(&["group-remove-file", "1", "1"]) 
        .success()
        .stdout(contains(format!("Removed file ({}) from group 'tust'", path)));
}

#[test]
fn group_list_files_empty() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-list-files", "1"]) 
        .success()
        .stdout(contains("No files"));
}

#[test]
fn group_list_files() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    let path1 = get_canonical_path("tests/data/tust");
    let path2 = get_canonical_path("tests/data/tust2");
    let path3 = get_canonical_path("tests/data/nested/nested_tust");
    let path4 = get_canonical_path("tests/data/nested/nested_tust2");
    ctx.cmd(&["track-files", "tests/data", "-r", "-g", "1"]) 
        .success()
        .stdout(
            contains("File is now tracked with id 1")
            .and(contains("File is now tracked with id 2"))
            .and(contains("File is now tracked with id 3"))
            .and(contains("File is now tracked with id 4"))
            .and(contains(format!("Added file ({path1}) to group 'tust'")))
            .and(contains(format!("Added file ({path2}) to group 'tust'")))
            .and(contains(format!("Added file ({path3}) to group 'tust'")))
            .and(contains(format!("Added file ({path4}) to group 'tust'")))
    );

    ctx.cmd(&["group-list-files", "1"]) 
        .success()
        .stdout(
            contains("4 files in group 'tust':")
            .and(contains(&path1))
            .and(contains(&path2))
            .and(contains(&path3))
            .and(contains(&path4))
    );
}

#[test]
fn group_has_file_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-has-file", "1", "1"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_has_file_no_file() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-has-file", "1", "1"]) 
        .failure()
        .stderr(contains("File with id 1 not found"));
}

#[test]
fn group_has_file_false() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["track-file", "tests/data/tust"]) 
        .success()
        .stdout(contains("File is now tracked with id 1"));

    let path = get_canonical_path("tests/data/tust");
    ctx.cmd(&["group-has-file", "1", "1"]) 
        .success()
        .stdout(contains(format!("Group 'tust' does not have file ({})", path)));
}

#[test]
fn group_has_file_true() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    let path = get_canonical_path("tests/data/tust");
    ctx.cmd(&["track-file", "tests/data/tust", "-g", "1"]) 
        .success()
        .stdout(
            contains("File is now tracked with id 1")
            .and(contains(format!("Added file ({}) to group 'tust'", path)))
        );

    ctx.cmd(&["group-has-file", "1", "1"]) 
        .success()
        .stdout(contains(format!("Group 'tust' has file ({})", path)));
}

#[test]
fn group_add_child_same() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-add-child", "1", "1"]) 
        .failure()
        .stderr(contains("Can not add group to itself"));
}

#[test]
fn group_add_child_no_parent() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-add-child", "1", "2"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_add_child_no_child() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-add-child", "1", "2"]) 
        .failure()
        .stderr(contains("Group with id 2 not found"));
}

#[test]
fn group_add_child_cycle() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["group-add-child", "1", "2"])
        .success()
        .stdout(contains("Added child 'tust2' to parent 'tust'"));

    ctx.cmd(&["group-add-child", "2", "1"])
        .failure()
        .stderr(contains("Adding child 'tust' to parent 'tust2' would create a cycle"));
}

#[test]
fn group_add_child() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["create-group", "tust3"]) 
        .success()
        .stdout(contains("Group 'tust3'(id: 3) created"));

    ctx.cmd(&["group-add-child", "1", "2"])
        .success()
        .stdout(contains("Added child 'tust2' to parent 'tust'"));

    ctx.cmd(&["group-add-child", "2", "3"])
        .success()
        .stdout(contains("Added child 'tust3' to parent 'tust2'"));
}

#[test]
fn group_remove_child_same() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-remove-child", "1", "1"]) 
        .failure()
        .stderr(contains("Can not remove group from itself"));
}

#[test]
fn group_remove_child_no_parent() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-remove-child", "1", "2"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_remove_child_no_child() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-remove-child", "1", "2"]) 
        .failure()
        .stderr(contains("Group with id 2 not found"));
}

#[test]
fn group_remove_child_group_is_not_a_child() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["group-remove-child", "1", "2"]) 
        .success()
        .stdout(contains("Group 'tust2' is not a child of group 'tust'"));
}

#[test]
fn group_remove_child() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["create-group", "tust3"]) 
        .success()
        .stdout(contains("Group 'tust3'(id: 3) created"));

    ctx.cmd(&["group-add-child", "1", "2"]) 
        .success()
        .stdout(contains("Added child 'tust2' to parent 'tust'"));

    ctx.cmd(&["group-add-child", "1", "3"]) 
        .success()
        .stdout(contains("Added child 'tust3' to parent 'tust'"));

    ctx.cmd(&["group-remove-child", "1", "2"]) 
        .success()
        .stdout(contains("Removed child 'tust2' from parent 'tust'"));

    ctx.cmd(&["group-list-children", "1"])
        .success()
        .stdout(
            contains("1 children in group 'tust':")
            .and(contains("tust2").not())
            .and(contains("tust3"))
        );
}

#[test]
fn group_list_children_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-list-children", "1"])
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_list_children_empty() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-list-children", "1"]) 
        .success()
        .stdout(contains("No children"));
}

#[test]
fn group_list_children() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["create-group", "tust3"]) 
        .success()
        .stdout(contains("Group 'tust3'(id: 3) created"));

    ctx.cmd(&["group-add-child", "1", "2"]) 
        .success()
        .stdout(contains("Added child 'tust2' to parent 'tust'"));

    ctx.cmd(&["group-add-child", "1", "3"]) 
        .success()
        .stdout(contains("Added child 'tust3' to parent 'tust'"));

    ctx.cmd(&["group-list-children", "1"]) 
        .success()
        .stdout(
            contains("2 children in group 'tust':")
            .and(contains("tust2"))
            .and(contains("tust3"))
        );
}

#[test]
fn group_has_child_no_parent() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-has-child", "1", "2"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_has_child_no_child() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-has-child", "1", "2"]) 
        .failure()
        .stderr(contains("Group with id 2 not found"));
}

#[test]
fn group_has_child_false() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["group-has-child", "1", "2"]) 
        .success()
        .stdout(contains("Group 'tust' does not have child 'tust2'"));
}

#[test]
fn group_has_child_true() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["group-add-child", "1", "2"]) 
        .success()
        .stdout(contains("Added child 'tust2' to parent 'tust'"));

    ctx.cmd(&["group-has-child", "1", "2"]) 
        .success()
        .stdout(contains("Group 'tust' has child 'tust2'"));
}

#[test]
fn group_add_tag_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-add-tag", "1", "tust"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_add_tag() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-add-tag", "1", "tust1"]) 
        .success()
        .stdout(contains("Added tag 'tust1' to group 'tust'"));

    ctx.cmd(&["group-add-tag", "1", "tust2"]) 
        .success()
        .stdout(contains("Added tag 'tust2' to group 'tust'"));

    // also check if tag already existed
    ctx.cmd(&["group-add-tag", "1", "tust1"]) 
        .success()
        .stdout(contains("Added tag 'tust1' to group 'tust'"));

    ctx.cmd(&["group-list-tags", "1"])
        .success()
        .stdout(
            contains("2 tags in group 'tust':")
            .and(contains("tust1"))
            .and(contains("tust2"))
    );
}

#[test]
fn group_remove_tag_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-remove-tag", "1", "tust"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_remove_tag_no_tag() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-remove-tag", "1", "tust"]) 
        .failure()
        .stderr(contains("Tag 'tust' does not exist"));
}

#[test]
fn group_remove_tag() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-add-tag", "1", "tust1"]) 
        .success()
        .stdout(contains("Added tag 'tust1' to group 'tust'"));

    ctx.cmd(&["group-add-tag", "1", "tust2"]) 
        .success()
        .stdout(contains("Added tag 'tust2' to group 'tust'"));

    ctx.cmd(&["group-remove-tag", "1", "tust1"]) 
        .success()
        .stdout(contains("Removed tag 'tust1' from group 'tust'"));

    ctx.cmd(&["group-list-tags", "1"]) 
        .success()
        .stdout(
            contains("1 tags in group 'tust':")
            .and(contains("tust2"))
            .and(predicate::str::contains("tust1").not())
    );
}

#[test]
fn group_list_tags_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-list-tags", "1"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_list_tags_empty() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-list-tags", "1"]) 
        .success()
        .stdout(contains("No tags"));
}

#[test]
fn group_list_tags() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-add-tag", "1", "tust1"]) 
        .success()
        .stdout(contains("Added tag 'tust1' to group 'tust'"));

    ctx.cmd(&["group-add-tag", "1", "tust2"]) 
        .success()
        .stdout(contains("Added tag 'tust2' to group 'tust'"));

    ctx.cmd(&["group-list-tags", "1"]) 
        .success()
        .stdout(
            contains("2 tags in group 'tust':")
            .and(contains("tust1"))
            .and(contains("tust2"))
    );
}

#[test]
fn group_has_tag_no_group() {
    let ctx = TestContext::new();

    ctx.cmd(&["group-has-tag", "1", "tust"]) 
        .failure()
        .stderr(contains("Group with id 1 not found"));
}

#[test]
fn group_has_tag_false() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["create-group", "tust2"]) 
        .success()
        .stdout(contains("Group 'tust2'(id: 2) created"));

    ctx.cmd(&["group-add-tag", "2", "tust"]) 
        .success()
        .stdout(contains("Added tag 'tust' to group 'tust2'"));

    ctx.cmd(&["group-has-tag", "1", "tust"]) 
        .success()
        .stdout(contains("Group 'tust' does not have tag 'tust'"));
}

#[test]
fn group_has_tag_true() {
    let ctx = TestContext::new();

    ctx.cmd(&["create-group", "tust"]) 
        .success()
        .stdout(contains("Group 'tust'(id: 1) created"));

    ctx.cmd(&["group-add-tag", "1", "tust"]) 
        .success()
        .stdout(contains("Added tag 'tust' to group 'tust'"));

    ctx.cmd(&["group-has-tag", "1", "tust"]) 
        .success()
        .stdout(contains("Group 'tust' has tag 'tust'"));
}
