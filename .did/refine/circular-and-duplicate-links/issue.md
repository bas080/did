# Research: Circular Symlink Cycle Prevention & Link Conciseness

## Problem Statement
1. **Repetitive Links**: When a dependency (e.g. `test-workflow.md`) applies to multiple subdirectories (`release-linux`, `release-windows`, `release-macos`, etc.), calling `did link` individually for each subdirectory creates duplicate symlinks pointing to the same target (`../test-workflow.md`).
2. **Circular Dependencies**: If task/directory `A` links to `B` and `B` links to `A` (directly or transitively), a cycle is created in the dependency graph. Without cycle detection, directory traversals (`WalkDir`, `has_unresolved_subitems`) can enter infinite loops or infinite recursion.

## Research Findings & Prevention Strategies

### 1. Cycle Detection During `did link`
Before `did link TARGET DEST` creates a symlink on disk:
- Perform a Directed Acyclic Graph (DAG) reachability check starting from `TARGET` following existing symlinks.
- If `DEST` (or any parent directory of `DEST`) is reachable from `TARGET`, creating the symlink `.did/DEST/TARGET_NAME -> TARGET` would complete a cycle (`DEST -> TARGET -> ... -> DEST`).
- **Action**: Reject `did link` with an error:
  ```text
  error: cannot link 'TARGET' into 'DEST': creates a circular dependency cycle
  ```

### 2. Cycle-Safe Directory Traversal
In `has_unresolved_subitems` and tree walkers:
- Maintain a `HashSet<PathBuf>` tracking canonical paths visited along the current traversal path.
- If a symlink points to a path already present in the traversal stack, stop following that path and log a warning to prevent infinite loops.

### 3. More Concise Link Structures
Instead of creating 5 repetitive symlinks across sister folders:
- **Parent-Level Linking**: Link `test-workflow.md` at the parent folder level (`github-actions/`) if all sub-tasks in `github-actions/` depend on `test-workflow.md`.
- **Batch / Pattern Linking**: Support syntax or pattern expansion in `did link` (e.g. `did link TARGET DEST1 DEST2 DEST3`).
