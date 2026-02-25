# [WIP] bioctl
**This is an experimental project exploring the viability of a hierarchical, reference-based database model for organizing large research datasets.**

bioctl is a lightweight tool for organizing large research datasets.\
It structures large collections of files using hierarchical groups and tags.\
bioctl does **not** move, copy, or modify files.

## Installation
```shell
cargo build --release
```
Binary will be located at:
```shell
target/release/bioctl
```

## Example Workflow (CLI)
Create project:
```shell
bioctl create-group my_project
```

Create a run group and add to project (using ids):
```shell
bioctl create-group first_run
bioctl group-add-child 1 2
```

Tracking files and add to run group:
```shell
bioctl track-files ./output/ -r -g 2
```

List all files of run group:
```shell
bioctl group-list-files 2
```

use help to see other useful commands
```shell
bioctl help
```

## Core Concepts
### Files
Files are stored as references (paths only).
bioctl does not manage file contents.

### Groups
Everything is a **Group**:
- project
- simulation run
- analysis step

Groups can contain:
- Files
- Other groups (with cycles explicitly disallowed)

The hierarchy forms a directed acyclic graph (DAG).\
Cycles between groups are explicitly disallowed.

### Tags:
Used as simple labels for easier ordering

## Architecture
- Rust
- SQLite backend
- CLI via clap (may change)

No file contents are stored or modified.\
All data is stored locally in a SQLite database.\
Use environment variable 'BIOCTL_DB_PATH' to set database path (:memory: for in memory db).

## Current Status
This project is under active development.

The interface and data model may change significantly.

### Implemented:
- Group creation / deletion
- File registration
- Group <-> File linking
- Group <-> Group hierarchy
- Basic tags
- CLI Interface (currently id-based; will change in the future)
- Integration testing using ```cargo test```

### Planned:
- Structured metadata system (key–value)
- Improved CLI (names instead of raw ids)
- Improved Group usability and navigation
- Web UI

## Design Goals:
- Zero configuration
- Single executable
- Local first
- No file content manipulation
- Automatic file registration
