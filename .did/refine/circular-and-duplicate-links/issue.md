# Research: Dependency Linking Patterns & Symlink Blocking Semantics

## Problem & Context
When expressing dependencies across task nodes (e.g. `test-workflow.md` blocking all `github-actions/release-*` tasks):
1. How can dependencies be defined cleanly without creating repetitive nested symlink folders in every sub-task?
2. How should symlink dependencies behave relative to regular sibling files in the same directory?

---

## 1. Defining Subtree Prerequisites via Subdirectories

To prove how a prerequisite task (like `test-workflow`) blocks other tasks in a folder without needing to nest those other tasks:

### Structure:
```text
.did/github-actions/
  ├── test-workflow/
  │     └── task.md         (Actionable Leaf Prerequisite)
  ├── release-linux.md      (Blocked by child directory test-workflow/)
  ├── release-windows.md    (Blocked by child directory test-workflow/)
  └── release-macos.md      (Blocked by child directory test-workflow/)
```

### How It Works:
- `test-workflow/task.md` is in a child subdirectory `test-workflow/`.
- `release-linux.md`, `release-windows.md`, and `release-macos.md` are in `github-actions/`.
- Because `github-actions/` contains a child subdirectory `test-workflow/` with an unresolved file `task.md`, all tasks directly in `github-actions/` (`release-linux.md`, `release-windows.md`, etc.) are blocked by `test-workflow/task.md`.
- `release-*` files do **not** need to be nested inside subdirectories—they remain flat inside `github-actions/`.
- Once `test-workflow/task.md` is completed (`did done test-workflow/task.md`), `test-workflow/` has no unresolved files. `release-linux.md`, `release-windows.md`, etc. immediately become actionable!

---

## 2. Alternative Rule: Symlinks Represent Explicit Folder-Level Blockers

An alternative design rule evaluates symlinks differently from regular task files:

### Behavior Rules:
1. **Regular Sibling Files**: `a.md` and `b.md` in the same folder are independent actionable leaf nodes. They do **not** block each other.
2. **Symlink Dependencies**: **All open symlinks in a directory represent explicit blocking dependencies for all task files in that directory.**
   - If directory `auth/` contains task `jwt.md` and symlink `dep.md -> ../specs/auth_spec.md`:
     - `dep.md` points to an unresolved target (`auth_spec.md`).
     - Therefore, `jwt.md` in `auth/` is **blocked** by `dep.md`.
     - `did status auth/` outputs `dep.md` (or `specs/auth_spec.md`) as the actionable task.
   - Once `auth_spec.md` is completed (`did done specs/auth_spec.md`), `dep.md` is updated to `.dep.md` (resolved).
   - Now `auth/` has no open symlinks, so `jwt.md` becomes **actionable**!

### Advantages:
- **Concise Dependency Specification**: Linking a single target into a folder (`did link target folder`) cleanly blocks all tasks in `folder` until `target` is done.
- **No Extra Folder Nesting**: Users do not need to create extra subdirectories just to attach a symlink dependency.
- **No Circular Symlink Cascades**: Symlinks act as terminal leaf dependencies for their containing directory.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `did link` reject duplicate links and return an explicit non-zero exit code or exit silently with status 0?
2. @bas080 Should cycle detection be performed recursively across all symlink chains during `did link`?
