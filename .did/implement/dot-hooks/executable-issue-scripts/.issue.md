# Feature Specification: Executable Issue & Task Scripts in `did show`

## Overview
Allows task/issue files themselves to be executable scripts. When `did show <PATH>` is invoked on an executable task file (or executable sub-task script), `did` executes the script and prints its `stdout`/`stderr` output rather than printing raw text content.

## Use Cases
1. **Dynamic Assertion Tasks**: An issue or sub-item can be an executable test script (e.g. `check-coverage.sh` or `check-coverage`). Running `did show` executes the check dynamically to determine if requirements (e.g. 100% code coverage) are satisfied.
2. **Automated Verification**: Non-zero exit code during `did show` or `did done` indicates that automated verification for that task failed.

## Detailed Requirements
1. **Execution Check**:
   - Check if task file `<PATH>` has Unix executable permissions (`0o111`).
   - If **executable**:
     - Execute process setting standard environment variables (`DID_EVENT=show`, `DID_TARGET`, `DID_REPO_ROOT`, `DID_STATE_DIR`).
     - Print `stdout` and `stderr`.
     - Non-zero exit code aborts `did show` (or flags validation failure).
   - If **non-executable**:
     - Print raw file content as standard markdown text.
2. **Parent & Ancestor Directory Context**:
   - Continues ancestor `show` hook executions before running the target task script.
