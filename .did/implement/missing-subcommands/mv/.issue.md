# Sub-Issue: `did mv <OLD_PATH> <NEW_PATH>`

## Overview
Subcommand for relocating task files and directories inside `.did/`.

---

## Detailed Requirements

1. **`did mv <OLD_PATH> <NEW_PATH>`**:
   - Relocates files or directories inside `.did/`.
   - Recomputes and updates all inbound and outbound relative symlink targets across `.did/`.
   - Can be used by users and AI agents to move issues between custom user workflow folders (e.g. `did mv refine/task.md implement/task.md`).
