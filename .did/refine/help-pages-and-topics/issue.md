# Feature Proposal: `did help <TOPIC>` Extended Help Pages

## Overview
Keep default `did --help` output short, concise, and focused. Provide an extended help page system (`did help <TOPIC>`) to display detailed documentation on subcommands, concepts, and advanced features (e.g. `did help hooks`, `did help link`, `did help status`).

## Detailed Requirements

### 1. Default `did --help` / `did help`
- Keep default overview short, clean, and sweet (listing primary subcommands and flags concisely).

### 2. Topic-Specific Extended Help Pages
Users can query in-depth topic guides:
- `did help link`: Explains relative symlink creation, directory linking, and dependency propagation.
- `did help hooks`: Documents hook script naming, discovery hierarchy, and stdout/exit code rules.
- `did help status`: Explains leaf node discovery, blocking rules, and status limit environment variables.
- `did help show`: Documents parent directory top-down context formatting.

### 3. Implementation Approach
- Embedded markdown / text help topics compiled directly into the binary or loaded from built-in help pages.


## Open Questions for Refinement (@bas080)
1. @bas080 Should extended help topic Markdown files be stored in `.did/.hooks/help/` or embedded inside the compiled binary?
