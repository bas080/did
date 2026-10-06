# Issue: Restrict 'did link' to Directories Only

## Proposal
Only allow linking a directory in `did link` (e.g., `did link backend/auth frontend`) on the basis that directories are never renamed/hidden with a leading dot upon completion, thereby eliminating the need to search and rename symlinks when a task is completed (`did done`).

## Analysis of Potential Unforeseen Issues & Trade-Offs

1. **Loss of Fine-Grained (Task-to-Task) Dependencies**:
   - Restricting links to directories prevents linking a single specific task file as a dependency.
   - If task `A` only depends on task `B` (and not tasks `C` or `D` in the same folder), linking the containing directory forces `A` to depend on *all* tasks in that directory.

2. **Directory Structure Inflation**:
   - If users want to link an individual issue/task, they would be forced to create a wrapper directory for every task (e.g., `.did/tasks/jwt/jwt.md` instead of `.did/tasks/jwt.md`) just so the directory can be linked. This creates unnecessary folder nesting.

3. **Inability to Alias / Share Individual Tasks Across Modules**:
   - Tracking a single cross-cutting bug or task in multiple module directories (`backend/`, `frontend/`, `docs/`) requires wrapper directories for each shared issue.

4. **Reduced Git Status Visibility**:
   - When linking task files directly, completing a task (`did done`) renames both the file and its symlinks (`.jwt.md`), making task completion explicitly visible in `git status` and `git diff`. Symlinks to directories remain unchanged regardless of whether sub-tasks are completed.

## Recommendation
Consider keeping file-level symlink support (with automatic symlink renaming on `did done`), or support both file and directory linking to preserve fine-grained dependency modeling.
