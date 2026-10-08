# Feature Specification: Restrict Parent Context Files in `did show` to `hooks/show`

## Overview
Replaces `agents.md` with `hooks/show`. When running `did show <PATH>`, top-down parent directory context output will ONLY print file contents if the parent file is named `hooks/show`.

## Detailed Rules

1. **Only Print `hooks/show` Contents**:
   - In parent directories from root `.did/` down to target task file's directory:
     - File contents of regular parent task/issue files (e.g. `parent_notes.md`, `specs.md`) will **NOT** be printed to stdout.
     - Files named `hooks/show` **WILL** be processed:
       - If executable: run script and print `stdout`.
       - If non-executable: print path header and file contents to `stdout`.

2. **Files In This Repository Requiring Renaming to `hooks/show`**:
   1. `.did/refine/refine.md` → `.did/refine/hooks/show`
   2. `.did/implement/implement.md` → `.did/implement/hooks/show`
