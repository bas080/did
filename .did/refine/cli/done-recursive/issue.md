# Feature Specification: `did done -r` Recursive Directory Completion

## Overview
Proposes adding a recursive flag (`-r` / `--recursive`) to `did done` to allow completing all open tasks inside a directory recursively:
```bash
did done -r <DIRECTORY>
```

## Requirements
1. Running `did done -r <DIRECTORY>` recursively finds all open task files inside `<DIRECTORY>` and marks them resolved (dot-prefixes file names and updates referencing symlinks).
2. Executes ancestor `done` hooks for each completed task.
3. Fails if any task in the directory is blocked by unresolved external prerequisites outside `<DIRECTORY>`.

@bas080
