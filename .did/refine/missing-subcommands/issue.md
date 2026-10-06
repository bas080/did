# Proposal & Research Issue: Native `did` Subcommands for Common Issue Management Operations

## Overview
Analyzes the bash commands used outside of `did` during developer and AI agent workflows to manage issues, state directories, and issue lifecycles. Proposes new `did` subcommands to eliminate the need for raw shell commands like `mv`, `rm`, `mkdir`, and `find`.

---

## Analysis of Outside Shell Commands Used

### 1. File & Folder Relocation (`mv`)
- **Outside Command**: `mv .did/implement/vague-issue/ .did/refine/vague-issue/`
- **Use Case**: Moving issues between staging (`refine/`) and implementation (`implement/`), reorganizing folder structures, or renaming tasks.
- **Proposed `did` Subcommands**:
  - **`did mv <OLD_PATH> <NEW_PATH>`**: Moves task files or directories, updating relative symlink references throughout `.did/`.
  - **`did demote <PATH>`**: Shortcut to move a task from `.did/implement/<PATH>` back to `.did/refine/<PATH>`.
  - **`did promote <PATH>`**: Shortcut to move a task from `.did/refine/<PATH>` into `.did/implement/<PATH>`.

### 2. Task Deletion (`rm` / `rmdir`)
- **Outside Command**: `rm .did/refine/obsolete-issue.md`
- **Use Case**: Removing duplicate or cancelled task files.
- **Proposed `did` Subcommand**:
  - **`did rm <PATH>`**: Safely removes a task file or directory, warning if other tasks contain active symlinks pointing to the removed item.

### 3. Structural Hierarchy Inspection (`find` / `tree`)
- **Outside Command**: `find .did -maxdepth 2 -type d`
- **Use Case**: Inspecting category subdirectories and nested task hierarchies.
- **Proposed `did` Subcommand**:
  - **`did tree [PATH]`**: Visual directory tree showing tasks, sub-items, and dependency links.
