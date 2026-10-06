Feature Specification: Display Blocking Reasons on 'show', 'done', and 'status'

## Overview
When tasks are blocked by unresolved dependencies (unresolved child tasks in subdirectories or open symlinks), users need clear diagnostic feedback explaining exactly why a task cannot be viewed (`did show`), marked done (`did done`), or why it isn't actionable in `did status`.

## Detailed Requirements

### 1. `did show <PATH>`
When `did show` is called on a task file that has open/unresolved sub-items (and `-a` is NOT supplied):
- Exit Code: Non-zero (`1`).
- Error Output (stderr):
  ```
  error: task 'backend/auth/jwt.md' is blocked by unresolved sub-items:
    - backend/auth/research.md
    - backend/auth/dep.md -> ../specs/jwt.md
  ```

### 2. `did done <PATH>`
When `did done` is called on a task file that has open/unresolved sub-items:
- Exit Code: Non-zero (`1`).
- Error Output (stderr):
  ```
  error: cannot mark task 'backend/auth/jwt.md' done: unresolved sub-items remain:
    - backend/auth/sub/c.md
  ```

### 3. `did status <PATH>` on Blocked Tasks
When `did status <PATH>` is run on a specific task file or directory subtree where items are blocked:
- If a specific task path is passed (e.g., `did status backend/auth/jwt.md`) and it is blocked, `did status` lists the task alongside its blocking dependencies:
  ```
  backend/auth/jwt.md (blocked by: backend/auth/sub/c.md)
  ```
- Default `did status` without arguments continues to output actionable (unblocked) leaf tasks line-by-line.
