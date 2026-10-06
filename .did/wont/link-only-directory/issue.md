# Issue: Restrict 'did link' to Directories Only

## Proposal
Only allow linking a directory in `did link` (e.g., `did link backend/auth frontend`) on the basis that directories are never renamed/hidden with a leading dot upon completion, thereby eliminating the need to search and rename symlinks when a task is completed (`did done`).

## Analysis of Potential Unforeseen Issues & Trade-Offs

### 1. Risk of Symlink Cycles & Traversal Infinite Loops
- When directory symlinks are created (e.g. `did link b a` and `did link a b`), recursive directory walkers (such as `WalkDir` or status logic) can encounter infinite directory symlink cycles if symlink traversal follows directories without strict loop detection.
- File symlinks, by contrast, are terminal leaf nodes and cannot introduce cyclic directory containment.

### 2. Accidental Mass Dependency Blocking
- Linking an entire directory `backend/` into `frontend/` means that tasks in `frontend/` become blocked until **every single task** in `backend/` is marked done (even unrelated tasks like documentation or database migrations in `backend/`).
- This leads to artificial workflow bottlenecks where tasks cannot be worked on because an unrelated task in the linked directory remains open.

### 3. Loss of Fine-Grained (Task-to-Task) Dependencies
- Restricting links to directories prevents linking a single specific task file as a dependency.
- If task `A` only depends on task `B` (and not tasks `C` or `D` in the same folder), linking the containing directory forces `A` to depend on *all* tasks in that directory.

### 4. Directory Structure Inflation
- If users want to link an individual issue/task, they would be forced to create a wrapper directory for every task (e.g., `.did/tasks/jwt/jwt.md` instead of `.did/tasks/jwt.md`) just so the directory can be linked. This creates unnecessary folder nesting.

### 5. Inability to Alias / Share Individual Tasks Across Modules
- Tracking a single cross-cutting bug or task in multiple module directories (`backend/`, `frontend/`, `docs/`) requires wrapper directories for each shared issue.

### 6. Reduced Git Status Visibility
- When linking task files directly, completing a task (`did done`) renames both the file and its symlinks (`.jwt.md`), making task completion explicitly visible in `git status` and `git diff`. Symlinks to directories remain unchanged regardless of whether sub-tasks are completed.

## Recommendation
Consider keeping file-level symlink support (with automatic symlink renaming on `did done`), or support both file and directory linking to preserve fine-grained dependency modeling.
