# Feature Specification: `did archive` Command

## Overview
Proposes the `did archive [PATH]` subcommand to clean up resolved (dot-prefixed) task files and directories by moving them into an archive directory (e.g., `.did/archive/` or `.did/.archive/`).

---

## Detailed Requirements

### 1. Archiving Resolved Tasks
- `did archive` scans `.did/` (or a specified subtree path) for resolved tasks (files or folders starting with `.`).
- Moves completed task files into `.did/archive/` preserving their original relative folder structure.

### 2. `.hooks` Preservation Guard (CRITICAL RULE)
- **Do NOT move directories that contain `.hooks` or `hooks` subdirectories**, even if all task files inside that directory are marked done.
- Preserving directories with `.hooks/` ensures that top-down parent directory context and lifecycle hook scripts remain active for future tasks added to those subtrees.

### 3. Symlink & Link Integrity
- Any symlinks pointing to archived items must be updated to point to the new location in `.did/archive/`, or cleanly flagged if broken.

---

## Open Questions for Refinement
1. Should `did archive` move items to `.did/archive/` (visible directory) or `.did/.archive/` (hidden state folder)?
2. Should `did archive` run automatically after `did done` or remain an explicit manual subcommand?
