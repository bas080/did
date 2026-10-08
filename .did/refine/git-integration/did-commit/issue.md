# Feature Specification: `did commit` Git Integration

## Overview
To maintain a tight synchronization between the project state (the `.did/` directory) and the version control history, this proposal introduces `did commit`. This allows users to commit the current state of their task tracker directly to Git.

## Proposed Behavior
`did commit` should create a Git commit of all changes within the `.did/` directory.

### 1. Standalone Command
- `did commit -m "Message"`: Commits all changes in `.did/` with the provided message.
- If no message is provided, it can either fail or use a default message like "did: update task state".

### 2. Optional Integration (Disabled by Default)
To keep `did` lightweight and avoid forcing Git dependencies on all users:
- Integration should be optional.
- A config setting (e.g., in a `.didrc` or environment variable `DID_GIT_SYNC=true`) enables this behavior.
- If disabled, `did commit` should return a helpful message explaining how to enable it.

### 3. Integration with `did done`
Optionally, `did done <PATH>` could support a `--commit` flag to automatically trigger a commit upon task completion.

@bas080
