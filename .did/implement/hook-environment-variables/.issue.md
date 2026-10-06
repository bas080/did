# Feature Specification: Standard Environment Variables for Lifecycle Hooks

## Overview
Proposes exporting standard `DID_*` environment variables to lifecycle hook scripts (`.hooks/show`, `.hooks/done`, `.hooks/mv`, `.hooks/add`, `.hooks/status`, `.hooks/link`).

---

## Detailed Requirements

When executing any hook script, `did` sets the following environment variables:
- `DID_EVENT`: Event name (`show`, `done`, `mv`, `add`, `status`, `link`).
- `DID_TARGET`: Target task path relative to `.did/` (e.g. `backend/auth/jwt.md`).
- `DID_REPO_ROOT`: Absolute path to the project root directory containing `.did/`.
- `DID_STATE_DIR`: Absolute path to the `.did/` directory.
