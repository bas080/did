# Proposal: Explicit Issue & Task Prioritization System

## Overview
Explores explicit prioritization mechanisms for task nodes within `did`. While prefixing directory names with numbers (e.g., `01-auth/`, `02-api/`) forces alphabetical sorting, it is overly implicit, rigid, and makes reordering tasks tedious. This proposal explores explicit, developer-friendly methods to declare and query issue priorities.

---

## Explored Alternatives

### Alternative 1: Priority Headers in Task Metadata (`priority: high`)
- Task Markdown files declare explicit priority fields at the top of the file:
  ```markdown
  ---
  priority: 1
  ---
  # Task Description
  ```
- Or simple key-value header: `Priority: High` / `Priority: P0` / `Priority: 1`.
- `did status` sorts actionable tasks by priority level before path name.

### Alternative 2: Priority Directory Categorization (Priority Subtrees)
- Organize issues into top-level explicit priority subfolders within `.did/`:
  - `.did/must/` (P0 / Critical)
  - `.did/should/` (P1 / High)
  - `.did/could/` (P2 / Normal)
- `did status` traverses `.did/must/` first, ensuring high-priority tasks always appear at the top of actionable listings.

### Alternative 3: `did status --sort=priority` Flag
- Adds explicit sorting flags to `did status`:
  - `did status --sort=priority`
  - `did status --priority=P0`

---

## Open Questions for Refinement (@bas080)
1. @bas080 Should priority be declared in task file content (e.g. metadata header `Priority: P0`) or via directory placement (`.did/must/`)?
2. @bas080 Should `did status` by default sort by priority before alphabetical path sorting?
