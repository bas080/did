# Feature Specification: `did rm`, `did cat`, and `did clean` Subcommands

## Overview
Identifies CLI operational gaps where developers or AI agents currently resort to non-`did` bash commands (`rm`, `cat`, `find`) to interact with `.did/`.

## Proposed Subcommands
1. **`did rm <PATH>`**: Safely removes task files or directories from `.did/` while updating or warning about active inbound symlink dependencies.
2. **`did cat <PATH>`**: Prints raw task file content without executing `show` hooks or parent context formatting.
3. **`did clean`**: Detects and purges dangling/broken symlinks across `.did/`.
