# Feature Specification: `did diff` Git Integration

## Overview
Proposes `did diff [COMMIT/BRANCH]` to compare the current `.did/` state against a Git commit, branch, or tag (defaulting to `HEAD`).

---

## Detailed Requirements

1. **Changed Tasks Detection**:
   - Queries `git diff` for changes inside `.did/`.
   - Categorizes tasks as:
     - **Added**: Tasks created in current workspace.
     - **Resolved**: Tasks marked done in current workspace.
     - **Modified**: Tasks whose content was modified.
2. **Output Format**:
   ```
   [ADDED] backend/auth/jwt.md
   [RESOLVED] frontend/login.md (.login.md)
   ```


## Open Questions for Refinement (@bas080)
1. @bas080 Should `did diff` compare `.did` against `HEAD` or against a specified git ref/branch?
