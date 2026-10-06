# Rejected Proposal: `did init` Subcommand

## Overview
Proposes a dedicated `did init` subcommand to initialize the `.did/` state directory in the current working directory.

---

## Decision: WON'T IMPLEMENT (`.did/wont/`)

### Rationale
- **Automatic Initialization**: `did add <PATH>` automatically initializes and creates the `.did/` state directory when adding the first task if `.did/` does not already exist.
- **Redundant CLI Surface**: Having a separate `did init` command is redundant since `did add` transparently handles state initialization.
