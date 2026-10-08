# Introduce 'start' and 'stop' State Transitions for Active Work Tracking @bas080

## Overview
Currently, tasks in `did` exist in a binary state of either 'Open' (distributed across `refine/`, `implement/`, `backlog/`) or 'Resolved' (hidden dot-files). There is no explicit mechanism to mark a task as "Currently In Progress" (WIP). 

Introducing `did start <path>` and `did stop <path>` would allow developers to explicitly track their active focus, enabling better visibility into current work-in-progress and preventing the 'fragmentation' of attention across too many open tasks.

## Proposed Suggestions for Implementation

### Option A: The 'Active' Directory (State-based)
- **Mechanism**: `did start` moves the task file (or a symlink to it) into a new `.did/active/` directory. `did stop` moves it back to its original location (e.g., `.did/implement/`).
- **Pros**: Very clear filesystem separation.
- **Cons**: Moving files can break relative symlinks unless handled carefully.

### Option B: The 'Focus' Symlink (Single-task Focus)
- **Mechanism**: `did start` creates a symlink at `.did/focus` pointing to the target task. `did stop` removes the symlink.
- **Pros**: Encourages single-tasking. Extremely lightweight.
- **Cons**: Only supports one active task at a time.

### Option C: State Prefix/Metadata (Naming-based)
- **Mechanism**: `did start` renames the file to include a prefix (e.g., `_active_task.md`). `did stop` removes the prefix.
- **Pros**: No directory moves.
- **Cons**: Changes the path, which may affect other tools or hooks.

## Requirements
- [ ] Define a consistent mechanism for marking a task as 'Started'.
- [ ] Implement `did start <path>` to initiate the WIP state.
- [ ] Implement `did stop <path>` to exit the WIP state.
- [ ] **Status Integration**: Enhance `did status` to visually prioritize and highlight started tasks (e.g., using a special icon or color).
- [ ] **Validation**: Ensure that starting a task already marked as 'Done' is prohibited.

## Acceptance Criteria
- [ ] Running `did start <path>` correctly marks the task as active.
- [ ] Running `did stop <path>` returns the task to its previous open state.
- [ ] `did status` clearly distinguishes between 'Open' and 'Started' tasks.
- [ ] The mechanism does not break existing ancestor hook resolution.

## Test Plan
- [ ] Start a task from `implement/` and verify it is marked as active.
- [ ] Run `did status` and verify the active task is highlighted.
- [ ] Stop the task and verify it returns to `implement/`.
- [ ] Attempt to start a resolved (dot) file and verify it fails.
