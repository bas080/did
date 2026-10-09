# did - Filesystem-Native Issue Tracker with Hooks

`did` is a lightweight, filesystem-native issue tracker that uses directory structures, relative symlinks, and event-driven lifecycle hooks to build programmable, local software factories and development workflows.

All issues live in `.did/` as Markdown files, making state transparent and fully version-controlled with Git.

## Key Features

- **Filesystem State**: All tasks live in `.did/` as Markdown files.
- **Lifecycle Hooks**: Define custom hooks in `.hooks/` (e.g. `test`, `show`, `status`, `add`, `close`, `open`, `mv`, `blocks`, `rm`, `help`) to enforce rules and automate workflows.
- **Prerequisites & Dependencies**: Use `did blocks` to express dependencies between sub-tasks across directories.
- **Actionable Tracking**: `did status` identifies actionable leaf tasks whose sub-items/dependencies are resolved.

For advanced documentation, hook specifications, and environment variable references, see [ADVANCED.md](ADVANCED.md).

## Quick Start

### 1. Add Issues
```bash
# Add issue with inline content (auto-creates .did/ directory if needed)
did add refine/auth/jwt.md -m "Implement JWT token validation"

# Add issue using $EDITOR
did add refine/ui/login.md
```

### 2. Block Sub-Tasks
```bash
# Mark a task as a prerequisite for another directory
did blocks refine/auth/jwt.md refine/ui
```

### 3. Move Issues Across Workflow Stages
```bash
# Move refined issue into implement stage
did mv refine/auth/jwt.md implement/auth/jwt.md
```

### 4. Check Status
```bash
# List actionable issues
did status

# Display directory tree view
did status --tree

# List blocked tasks
did status -b
```

### 5. Inspect & Resolve Issues
```bash
# Inspect issue content
did show implement/auth/jwt.md

# Mark task closed (dot-prefixes filename)
did close implement/auth/jwt.md

# Reopen task
did open implement/auth/jwt.md
```

### 6. Validate Repository Health
```bash
# Runs sanity checks and executes .did/.hooks/test lifecycle hook
did test
```

## Installation

### Direct Binary Download (Linux x86_64)

```bash
curl -sL -o did https://github.com/bas080/did/releases/download/latest/did-linux-x86_64
chmod +x did
sudo mv did /usr/local/bin/
did --help
```

### Build from Source (Cargo)

```bash
cargo build --release
```

## Documentation

For full details on lifecycle hooks, environment variables, state cache proposals, and telemetry logging, see [ADVANCED.md](ADVANCED.md).

## License

MIT
