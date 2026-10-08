# Issue: Check for Executable Files Lacking Shebang in `did test`

## Overview
Currently, `did test` verifies if non-executable files have a `#!` shebang line and reports a violation if execution permissions are missing. However, it does not check the inverse: executable files (`0o111` mode) that lack a shebang line or binary magic bytes.

When Unix attempts to execute a text file with execute permissions that lacks a shebang, `execve` returns `Exec format error (os error 8)`.

## Proposed Solution
In `cmd_test`, check if a file is executable (`is_executable(path)`). If it is executable, verify that its content either starts with a valid shebang line (`#!`) or binary header (e.g. `\x7fELF`, `MZ`). If not, flag a violation:
```
Executable file lacks shebang line (#!) or binary header: '<PATH>'
```

@bas080
