# did - Filesystem-Native Issue Tracker with Hooks

`did` is a lightweight, filesystem-native issue tracker that uses directory structures, relative symlinks, and event-driven lifecycle hooks to build programmable, local software factories and development workflows.

## Goal

The primary goal of `did` is to provide a local, git-versioned issue tracking system where directory structures define workflows (e.g., `.did/refine/`, `.did/implement/`) and executable lifecycle hooks (`.hooks/`) automate quality gates, guidelines, and execution pipelines directly inside the repository.

By treating issues as files and directory paths as states, `did` allows developers and autonomous AI agents to build self-testing, self-guided local software factories.

## Key Features

- **Filesystem State**: All issues and tasks live in `.did/` as Markdown or text files, making state transparent and fully version-controlled with Git.
- **Lifecycle Hooks**: Define custom hooks in `.hooks/` (such as `test`, `show`, `status`, `add`, `done`, `mv`, `link`, `rm`, `help`) to enforce rules, trigger checks, or display contextual guidance.
- **Software Factory Automation**: Intercept CLI operations with executable hooks to automate stage transitions, enforce quality constraints, or collect telemetry.
- **Dependency Symlinks**: Use symlinks (`did link`) to express prerequisite dependencies between sub-tasks across directories.
- **Actionable Tracking**: `did status` identifies actionable leaf tasks whose sub-items/dependencies are resolved.
- **Execution Telemetry**: Structured XML logging (`DID_LOG_PATH`) for tracking command invocations across automated pipeline runs.

## Software Factories & Lifecycle Hooks

In `did`, any directory in `.did/` can contain a `.hooks/` directory with hook scripts named after CLI subcommands (`add`, `show`, `status`, `done`, `test`, `link`, `mv`, `rm`, `help`).

When a `did` subcommand executes:
1. **Hook Discovery**: `did` traverses from the target path up to `.did/` looking for matching `.hooks/<cmd>` files.
2. **Hook Execution**: If executable, hooks run before the subcommand completes, receiving context via environment variables (`DID_EVENT`, `DID_TARGET`, `DID_DEST`, `DID_OLD`, `DID_NEW`, `DID_REPO_ROOT`, `DID_STATE_DIR`).
3. **Quality Gates**: If a hook exits with a non-zero exit code, `did` aborts the operation.

This allows defining local factory workflows—for example, requiring issue refinement tagging before moving issues to `implement/`, or enforcing unit tests and state sanity checks when running `did test`.

## Installation

### Direct Binary Download (Linux x86_64)

To install the **latest release**:
```bash
curl -sL -o did https://github.com/bas080/did/releases/download/latest/did-linux-x86_64
chmod +x did
sudo mv did /usr/local/bin/
did --help
```

To install the **latest development build**:
```bash
curl -sL -o did https://github.com/bas080/did/releases/download/development/did-linux-x86_64
chmod +x did
sudo mv did /usr/local/bin/
did --help
```

### Build from Source (Cargo)

Ensure you have Rust installed (1.80+), then build with Cargo:

```bash
cargo build --release
```

The compiled binary will be placed at `target/release/did`.

## Quick Start

### 1. Add Issues
```bash
# Add issue with inline content (auto-creates .did/ directory if needed)
did add refine/auth/jwt.md -m "Implement JWT token validation"

# Add issue using $EDITOR
did add refine/ui/login.md
```

### 2. Link Dependencies
```bash
# Link a task into another directory as a dependency (prerequisite)
did link refine/auth/jwt.md refine/ui
```

### 3. Move Issues Across Workflow Stages
```bash
# Move refined issue into implement stage
did mv refine/auth/jwt.md implement/auth/jwt.md
```

### 4. Check Status
```bash
# List actionable issues (unblocked leaf tasks)
did status

# List all issues including blocked and completed tasks
did status -a
```

### 5. Validate Repository Health
```bash
# Runs sanity checks and executes .did/.hooks/test lifecycle hook
did test
```

### 6. Inspect & Resolve Issues
```bash
# Inspect issue content
did show implement/auth/jwt.md

# Mark task resolved (dot-prefixes filename and updates symlinks)
did done implement/auth/jwt.md

# Reopen task
did undone implement/auth/jwt.md
```

## Environment Variables

| Variable | Description |
| :--- | :--- |
| `DID_STATUS_LIMIT` / `DID_LIMIT` | Sets the maximum number of items returned by `did status` or `did query` before displaying a truncation notice on `stderr`. |
| `DID_LOG_PATH` | Activates XML execution telemetry logging. Relative paths are resolved relative to the parent directory where `.did/` lives. |
| `DID_DEBUG` | Enables verbose diagnostic logging on `stderr`. |
| `EDITOR` | Specifies the text editor to invoke when running `did add PATH` without a `-m` message flag (defaults to `vi`). |

## CLI Reference

```
SYNOPSIS
       did [FLAGS] [COMMAND] [ARGS...]

COMMANDS
       add            Create a task node or nested issue at PATH
       status         List actionables (leaf nodes with all sub-items done)
       query          Search task paths and file contents for QUERY (alias: search)
       show           Print task contents at PATH (requires sub-items done unless -a)
       done           Mark PATH as resolved (hides it; fails if sub-items remain open)
       undone         Mark a resolved PATH as open/undone (removes leading dot)
       link           Symlink TARGET into DEST directory as a dependency (alias: ln)
       mv             Move or rename a task file or directory at OLD_PATH to NEW_PATH (alias: move)
       rm             Remove a task file or directory at PATH (alias: remove)
       test           Validate repository health and run test lifecycle hook
       autocomplete   Generate shell completion scripts (e.g. bash)

FLAGS
       -h, --help     Print help information
       -V, --version  Print version information
       -a, --all      Include resolved (hidden) tasks and blocked nodes
```

## License

MIT
