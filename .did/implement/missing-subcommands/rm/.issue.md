# Feature Specification: `did rm` / `did remove` Subcommand & `remove` Lifecycle Hook

## Overview
Adds a `did rm` subcommand (with alias `did remove`) and `remove` lifecycle hook (`.hooks/remove`).
Safely removes task files or directories from `.did/` while enforcing directory deletion safety flags (`-r` / `--recursive`) and executing `remove` lifecycle hooks.

## Detailed Requirements

### 1. Command Syntax & Aliases
```bash
did rm [FLAGS] <PATH>
did remove [FLAGS] <PATH>
```

### 2. Flags & Directory Safety
- **Single File Removal**: `did rm task.md` removes a single task file.
- **Directory Removal Protection**: Attempting to remove a directory without `-r` or `--recursive` fails with an error:
  `error: 'path/to/dir' is a directory. Use 'did rm -r path/to/dir' to remove recursively.`
- **Recursive Removal (`-r` / `--recursive`)**: Safely removes a task directory and its contained sub-items.

### 3. `remove` Lifecycle Hook (`.hooks/remove`)
- Before removing `<PATH>`, `did` executes ancestor `remove` hooks (`.hooks/remove` or `hooks/remove`).
- Exported environment variables: `DID_EVENT="remove"`, `DID_TARGET`, `DID_REPO_ROOT`, `DID_STATE_DIR`.
- Non-zero exit code aborts removal immediately.
