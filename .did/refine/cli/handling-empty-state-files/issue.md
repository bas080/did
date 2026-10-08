# Feature Specification: Handling Empty State Files on `did add`

## Overview
Currently, running `did add <path>` without `-m` opens ``. If the user exits the editor without adding content, or if non-interactive invocation occurs, a blank file is created.

## Proposed Solution
Enhance `did add <PATH>` handling when no `-m` message or stdin content is provided:
1. When invoking ``, if the user closes the editor without typing any body content, default to using a formatted title derived from the task path's filename stem (e.g. `# Task Title`).
2. Alternatively, require either `-m`, `stdin`, or non-empty editor buffer, or fallback to auto-generating a default Markdown header based on the file stem instead of leaving an empty file.

@bas080
