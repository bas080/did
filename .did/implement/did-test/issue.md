# Feature Specification: `did test` Validation Command

## Overview
Adds a `did test` subcommand that scans the `.did/` repository to verify repository health, consistency, and adherence to tracker conventions. `did test` outputs a diagnostic report to stdout/stderr and exits with a non-zero exit code if any violations are detected.

---

## Checks Performed by `did test`

### 1. Link & Dependency Integrity
- Verifies that all symlinks in `.did/` point to valid targets (no broken symlinks).
- Checks that relative symlinks correctly resolve within `.did/`.
- Verifies that resolved (dot-prefixed) task files do not leave behind active non-dot symlinks.

### 2. State & Marking Done Rules
- Ensures tasks marked done (dot-prefixed files) do not have open (non-dot) sub-items under child directories.

### 3. Executable Shebang Verification
- Detects non-executable files that start with a shebang line (e.g. `#!/bin/sh` or `#!/usr/bin/env bash`).
- Reports these as potential script permission errors (missing `chmod +x` or `0o111` execution bits).

### 4. Hook Directory Sanctity (`.hooks/`)
- Checks that `.hooks/` directories strictly contain valid hook files named after reserved hook events (`show`, `status`, `done`, `add`, `link`, `mv`).
- Flags any non-hook files or subdirectories placed inside `.hooks/` as invalid state structure.

---

## Output & Exit Codes
- **Output**: Prints a human-readable diagnostic summary report.
- **Exit Status**:
  - `0`: All rules satisfied; repository is clean.
  - Non-zero (`1`): Issues or rule violations detected.
