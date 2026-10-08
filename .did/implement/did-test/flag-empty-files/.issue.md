# Feature Specification: Flag Empty Task Files in `did test`

## Overview
Currently, `did test` checks for broken symlinks, hook directory sanctity, and executable permission / shebang consistency. It does not flag empty task files (0 bytes).

## Proposed Solution
In `cmd_test`, check if a task file is completely empty (0 bytes) or contains only whitespace. If such a file is found in `.did/`, record a violation:
```
Empty or whitespace-only task file found: '<PATH>'
```

This ensures state files contain valid issue content or headers, preventing accidental empty files from being tracked as active tasks.

@bas080
