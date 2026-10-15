# did - Filesystem-Native Note & Workflow Tracker with Hooks

`did` is a lightweight, filesystem-native note and workflow tracker that uses directory structures, relative symlinks, and event-driven lifecycle hooks to build programmable, local software factories, note archives, and development workflows.

All notes live in `.did/` as Markdown files, making state transparent and fully version-controlled with Git.

## Key Features

- **Filesystem State**: All notes live in `.did/` as Markdown files.
- **Lifecycle Hooks**: Define custom hooks in `.hooks/` (e.g. `test`, `show`, `status`, `add`, `close`, `open`, `mv`, `link`, `rm`, `help`) to enforce rules and automate workflows.
- **Note Linking**: Use `did link` to express relative symlink relationships between notes across directories.
- **Subject Hierarchies & Status**: `did status` lists open notes across topic and subject directories.

For advanced documentation, hook specifications, and environment variable references, see [ADVANCED.md](ADVANCED.md).

## Quick Start

### 1. Add Notes
```bash
# Add a note with a title (auto-creates .did/ directory if needed)
did add "Implement JWT token validation"

# Add a note with piped body content
echo "Detailed requirements..." | did add "Implement JWT token validation"

# Pipe directly into did without a subcommand to create a note
echo -e "Fix login bug\n\nSteps to reproduce..." | did
```

### 2. Link Notes
```bash
# Create a relative symlink from a note into another directory
did link refine/auth/jwt.md refine/ui
```

### 3. Move Notes Across Workflow Stages
```bash
# Move refined note into implement stage
did mv refine/auth/jwt.md implement/auth/jwt.md
```

### 4. Check Status
```bash
# List open notes
did status

# Display directory tree view
did status --tree
```

### 5. Inspect & Resolve Notes
```bash
# Inspect note content
did show implement/auth/jwt.md

# Mark note resolved (dot-prefixes filename)
did close implement/auth/jwt.md

# Reopen note
did open implement/auth/jwt.md
```

### 6. Validate Repository Health
```bash
# Runs sanity checks and executes .did/.hooks/test lifecycle hook
did test
```

### 7. Interactive Fuzzy Finder
```bash
# Browse notes interactively with note preview (requires fzf)
did fzf
```

## Installation

### Direct Binary Download (Linux x86_64)

```bash
curl -sL -o did https://github.com/bas080/did/releases/download/latest/did-linux-x86_64
chmod +x did
sudo mv did /usr/local/bin/
did --help
```

### Direct Binary Download (Windows x86_64)

In PowerShell:
```powershell
Invoke-WebRequest -Uri "https://github.com/bas080/did/releases/download/latest/did-windows-x86_64.exe" -OutFile "$env:USERPROFILE\did.exe"
# Add to PATH if not already present
$env:PATH += ";$env:USERPROFILE"
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
