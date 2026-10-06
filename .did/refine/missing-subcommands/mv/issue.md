# Sub-Issue: `did mv`, `did promote`, and `did demote`

## Overview
Subcommands for relocating task files and directories, promoting issues to implementation (`.did/implement/`), and demoting vague issues back to refinement (`.did/refine/`).

---

## Detailed Requirements

1. **`did mv <OLD_PATH> <NEW_PATH>`**:
   - Relocates files or directories inside `.did/`.
   - Recomputes and updates all inbound and outbound relative symlink targets across `.did/`.
2. **`did promote <PATH>`**:
   - Shortcut for `did mv refine/<PATH> implement/<PATH>`.
3. **`did demote <PATH>`**:
   - Shortcut for `did mv implement/<PATH> refine/<PATH>`.
