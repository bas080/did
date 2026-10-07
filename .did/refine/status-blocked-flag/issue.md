# Feature Specification: `-b` / `--blocked` Flag for `did status` & `did search`

## Overview
Proposes adding a `-b` / `--blocked` flag to `did status` and `did search`. While `-a` / `--all` includes all tasks (actionable, resolved, and blocked), `-b` selectively lists **only** blocked tasks (tasks that have unresolved sub-items in deeper subdirectories).

---

## Detailed Requirements

### 1. Flag Definition
- `-b` / `--blocked` flag added to `did status` and `did search`.
- Can be combined with subtree path filters (e.g., `did status -b backend`).

### 2. Output Rules
- Displays only tasks that are blocked by unresolved child subdirectories or broken dependencies.
- Prints `No blocked tasks found.` to `stderr` if no blocked tasks exist in the selected subtree.

---

## Open Clarifying Questions for Refinement (@bas080)
1. **Precedence with `-a`**: @bas080 If both `-a` and `-b` are passed, should `-b` strictly override `-a` (showing only open blocked tasks) or filter `-a` (showing resolved blocked tasks as well)?
2. **Limit Behavior**: @bas080 Should blocked task results count towards `DID_STATUS_LIMIT` truncations?
