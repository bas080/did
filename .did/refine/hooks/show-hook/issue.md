# Feature Specification: `show` Hook & Context Provider

## Overview
Replaces `agents.md` with `hooks/show`. The `show` hook provides both dynamic script execution and static parent directory context during `did show <PATH>`.

## Behavior Requirements

### 1. Discovery Hierarchy
- During `did show <PATH>`, `did` traverses ancestor directories top-down from root `.did/` down to `<PATH>`'s parent folder.
- In each ancestor directory `dir`, `did` checks for the presence of `.did/hooks/show` or `dir/hooks/show`.

### 2. Executable Permission Check
For each discovered `hooks/show` file:
- **If Executable** (Unix file permission bit `0o111` set):
  - Execute the script.
  - Print its `stdout` output directly to `stdout`.
  - If the script exits with a non-zero exit code, abort `did show` immediately with an error diagnostic.
- **If Non-Executable**:
  - Print its relative path header (e.g. `refine/hooks/show`).
  - Print its file contents directly to `stdout`.

### 3. Replacement of `agents.md`
- Context files previously named `agents.md` or `AGENTS.md` are deprecated in favor of `hooks/show`.
- Non-executable `hooks/show` files serve as static contextual instructions for human developers and AI agents.
