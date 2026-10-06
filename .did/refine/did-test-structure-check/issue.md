# Feature Proposal: `did test` Structure & Integrity Check Command

## Overview
Add a `did test` subcommand that verifies the integrity and structure of the `.did/` state directory.

## Requirements

### 1. Built-in Sanity Checks
`did test` automatically executes built-in integrity assertions:
- **Broken Symlink Detection**: Verifies all symlinks in `.did/` point to valid existing target paths. Outputs an error for any dangling or broken symlinks.
- **Dangling Dot-Files**: Checks for orphaned completed files (`.task.md`) that lack expected target associations or corrupted names.
- **Directory Structure Sanity**: Verifies `.did/` root structure and permissions.

### 2. User-Defined Test Scripts
Users can define custom validation scripts (e.g. `.did/test.sh` or `.did/hooks/test`) that run automatically when `did test` is called:
- Executes user script and checks its exit code.
- Allows teams to enforce project-specific structural conventions (e.g. "All issues must have a `.md` extension", "No unrefined tasks in `implement/`").

### 3. Exit Codes & Diagnostics
- Returns exit code `0` if all built-in checks and user test scripts pass.
- Returns non-zero exit code (`1`) and outputs descriptive error diagnostics to `stderr` if any check fails.
