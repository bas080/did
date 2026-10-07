# Feature Specification: Depth Limiting (`-L`) & `did tree` Visual Hierarchy Subcommand

## Overview
Adds a `-L <DEPTH>` / `--level <DEPTH>` flag to `did status` and introduces a separate `did tree [PATH]` subcommand for rendering ASCII tree directory structures.

## Detailed Requirements

### 1. `did status -L <DEPTH>`
- Constrains status traversal depth.
- When `-L 1` is specified, `did status` prints top-level actionable task files as well as **child directory paths** (e.g. `backend/auth/`) if actionable leaf tasks reside deeper inside that directory.

### 2. `did tree [PATH]` Subcommand
- Separate subcommand rendering task directories, files, and symlink dependencies in ASCII tree format.
- Accepts `-L <DEPTH>` to limit printed tree depth.
