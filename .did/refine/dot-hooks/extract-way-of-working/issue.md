# Feature Proposal: Extract Way-of-Working (`.hooks/` Directory Backup)

## Overview
Adds a subcommand or export option (e.g. `did export-hooks <DEST_DIR>`) that extracts and backs up all `.hooks/` and `hooks/` directories across `.did/` into a backup location `<DEST_DIR>`.

## Rationale
Since `did` stores project way-of-working instructions and lifecycle automation scripts inside `.hooks/` directories, backing up or extracting `.hooks/` allows projects to easily share, template, or restore established way-of-working practices across repositories.

## Proposed Command Syntax
```bash
did export-hooks <DEST_DIR>
```

## Requirements
1. Scans `.did/` for all `.hooks` and `hooks` directories.
2. Copies hook files and directory hierarchy into `<DEST_DIR>`, preserving permissions (`chmod +x`).


## Open Questions for Refinement (@bas080)
1. @bas080 Should hook configuration export/import be managed via `did export-hooks` or git submodules?
