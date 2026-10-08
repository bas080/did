# Feature Specification: `did commit` Git Integration

## Overview
To maintain a tight synchronization between the project state (the `.did/` directory) and the version control history, this proposal introduces `did commit`. This allows users to commit the current state of their task tracker directly to Git.

## Proposed Behavior
`did commit` should create a Git commit of all changes within the `.did/` directory.

### 1. Commit Message Logic
- Instead of a simple message flag, `did commit` should use the contents of the relevant issue (e.g., the task being marked as done or the most recently modified issue) as a template for the Git commit message.
- A `-m "Message"` flag can still be provided to override this behavior.

### 2. Optional Integration (Disabled by Default)
To keep `did` lightweight and avoid forcing Git dependencies on all users:
- Integration is disabled by default.
- It requires a configuration setting to be enabled.

## Blockers
This feature is **blocked** by the implementation of a general configuration file system for `did` (to handle the enabled/disabled state of Git integration).
