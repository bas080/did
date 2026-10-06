# Feature Specification: Restrict Parent Context Files in `did show`

## Overview
When running `did show <PATH>`, top-down parent directory context output will ONLY print file contents if the parent file is named `agents.md` or `AGENTS.md`.

## Detailed Rules

1. **Only Print `agents.md` / `AGENTS.md` Contents**:
   - In parent directories from root `.did/` down to target task file's directory:
     - File contents of regular parent task/issue files (e.g. `parent_notes.md`, `specs.md`) will **NOT** be printed to stdout.
     - File contents of parent files named `agents.md` or `AGENTS.md` **WILL** be printed as context headers and content.

2. **Filepath Header Display**:
   - Parent filepaths are listed as headers, but their contents are omitted unless the file is `agents.md` or `AGENTS.md`.
   - This prevents cluttering output with irrelevant sibling issue details that the current task does not need to know about.

## Files In This Repository Requiring Renaming to `agents.md`

To align with this restriction so that parent directory context files continue to be displayed during `did show`, the following header files in `.did/` should be renamed:

1. `.did/could/could.md` → `.did/could/agents.md` (or `AGENTS.md`)
2. `.did/refine/refine.md` → `.did/refine/agents.md` (or `AGENTS.md`)
3. `.did/implement/implement.md` → `.did/implement/agents.md` (or `AGENTS.md`)
