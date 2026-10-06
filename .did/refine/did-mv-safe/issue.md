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

---

## Required Unit & Integration Test Suite (`tests/cli_tests.rs`)

To ensure `did mv` is robust and safe, implementation must be verified with the following test cases:

1. **`test_did_mv_single_file_updates_inbound_symlinks`**:
   - Create task file `backend/auth/jwt.md` and link it into `frontend/jwt.md`.
   - Execute `did mv backend/auth/jwt.md backend/auth/token.md`.
   - Assert `backend/auth/jwt.md` no longer exists and `backend/auth/token.md` exists.
   - Assert `frontend/jwt.md` symlink target is updated to `../backend/auth/token.md`.

2. **`test_did_mv_directory_updates_inbound_and_outbound_symlinks`**:
   - Create folder `backend/auth/` containing task `jwt.md` and an outbound symlink `spec.md -> ../../docs/spec.md`.
   - Create inbound symlink `frontend/jwt.md -> ../backend/auth/jwt.md`.
   - Execute `did mv backend/auth backend/security`.
   - Assert `backend/auth/` no longer exists and `backend/security/jwt.md` exists.
   - Assert inbound symlink `frontend/jwt.md` points to `../backend/security/jwt.md`.
   - Assert outbound symlink `backend/security/spec.md` target is updated relative to `backend/security/` (`../../docs/spec.md`).

3. **`test_did_mv_destination_collision_fails`**:
   - Create files `a.md` and `b.md`.
   - Execute `did mv a.md b.md`.
   - Assert command fails (exit code non-zero) and `stderr` contains `destination path already exists`.

4. **`test_did_mv_auto_creates_parent_directories`**:
   - Create task `a.md`.
   - Execute `did mv a.md nested/deep/folder/a.md`.
   - Assert parent directories `nested/deep/folder/` are created and `nested/deep/folder/a.md` exists.

5. **`test_did_mv_resolved_dot_files`**:
   - Create completed task `.jwt.md` and symlink `.jwt.md -> ../backend/auth/.jwt.md`.
   - Execute `did mv backend/auth/.jwt.md backend/auth/.token.md`.
   - Assert symlinks pointing to `.jwt.md` are updated to `.token.md`.

6. **`test_did_mv_non_existent_source_fails`**:
   - Execute `did mv non_existent.md target.md`.
   - Assert exit code non-zero and error printed on `stderr`.
