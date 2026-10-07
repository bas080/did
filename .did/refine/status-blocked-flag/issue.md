# Feature Specification: `-b` / `--blocked` Flag for `did status` & `did query`

## Overview
Adds a `-b` / `--blocked` flag to `did status` and `did query` to list **only** blocked tasks (tasks with unresolved child sub-items or broken dependencies).

## Detailed Requirements

### 1. Mutual Exclusivity with `-a` / `--all`
- Passing both `-a` and `-b` (e.g. `did status -a -b`) **fails with a non-zero exit code** and prints an error message to `stderr`:
  `error: flags '-a/--all' and '-b/--blocked' are mutually exclusive and cannot be used together.`

### 2. Truncation Limit Handling
- Output from `did status -b` respects `DID_STATUS_LIMIT` environment variable.

### 3. Subtree Path Filtering
- `did status -b [PATH]` filters blocked tasks within `PATH`.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `did status -b` list blocked tasks grouped by blocking dependency or flat list?
