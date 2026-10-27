# Pure Note System with Hook-Driven Workflows and Subcommand Aliases

Generalize `did` into a pure, filesystem-native note system where all `.md` files in `.did/` are notes, subdirectories represent subject hierarchies, and workflow rules/blocking semantics are delegated to `.hooks/`.

## Key Architectural Principles

1. **Pure & Simple Notes**:
   - Every file in `.did/` is a Note.
   - Directories represent subject/topic hierarchies rather than automatic blocking parent tasks.
   - `did status`, `did query`, and `did fzf` list all open notes directly without hiding items behind automatic blocking checks.

2. **`did link` Subcommand**:
   - `link` is the primary subcommand for creating relative symlinks between notes (with `blocks` and `ln` preserved as aliases).

3. **Hook-Driven Workflows**:
   - Complex workflows, task gates, and blocking checks are handled via `.hooks/` scripts (`.hooks/status`, `.hooks/close`, `.hooks/show`, etc.) rather than hardcoded CLI logic.

4. **Normalized "Note" Terminology**:
   - CLI descriptions, help pages, default filenames (`note.md`), error messages, and git log actions use "note" everywhere.

## Before & After CLI Examples

### `did --help`
```
# Before
filesystem-native issue tracker with hooks for defining local software factories

# After
filesystem-native note and workflow tracker with hooks for defining local software factories
```

### `did add --help`
```
# Before
Create a task issue

# After
Create a note
```

## Requirements
- Make `link` the primary subcommand in `src/cli.rs` and `src/commands.rs`, with `blocks` and `ln` as aliases.
- Update CLI help texts, subcommand descriptions, fallback filenames (`note.md`), error messages, and log output verbs (`Note created`, `Note closed`, etc.) to use "note" everywhere.
- Remove hardcoded directory blocking from `cmd_status`, `cmd_show`, `cmd_done`, `cmd_undone`, `cmd_fzf`, and `cmd_search` so all open notes are visible and actionable by default.
- Delegation of prerequisite blocking checks to `.hooks/` (e.g. `.hooks/close`, `.hooks/show`).
- Update `README.md` and `ADVANCED.md`.
- Ensure all tests pass.
