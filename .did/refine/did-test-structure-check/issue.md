# Feature Proposal: Directory Script Execution on `did show` and Integrity Checks

## Overview
Allow users to define custom executable scripts in directory nodes that run automatically when `did show` is called on a child or sibling task. These scripts can enforce structural assertions and conditionally output custom instructions.

## Requirements

### 1. `did show` Directory Script Hooks
When `did show <PATH>` is executed:
- `did` inspects parent and sibling directories for executable check/instruction scripts (e.g., `check.sh`, `instructions.sh`, or `.did/hooks/show`).
- **Execution & Output**:
  - The script is executed prior to displaying the task content.
  - Any stdout emitted by the script is displayed as conditional dynamic instructions.
  - If the script exits with a non-zero exit code, `did show` aborts with an error, allowing directory-level validation assertions to be enforced.

### 2. Built-in Sanity Checks (`did test`)
`did test` performs built-in directory integrity checks:
- **Broken Symlink Detection**: Verifies all symlinks in `.did/` point to valid existing target paths. Outputs errors for any dangling or broken symlinks.
- **Directory Structure Sanity**: Verifies `.did/` root structure and permissions.
