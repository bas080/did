# Feature Specification: `did` Lifecycle Hooks System

## Overview
Consolidate command event hooks into a unified lifecycle hooks system. Users and teams can define executable scripts at the repository level (`.did/hooks/`) or within specific directory subtrees to run custom checks, enforce structural constraints, or output dynamic contextual instructions during `did` command execution.

## Supported Lifecycle Hooks

### 1. `show` Hook
- **Trigger**: Executed when `did show <PATH>` is called.
- **Behavior**:
  - Scans parent and directory nodes for hook scripts.
  - Any `stdout` emitted by the hook is printed as dynamic contextual guidance above task content.
  - If the hook returns a non-zero exit code, `did show` aborts with an error diagnostic.

### 2. `status` Hook
- **Trigger**: Executed during `did status [PATH]`.
- **Behavior**:
  - Allows directory-level hooks to filter actionable tasks dynamically (e.g. checking git branch, build status, or external dependency checks).
  - Can abort or log warnings to `stderr` if directory integrity assertions fail.

### 3. `done` Hook (Pre-Completion)
- **Trigger**: Executed when `did done <PATH>` is called.
- **Behavior**:
  - Enforces custom completion checks (e.g. "All unit tests must pass before marking done").
  - Non-zero exit code prevents marking the task resolved.

### 4. `add`, `link`, `mv`, and `test` Hooks
- **`add` Hook**: Validates task file naming conventions or initial content formats upon creation.
- **`link` Hook**: Enforces custom dependency graph constraints (e.g. preventing illegal module dependencies).
- **`test` Hook**: Runs repo-wide structure assertions, broken symlink detectors, and user test scripts.

## Hook Location & Discovery Hierarchy
Hooks are discovered in order from root `.did/hooks/<event>` down to local directory hooks (`dir/hooks/<event>` or `dir/<event>.sh`).
