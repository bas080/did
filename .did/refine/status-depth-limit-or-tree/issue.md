# Proposal: Depth Limiting (`-L` / `--level`) & Visual Hierarchy (`did tree`)

## Overview
Proposes mechanisms to constrain traversal depth when querying task status or inspecting task hierarchies. Large repositories with deeply nested task subtrees can generate overwhelming output; depth limiting allows developers and AI agents to inspect top-level domain directories without being flooded by deeply nested sub-tasks.

---

## Detailed Requirements & Alternatives

### Option A: `did status -L <DEPTH>` / `did status --level <DEPTH>`
- Adds a `-L` / `--level <DEPTH>` flag to `did status`.
- **`-L 1`**: Displays only top-level task files or immediate child directories containing actionable tasks.
- **`-L 2`**: Limits directory traversal to 2 directory levels down from `.did/` (or the target subtree path).

### Option B: `did tree [PATH]` Subcommand
- Introduces a visual tree representation similar to the Unix `tree` command:
  ```
  .did/backend
  ├── auth/
  │   └── jwt.md
  └── db/
      └── migration.md
  ```
- Supports `-L <DEPTH>` to limit the printed directory depth.
- Color-codes actionable, blocked, and closed tasks.

---

## Open Questions for Refinement (@bas080)
1. @bas080 Should `-L 1` in `did status` print directory paths (e.g. `backend/auth/`) when actionable leaf tasks reside deeper inside, or only task files?
2. @bas080 Is `did tree` better suited as a separate subcommand rather than overloading `did status`?
