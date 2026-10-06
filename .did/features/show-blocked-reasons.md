Feature Specification: Display Blocking Reasons on 'show' and 'done'

## Overview
When tasks are blocked by unresolved dependencies (unresolved child tasks in deeper subdirectories or open symlinks), users need clear diagnostic feedback explaining exactly why a task cannot be viewed (`did show`) or marked done (`did done`).

Note: Direct sibling task files in the same directory are independent leaf nodes and do NOT block each other. A task is blocked only by unresolved items in child subdirectories below it or by open symlinks.

For `did status`, it simply prints actionable open tasks (or all tasks with `-a`). If no open tasks are found under `status`, it prints an informative message to stderr and exits with code 0.

## Detailed Requirements

### 1. `did show <PATH>`
When `did show` is called on a task file that has open/unresolved sub-items (and `-a` is NOT supplied):
- Exit Code: Non-zero (`1`).
- Error Output (stderr):
  ```
  error: task 'backend/auth/jwt.md' is blocked by unresolved sub-items:
    - backend/auth/sub/research.md (unresolved task in child directory)
    - backend/auth/dep.md -> ../specs/jwt.md (open symlink dependency)
  ```

### 2. `did done <PATH>`
When `did done` is called on a task file that has open/unresolved sub-items:
- Exit Code: Non-zero (`1`).
- Error Output (stderr):
  ```
  error: cannot mark task 'backend/auth/jwt.md' done: unresolved sub-items remain:
    - backend/auth/sub/c.md
  ```

### 3. `did status [PATH]`
- `did status` simply lists open actionable tasks line-by-line.
- If no open actionable tasks are found, it prints `No actionable tasks found.` to stderr and exits with exit code `0`.
