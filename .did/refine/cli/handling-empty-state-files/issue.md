# Feature Specification: Handling Empty State Files on `did add`

## Overview
Currently, running `did add <path>` without `-m` opens ``. If the user exits the editor without adding content, or if non-interactive invocation occurs, a blank file is created.

## Proposed Solution
Enhance `did add <PATH>` handling when no `-m` message or stdin content is provided:

1. When the editor is invoked and the user closes it without typing any body content, `did` will not create a blank file. Instead, it will automatically generate a default Markdown header derived from the task path's filename stem.
   - Example: `did add features/auth-fix.md` $\rightarrow$ creates file with content `# auth-fix`.
2. This ensures that every task in the system has at least a basic title, maintaining consistency and visibility in `did status` and `did show`.

@bas080
