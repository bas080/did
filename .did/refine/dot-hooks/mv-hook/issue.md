# Feature Specification: `mv` Lifecycle Hook (`.hooks/mv`)

## Overview
Defines the lifecycle hook for `did mv <OLD_PATH> <NEW_PATH>`. The `mv` hook allows project maintainers and teams to enforce move policies, permissions, or structural constraints before any task file or task directory is relocated inside `.did/`.

---

## Detailed Requirements

### 1. Hook Location & Invocation
- Location: `.hooks/mv` (or `.hooks/mv.sh`, `.hooks/mv.py`, etc.).
- When `did mv <OLD_PATH> <NEW_PATH>` is called, `did` checks ancestor directories for executable `mv` hooks before proceeding with the move.

### 2. Command Line Arguments
- The `mv` hook script receives exactly two positional arguments:
  ```bash
  .hooks/mv <OLD_PATH> <NEW_PATH>
  ```
- `OLD_PATH`: Source relative path inside `.did/`.
- `NEW_PATH`: Target relative path inside `.did/`.

### 3. Exit Code & Enforcement
- **Exit Code 0**: Move is permitted. `did mv` continues execution.
- **Non-Zero Exit Code**: Move is disallowed. `did mv` aborts immediately, performs no file operations, and exits with non-zero status.

---

## Open Clarifying Questions for Refinement
1. **Invocation Order**: Should `.hooks/mv` run before validating that `OLD_PATH` exists, or only after confirming valid paths?
2. **Directory vs File Moves**: If `OLD_PATH` is a directory containing multiple sub-tasks, should `.hooks/mv` run once for the top directory or once per contained file?


## Open Questions for Refinement (@bas080)
1. @bas080 Should the `mv` hook execute before or after relative symlinks are updated in `.did/`?
