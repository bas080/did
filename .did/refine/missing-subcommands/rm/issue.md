# Sub-Issue: `did rm <PATH>`

## Overview
Subcommand for safely removing task files or directories from `.did/`.

---

## Detailed Requirements

1. **Task File Deletion**:
   - Deletes specified task file or task folder inside `.did/`.
2. **Symlink Dependency Warning**:
   - Scans `.did/` for any active symlinks pointing to `<PATH>`.
   - Prompts or warns the user if removing `<PATH>` will break active dependency symlinks.
