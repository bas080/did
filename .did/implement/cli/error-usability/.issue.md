# Feature Proposal: Improving Non-Zero Exit Diagnostics and Actionable Errors

## Overview
Documents real-world scenarios where calling `did` commands returned non-zero exit codes, and details actionable diagnostic improvements to guide developers and AI agents back on track.

## Documented Non-Zero Exit Scenarios
1. **`did show <DIRECTORY>`**: Calling `did show` on a directory fails with exit code `1`.
   - **Improvement**: Prints explicit stderr hint: `error: 'did show' requires a task file, got directory: 'backend'. Use 'did status backend' instead.`
2. **`did done <BLOCKED_TASK>`**: Calling `did done` on a task blocked by unresolved sub-items fails with exit code `1`.
   - **Improvement**: Lists all blocking prerequisite sub-items in stderr output.
3. **`did link <TARGET> <DEST>` Destination Collision**: Calling `did link` when symlink already exists fails with exit code `1`.
   - **Improvement**: Prints exact destination collision path and suggests removing or relocating existing file.
4. **`did rm <DIRECTORY>` without `-r`**: Removing directory without `-r` fails with exit code `1`.
   - **Improvement**: Suggests exact command `did rm -r <DIRECTORY>`.

## Actionable Recommendations
- Always write failure diagnostics to `stderr` rather than `stdout`.
- Include concrete, copy-pasteable CLI command suggestions in error messages.
