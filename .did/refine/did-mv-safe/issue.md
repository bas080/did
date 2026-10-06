# Feature Specification: Safe `did mv` Command

## Overview
The `did mv <OLD_PATH> <NEW_PATH>` command provides a safe, atomic mechanism to move or rename task files and task directories inside `.did/` without leaving broken symlinks or invalid relative references anywhere in the repository.

## Detailed Requirements

### 1. File & Directory Support
- Supports moving a single task file (e.g. `did mv backend/auth.md backend/authentication.md`).
- Supports moving an entire task directory (e.g. `did mv backend/auth backend/security`).

### 2. Automatic Symlink Target Updating
When `OLD_PATH` is moved to `NEW_PATH`:
- **Inbound Symlinks**: Scan `.did/` for any symlinks pointing to `OLD_PATH` (or paths under `OLD_PATH` if moving a directory).
  - Update their target paths to point to the corresponding location under `NEW_PATH`.
  - Recompute relative symlink targets from each symlink's containing folder.
- **Outbound Symlinks**: If moving a directory that *contains* symlinks pointing outward to other tasks:
  - Recompute relative symlink targets inside the moved directory so they continue to resolve correctly from `NEW_PATH`.

### 3. Collision Prevention & Directory Creation
- If `NEW_PATH` already exists as a file or directory, return an error (`error: destination path already exists: NEW_PATH`) unless moving into an existing directory.
- Automatically create parent directories for `NEW_PATH` as needed (`mkdir -p`).

### 4. Atomic Rollback on Failure
- Perform pre-checks on permissions and destination paths before mutating the filesystem.
- If symlink updating or file move fails midway, abort and report an error without corrupting the state.
