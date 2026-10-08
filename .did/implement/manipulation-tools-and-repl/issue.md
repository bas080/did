# Feature Proposal: `.did` Manipulation Tools & Interactive REPL Shell

## Overview
To minimize the chance of corrupting `.did/` state directories when manually moving, linking, or editing tasks, `did` needs dedicated manipulation subcommands and an interactive REPL shell mode.

## Detailed Proposals

### 1. High-Level Manipulation Subcommands
Add safe CLI operations that abstract direct file/folder operations:
- `did rm <PATH>`: Safely remove a task file or directory while cleaning up pointing symlinks across `.did/`.
- `did cp <PATH> <DEST>`: Duplicate a task or task folder structure while resolving relative dependencies.
- `did clean`: Remove dangling or orphaned symlinks automatically.

### 2. Shell Integration & Aliases
Instead of a dedicated TUI/REPL binary, `did` will provide a mechanism for users to integrate subcommands directly into their environment via shell aliases.
- Provide a command (e.g., `did shell-init`) that outputs the necessary alias definitions for the user's current shell (Bash/Zsh).
- This allows users to source the aliases into their `.bashrc` or `.zshrc`, enabling them to run `status`, `show`, `done`, etc., as first-class citizens in their native terminal.
- This approach maintains the "coding agent first" philosophy by avoiding binary bloat and staying compatible with existing shell workflows.
