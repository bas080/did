# Feature Specification: `did` Lifecycle Hooks System

## Overview
Replaces `agents.md` with `hooks/<event>`. A unified lifecycle hooks system for `did`. Users and teams can define hook files at the repository root (`.did/hooks/`) or within specific directory subtrees (`dir/hooks/`).

## Executable Permission Check Rule
- If a hook file (e.g. `hooks/show`, `hooks/status`) is **executable** (Unix mode bit `0o111` set), `did` executes the script.
- If a hook file is **non-executable**, `did` prints its relative path header and file content directly to `stdout`.

## Supported Lifecycle Hooks

### 1. `show` Hook (`hooks/show`)
- **Trigger**: Executed when `did show <PATH>` is called.
- **Behavior**:
  - If executable: runs script and prints its `stdout`. Non-zero exit code aborts `did show`.
  - If non-executable: prints relative path header and file content as static guidance.

### 2. `status` Hook (`hooks/status`)
- **Trigger**: Executed during `did status [PATH]`.
- **Behavior**:
  - If executable: allows dynamic task filtering or pre-status checks.

### 3. `done` Hook (`hooks/done`)
- **Trigger**: Executed when `did done <PATH>` is called.
- **Behavior**:
  - Enforces pre-completion assertions (e.g., verifying tests pass before completing a task).

### 4. `add`, `link`, and `mv` Hooks
- **`add` Hook**: Validates task file naming conventions or initial content formats.
- **`link` Hook**: Enforces custom dependency graph constraints.
- **`mv` Hook**: Validates task relocation or renaming assertions.

## Hook Discovery & Reservation Rules
1. **Exact Filename Matching**:
   - Hooks are files named strictly after the event inside a `hooks/` folder (`hooks/show`, `hooks/status`, `hooks/done`, `hooks/add`, `hooks/link`, `hooks/mv`).
2. **`hooks` Directory Behavior**:
   - `hooks` is a reserved directory name for defining hook files and scripts.
   - Tasks and issues are permitted inside `hooks/` directories.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `did` enforce execution timeout limits (e.g. 5s) on hook scripts to prevent hanging invocations?
