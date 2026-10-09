# External State Cache for Out-of-Tool Changes

Propose an external state cache outside the repository (e.g., in user home directory `~/.cache/did/` or `~/.config/did/`) to track file state modifications made outside `did` CLI commands.

## Goal
When a user uses an external editor, git command, or file manager outside `did` to resolve or reopen tasks, `did` should detect state changes and invoke `close` or `open` lifecycle hooks automatically before executing `did test` or status checks.

## Requirements
- Maintain an external state cache in the user's home directory (e.g. `~/.cache/did/<repo-hash>/state.json`).
- Track file hashes/timestamps of `.did/` tasks.
- On `did test` execution, compare current state with cache and execute `close` or `open` lifecycle hooks for files whose status changed outside `did`.

## Questions
- [ ] @bas080: Should the state cache store file hashes or relative path state listings, and what directory under `$HOME` is preferred (`~/.cache/did/` or `~/.config/did/`)?
