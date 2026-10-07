# Feature Proposal: Clarifying Dependency Link Directionality (`did link`)

## Problem Statement
AI agents and developers sometimes get confused about dependency directionality when using `did link`:
Does `did link A B` mean "A depends on B" or "B depends on A"?

## Current Behavior & Rules
- `did link TARGET DEST_DIR` creates a relative symlink pointing to `TARGET` inside `DEST_DIR`.
- Because `DEST_DIR` contains an open symlink `TARGET`, **tasks inside `DEST_DIR` are blocked by `TARGET`**.
- Therefore: **`TARGET` is the prerequisite / blocker, and tasks inside `DEST_DIR` are the dependent / blocked items.**

## Proposals for Improvement
1. **Explicit Alias / Subcommands**:
   - `did depends-on <PREREQUISITE_TASK> <DEPENDENT_TASK_OR_DIR>`
   - `did block <BLOCKED_TASK_OR_DIR> --by <PREREQUISITE_TASK>`
2. **Help Guidance & Diagnostic Messages**:
   - In `did show` and `did status`, print explicit diagnostic messages when tasks are blocked by symlinks:
     `error: task 'frontend/login.md' is blocked by prerequisite link: 'backend/auth/jwt.md'`
