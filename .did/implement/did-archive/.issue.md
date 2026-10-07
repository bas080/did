# Feature Specification: `did archive` Hidden Command

## Overview
Adds the `did archive [PATH]` subcommand to clean up resolved (dot-prefixed) task files and directories by moving them into `.did/.archive/`.

## Detailed Requirements

### 1. Command Visibility
- **Hidden CLI Subcommand**: `did archive` is marked with `#[command(hide = true)]` so it is not listed in `did --help` output.

### 2. Archive Location
- Moves completed task files into `.did/.archive/` (hidden state folder) while preserving original relative directory structures.

### 3. `.hooks` Preservation Guard
- **Do NOT move directories that contain `.hooks` or `hooks` subdirectories**, preserving parent directory context and lifecycle hook scripts.
