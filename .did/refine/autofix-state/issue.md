# Feature Proposal: Automatic State Repair & Diagnostic Fixes (`did test --fix`)

## Overview
Explores automatic state repair mechanisms during `did test` or command execution to automatically repair minor state anomalies without requiring a separate `clean` command.

## Proposed Features
1. **`did test --fix` Flag**: Automatically repairs broken symlinks, sets missing `chmod +x` permissions on shebang hook scripts, or cleans dangling references.
