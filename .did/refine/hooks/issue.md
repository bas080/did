# Feature Specification: `did` Lifecycle Hooks System

## Overview
A unified lifecycle hooks system for `did`. Users and teams can define executable hook scripts at the repository root (`.did/hooks/`) or within specific directory subtrees (`dir/hooks/`) to run custom validation assertions or output dynamic contextual instructions during `did` command execution.

## Supported Lifecycle Hooks

### 1. `show` Hook
- **Trigger**: Executed when `did show <PATH>` is called.
- **Behavior**:
  - Any `stdout` emitted by the hook is printed as dynamic contextual guidance above task content.
  - Non-zero exit code aborts `did show` with an error diagnostic.

### 2. `status` Hook
- **Trigger**: Executed during `did status [PATH]`.
- **Behavior**:
  - Allows directory hooks to filter actionable tasks dynamically.
  - Can abort or log warnings to `stderr` if directory assertions fail.

### 3. `done` Hook
- **Trigger**: Executed when `did done <PATH>` is called.
- **Behavior**:
  - Enforces custom completion prerequisites before marking a task resolved.
  - Non-zero exit code prevents marking the task done.

### 4. `add`, `link`, and `mv` Hooks
- **`add` Hook**: Validates task file naming conventions or initial content formats.
- **`link` Hook**: Enforces custom dependency graph constraints.
- **`mv` Hook**: Validates task relocation or renaming assertions.

## Hook Discovery & Directory Reservation Rules

1. **Exact Filename Matching**:
   - Hooks are executable files named strictly after the event (e.g. `show`, `status`, `done`, `add`, `link`, `mv`).
   - Do **NOT** look for `.sh` extensions as a fallback.

2. **`hooks` Directory Behavior**:
   - `hooks` is a reserved directory name for containing lifecycle hook scripts.
   - It is permitted to define task issues inside a `hooks/` directory.
   - If no executable hook scripts are found inside a `hooks/` directory, it behaves as a standard task directory.

3. **Discovery Hierarchy**:
   - Discovered in top-down order from root `.did/hooks/<event>` down to local directory hooks (`dir/hooks/<event>`).

## Post-Implementation Requirement
- **Documentation Issue**: Once the hooks system implementation is complete, create a task/issue to document the hooks system and examples in `README.md`.
