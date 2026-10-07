# Feature Proposal: Automatic State Repair & Diagnostic Fixes (`did test --fix`)

## Overview
Explores automatic state repair mechanisms during `did test` or command execution to automatically repair minor state anomalies without requiring a separate `clean` command.

## Proposed Features
1. **`did test --fix` Flag**: Automatically repairs broken symlinks, sets missing `chmod +x` permissions on shebang hook scripts, or cleans dangling references.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `did test --fix` automatically remove broken symlinks and repair script permissions, or prompt interactively before making changes?
2. @bas080 Should state repair operations generate backup snapshots in `.did/.bak/` before modifying state?
