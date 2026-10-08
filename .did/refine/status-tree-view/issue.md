# Feature Proposal: Tree-Structured Output for `did status`

## Overview
Currently, `did status` lists actionable items. As the project grows, a flat list may become difficult to parse. This proposal suggests adding a tree-structured view (similar to the `tree` command) to visualize the hierarchy of tasks and their completion status.

## Proposed Solution
- Add a flag (e.g. `-t` / `--tree`) to `did status`.
- When enabled, render the output as a directory tree, marking completed items and highlighting open leaf nodes.
- **Filtering Logic**: The tree view respects the flags passed to `did status`:
  - **Default**: Pruned tree; only shows branches leading to unblocked and unfinished issues.
  - **With `-a` / `--all`**: Shows the full hierarchy, including completed and blocked tasks.
  - **With blocked flags**: Includes blocked nodes in the tree.
- **Status Indicators**:
  - `[x]` for completed tasks
  - `[ ]` for open/actionable tasks
  - `[!]` for blocked tasks

This ensures the tree remains focused by default while allowing for full state inspection when requested.

## Open Questions for Refinement (@bas080)
1. @bas080 Should status tree view use ASCII characters (|- `--`) or Unicode box drawing characters (├── └──) by default?
