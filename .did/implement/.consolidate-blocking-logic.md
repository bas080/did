# Consolidate Task Blocking Logic Across Subcommands

Consolidate task blocking detection logic throughout `did` so it is defined consistently in a single central helper module/function rather than re-implemented across different commands (`status`, `show`, `close`/`done`).

## Goal
An issue/task is defined as **blocked** when it has a sibling directory containing a file or link that is still open/unresolved.

## Requirements
- Centralize `has_unresolved_subitems` and `get_unresolved_blocking_items` into a single module or implementation method on `Repo`.
- Ensure `did status`, `did status -b`, `did status -t`, `did show`, and `did close` evaluate blocking state using identical logic.
- A task file is blocked if and only if a sibling directory with the same stem (or child directory under `index.md`) contains any open file or unresolved symlink.

## Acceptance Criteria
- [ ] Central helper function handles all blocking state checks across all subcommands.
- [ ] Unit and integration tests verify blocking behavior for files, directories, and symlinks.
