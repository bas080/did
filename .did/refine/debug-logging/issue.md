# Feature Specification: `DID_DEBUG` Verbose Debug Logging

## Overview
Adds internal debug logging throughout `did`. When the `DID_DEBUG` environment variable is defined and non-empty (e.g. `DID_DEBUG=1` or `DID_DEBUG=true`), `did` outputs verbose diagnostic logs directly to `stderr` during command execution. This assists developers when debugging path mapping, hook discovery, symlink resolution, and AST blocking rules.

---

## Detailed Requirements

### 1. Environment Variable Activation
- Debug logging is **DISABLED BY DEFAULT**.
- Debug logging is activated if `DID_DEBUG` is defined and non-empty in the environment.

### 2. Log Output & Formatting
- Debug messages are printed exclusively to `stderr` so as not to pollute `stdout` outputs.
- Debug message format:
  ```
  [DEBUG] Repo path resolved: raw='backend/auth/jwt.md' -> target='/workspace/.did/backend/auth/jwt.md'
  [DEBUG] Hook check in '/workspace/.did/backend': found '.hooks/show'
  [DEBUG] Blocking check for 'backend/auth.md': 1 unresolved child sub-items found
  ```

### 3. Key Debug Logging Operations
- **Path Resolution**: Log raw input paths and resolved `.did/` target paths.
- **Hook Discovery**: Log ancestor directory traversal and matched `.hooks/` scripts.
- **Blocking Traversal**: Log child directory traversal and detected blocking sub-items.
- **Symlink Updates**: Log symlinks scanned and updated during `did done` or `did mv`.
