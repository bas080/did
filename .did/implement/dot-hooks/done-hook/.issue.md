# Feature Specification: `done` Lifecycle Hook (`.hooks/done`)

## Overview
Defines the `done` lifecycle hook (`.hooks/done`, `.hooks/done.sh`, `.hooks/done.py`, `.hooks/done.md`). The `done` hook executes immediately before a task is marked resolved (`did done <PATH>`), after all prerequisite checks (e.g. checking that no open child sub-items or broken dependencies remain) have succeeded.

---

## Detailed Requirements

### 1. Hook Location & Invocation Timing
- Location: `.hooks/done` (or `.hooks/done.sh`, `.hooks/done.py`, etc.).
- When `did done <PATH>` is called:
  1. `did` first verifies that `<PATH>` exists, is a task file, and has no unresolved sub-items.
  2. If prerequisite checks pass, `did` checks ancestor directories for executable `done` hooks.
  3. If an executable `done` hook is found, `did` executes it before renaming the task file to `.task.md`.

### 2. Arguments Passed
- The `done` hook script receives the task path being marked done as a positional argument:
  ```bash
  .hooks/done <TASK_PATH>
  ```

### 3. Exit Code Enforcement
- **Exit Code 0**: Task completion proceeds. `did done` renames the file with a leading dot (`.`) and updates symlinks.
- **Non-Zero Exit Code**: Task completion is **ABORTED**. The task file is **NOT** marked done, and `did done` exits with a non-zero exit code.

### 4. Output Reporting & AI Reminders
- The `done` hook script can print output to `stdout` and `stderr`.
- `did done` forwards all hook output to `stdout`/`stderr`.
- **Primary Use Case**: The `done` hook can be configured to remind AI agents and developers to log new `did` issues for any discovered bugs, missing test cases, or follow-up work encountered during implementation before finalizing the task.

---

## Example `.hooks/done.sh` Script
```bash
#!/bin/bash
TASK_PATH="$1"
echo "[HOOK] Pre-completion check for task: $TASK_PATH"
echo "[REMINDER] Ensure any newly discovered bugs or follow-up tasks have been added using 'did add'."
exit 0
```
