# Feature Specification: Automatic File-to-Directory Conversion on `did add`

## Overview
Currently, if a task file exists at `parent.md` or `parent`, running `did add parent/child.md` fails with an OS error or path collision error because `parent` is a file rather than a directory. To create sub-tasks, a user or agent must manually convert `parent` into a directory containing an index file (`parent/index.md`).

## Proposed Feature
When `did add parent/child.md` is executed and `parent` is an existing file (not a directory):
1. Automatically convert `parent` file into a directory named `parent/`.
2. Move the original content of `parent` into `parent/index.<ext>` (where `<ext>` is the original file's extension, e.g., `.md` or `.txt`).
3. Update any relative symlinks across `.did/` that previously pointed to `parent` to point to the new index file (`parent/index.<ext>`).
4. Create the new child task `parent/child.md`.
