# Implement did init feature

## Overview
Add a `did init` subcommand to explicitly initialize the `.did/` state directory in the current working directory.

## Motivation
While `did add` auto-initializes `.did/` on first use, an explicit `init` command is useful for:
- **Bootstrapping**: Set up the tracker before creating any tasks
- **CI/CD**: Initialize `.did/` in fresh environments (containers, CI runners) without adding a task
- **Template projects**: Ship a repo with `.did/` structure pre-initialized
- **Explicit intent**: Users can initialize without remembering `did add` semantics
- **Hook execution**: Run lifecycle hooks (e.g., `.did/.hooks/init`) on first setup

## Proposed Behavior
```
did init [PATH]
```

- If no `PATH` given, uses current working directory
- Creates `.did/` with standard structure:
  ```
  .did/
  ├── refine/
  ├── implement/
  ├── backlog/
  ├── wont/
  └── .hooks/
      ├── add
      ├── close
      ├── mv
      ├── rm
      └── test
  ```
- If `.did/` already exists, exits 0 (idempotent) — optionally `--force` to reinitialize hooks
- Runs `.did/.hooks/init` if present (receives target path as `$1`)

## Acceptance Criteria
- [ ] `did init` creates `.did/` with all standard directories
- [ ] `did init` is idempotent (running twice succeeds)
- [ ] `did init /some/path` initializes at that path
- [ ] `did init --force` re-runs hook scripts
- [ ] `did add` still works without prior `did init` (backward compatible)
- [ ] `.did/.hooks/init` is invoked with target directory as argument
- [ ] `did test` passes

## Test Plan
1. **Fresh init**: `cd /tmp/test && did init && ls -la .did/`
2. **Idempotent**: `did init && did init && echo $?`
3. **Custom path**: `did init /tmp/other && test -d /tmp/other/.did`
4. **Hook execution**: Create `.did/.hooks/init` that touches a file, run `did init`, verify file created
5. **Force flag**: `did init --force` re-runs hook
6. **Backward compat**: `cd /tmp/fresh && did add "task" && test -d .did`
7. **Run full test suite**: `did test`

## Related
- Rejected proposal: `.did/wont/did-init/issue.md` (argued redundant; this issue re-evaluates for explicit init use cases)