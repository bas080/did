# Feature Proposal: Clarifying Dependency Linking & Diagnostics (`did blocks`)

## Overview & Core Semantics
- **Linux `ln` Semantics**: `did link A B` (or `did ln A B`) works like the Linux `ln` command: it creates `A` as a sub-item inside directory `B`. Because `A` becomes a leaf sub-item inside `B`, **`B` depends on `A`** (meaning `A` is the prerequisite blocking `B`).
- **Proposed Command Renaming**:
  - Rename / alias `link` to `did blocks TASK DEPENDENT`.
  - Example: `did blocks backend/auth/jwt.md frontend/ui` explicitly declares that `jwt.md` blocks `frontend/ui`.

## Diagnostic Output Formatting Rules
1. **`did show <PATH>`**:
   - Displays explicit blocked diagnostics when `<PATH>` is blocked by unresolved sub-items or dependency symlinks.
2. **`did status`**:
   - Standard `did status` outputs only actionable leaf nodes to `stdout`.
   - When using the `-a` / `--all` or `-b` / `--blocked` flags, `did status` outputs blocked reasons and items to **`stderr`** (not `stdout`), ensuring `stdout` output remains clean and parseable.
