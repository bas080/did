# did - File-System-Native Issue and Dependency Tracker

[![Coverage Status](https://img.shields.io/badge/coverage-100%25-brightgreen.svg)](https://github.com/bas080/did)

`did` is a lightweight, filesystem-native task and dependency tracker written in Rust. It manages issues, sub-tasks, and dependencies directly inside your directory structure without external databases or hidden state formats.

## Key Features

- **Filesystem State**: State is stored in `.did/` directories within your project repository, making tasks fully version-controllable with Git.
- **Hierarchical Tasks**: Directories represent task scopes and nesting. Pending tasks are visible files, while completed tasks are dot-prefixed (`.task`).
- **Dependency Symlinks**: Soft links (`did link`) express dependencies across directories and modules.
- **Actionable Tracking**: `did status` automatically identifies actionable leaf tasks whose sub-items/dependencies are all resolved.
- **Shell Auto-Completion**: Built-in completion generator for Bash, Zsh, Fish, PowerShell, and Elvish.

## Installation

### Direct Binary Download (Linux x86_64)

To install the **latest release**:
```bash
curl -sL -o did https://github.com/bas080/did/releases/download/latest/did-linux-x86_64
chmod +x did
sudo mv did /usr/local/bin/
did --help
```

To install the **latest development build** (pushed to main/master branch):
```bash
curl -sL -o did https://github.com/bas080/did/releases/download/development/did-linux-x86_64
chmod +x did
sudo mv did /usr/local/bin/
did --help
```

### Build from Source (Cargo)

Ensure you have Rust installed (1.80+), then build with Cargo:

```bash
cargo build --release
```

The compiled binary will be placed at `target/release/did`.

## Quick Start

### 1. Add Tasks
```bash
# Add task with inline content (auto-creates .did/ directory)
did add backend/auth/jwt.md -m "Implement JWT token validation"

# Add task using $EDITOR
did add frontend/ui/login.md
```

### 2. Link Dependencies
```bash
# Link a task into another directory as a dependency
did link backend/auth/jwt.md frontend/ui
```

### 3. Check Status
```bash
# List actionable tasks (unblocked leaf tasks)
did status

# List all tasks including blocked and completed tasks
did status -a
```

### 4. Inspect Task
```bash
did show backend/auth/jwt.md
```

### 5. Resolve Task
```bash
did done backend/auth/jwt.md
```
Prefixes the filename with a dot (`.jwt.md`), hiding it in filesystem listings and updating any dependent symlinks.

### 6. Mark Task Undone
```bash
did undone backend/auth/jwt.md
```
Removes the leading dot (`jwt.md`), restoring it as an open task.

### 7. Shell Completion
```bash
source <(did autocomplete bash)
```

## Environment Variables

| Variable | Description |
| :--- | :--- |
| `DID_STATUS_LIMIT` / `DID_LIMIT` | Sets the maximum number of items returned by `did status` or `did search` before displaying a truncation notice on `stderr`. |
| `DID_LOG_PATH` | Activates XML execution telemetry logging. Relative paths are resolved relative to the parent directory where `.did/` lives. |
| `EDITOR` | Specifies the text editor to invoke when running `did add PATH` without a `-m` message flag (defaults to `vi`). |

## Workflows & Prioritization

For task prioritization strategies (such as MoSCoW subtree folders like `.did/must/`, `.did/should/`, or task metadata headers), see [docs/prioritization.md](docs/prioritization.md).

## CLI Reference

```
SYNOPSIS
       did [FLAGS] [COMMAND] [ARGS...]

COMMANDS
       add            Create a task node or nested issue at PATH
       status         List actionables (leaf nodes with all sub-items done)
       show           Print task contents at PATH (requires sub-items done unless -a)
       done           Mark PATH as resolved (hides it; fails if sub-items remain open)
       undone         Mark a resolved PATH as open/undone (removes leading dot)
       link           Symlink TARGET into DEST directory using TARGET's basename
       mv             Move or rename a task file or directory at OLD_PATH to NEW_PATH
       autocomplete   Generate shell completion scripts (e.g. bash)

FLAGS
       -h, --help     Print help information
       -V, --version  Print version information
       -a, --all      Include resolved (hidden) tasks and blocked nodes
```

## License

MIT
