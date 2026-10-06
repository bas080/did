# Agent Instructions & Workflow

Welcome! This codebase uses `did`, a filesystem-native task and dependency tracker.

## `did` Task Tracking Workflow

When working on this repository, follow these guidelines:

1. **Check Actionable Tasks**:
   - Run `did status` to list current actionable tasks.
   - Run `did status -a` to view all tasks (including blocked and completed tasks).

2. **Creating Issues for Found Problems**:
   - **Whenever you discover an issue, bug, or needed feature while working in the repository, you MUST create a `did` issue for it.**
   - Example:
     ```bash
     did add issues/fix-symlink-resolution.md -m "Describe the issue clearly here"
     ```

3. **Inspecting Task Content**:
   - View task details with `did show <PATH>` (e.g., `did show issues/fix-symlink-resolution.md`).

4. **Linking Dependencies**:
   - Link dependent tasks/issues into directories using `did link <TARGET> <DEST_DIR>`.

5. **Completing Tasks**:
   - When a task or issue is resolved, mark it complete:
     ```bash
     did done <PATH>
     ```
   - This renames the file with a leading dot (`.`) and updates linked dependencies.
