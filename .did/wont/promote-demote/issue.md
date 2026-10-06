# Rejected Proposal: `did promote` & `did demote` Subcommands

## Overview
Proposes dedicated `did promote <PATH>` and `did demote <PATH>` subcommands to transition tasks between `.did/refine/` and `.did/implement/`.

---

## Decision: WON'T IMPLEMENT (`.did/wont/`)

### Rationale
- **User-Defined Workflow Hierarchy**: Folder concepts like `refine/` and `implement/` are team-specific ways of working, not hardcoded tracker concepts.
- **CLI Flexibility**: `did mv <OLD_PATH> <NEW_PATH>` provides a single, general-purpose command for relocating files and directories across any folder structure without baking rigid workflow assumptions into the binary.
