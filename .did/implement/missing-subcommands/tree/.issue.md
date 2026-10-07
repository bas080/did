# Sub-Issue: `did tree [PATH]`

## Overview
Subcommand for rendering a visual directory tree showing tasks, sub-items, and dependency links.

---

## Detailed Requirements

1. **Tree Hierarchy Output**:
   - Renders task directories and files in an ASCII tree format.
2. **Depth Limiting (`-L <DEPTH>`)**:
   - Accepts `-L` / `--level` to limit tree rendering depth.
3. **Task Status Color-Coding**:
   - Color-codes actionable leaf tasks, blocked nodes, and resolved (dot-prefixed) tasks.
