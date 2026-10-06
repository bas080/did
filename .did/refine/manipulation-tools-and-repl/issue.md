# Feature Proposal: `.did` Manipulation Tools & Interactive REPL Shell

## Overview
To minimize the chance of corrupting `.did/` state directories when manually moving, linking, or editing tasks, `did` needs dedicated manipulation subcommands and an interactive REPL shell mode.

## Detailed Proposals

### 1. High-Level Manipulation Subcommands
Add safe CLI operations that abstract direct file/folder operations:
- `did rm <PATH>`: Safely remove a task file or directory while cleaning up pointing symlinks across `.did/`.
- `did cp <PATH> <DEST>`: Duplicate a task or task folder structure while resolving relative dependencies.
- `did clean`: Remove dangling or orphaned symlinks automatically.

### 2. Interactive Bash REPL Shell (`did repl` / `did shell`)
Provide an interactive shell mode where `did` subcommands act as first-class citizens without needing to prefix `did` before every command:
- **Interactive Prompt**:
  ```text
  did (main)> status
  backend/auth/jwt.md

  did (main)> show backend/auth/jwt.md
  ...

  did (main)> done backend/auth/jwt.md
  ```
- **Bash Integration**:
  - Subcommands (`add`, `show`, `status`, `done`, `link`, `search`) execute directly as top-level shell commands.
  - Standard Bash commands (`ls`, `cd`, `cat`, `grep`, `pwd`) remain accessible inside the REPL environment.
  - Command line history, tab auto-completion, and readline support built-in.
