# Feature Specification: `did search` Command

## Overview
The `did search` command enables users to query task items across the `.did/` tracker repository. It searches both task paths/filenames and task file contents, returning matching task relative paths in the same clean format as `did status`.

## Command Synopsis

```text
SYNOPSIS
       did search [FLAGS] <QUERY> [PATH]

DESCRIPTION
       Search task paths and task file contents for QUERY.

FLAGS
       -a, --all      Include resolved (hidden) tasks and blocked nodes in search
       -h, --help     Print help information
```

## Detailed Requirements

### 1. Query Matching
- **Scope**: Matches `QUERY` against both:
  1. The task relative path/filename (e.g., `backend/auth/jwt.md`).
  2. The text contents of the task file.
- **Case Sensitivity**: Case-insensitive substring search by default (e.g., searching `jwt` matches `JWT`, `Jwt`, or `backend/auth/jwt.md`).

### 2. Output Format
- Outputs matching relative task paths one per line (alphabetically sorted), identical to `did status` output.
- Example:
  ```text
  $ did search "jwt"
  backend/auth/jwt.md
  frontend/login.md
  ```

### 3. Filtering & `-a` / `--all` Flag Behavior
- **Default (without `-a`)**: Searches only open actionable task files (unblocked leaf nodes).
- **With `-a` / `--all`**: Searches all tasks across the repository or subtree, including blocked nodes and resolved (`.`) dot-prefixed files.

### 4. Status Limit Integration
- Respects the status limit environment variable (`DID_STATUS_LIMIT` / `DID_...`).
- When matching results exceed the defined limit, outputs the truncated list up to the limit on `stdout` and prints a notice to `stderr` indicating truncation.
