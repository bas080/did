# Feature Specification: Bulk Operations for `did` CLI

## Overview
Currently, many `did` operations (`mv`, `done`, `rm`) require targeting a single task path. As project size increases, managing tasks individually becomes tedious. This proposal introduces bulk operations by supporting multiple arguments, allowing the shell to expand globs.

## Proposed Enhancements

### 1. Bulk Move (`did mv`)
Support moving multiple tasks at once. The shell expands the globs into a list of paths.
- Example: `did mv .did/refine/cli/*.md .did/implement/cli/`
- **Requirement**: The destination must be a directory. If multiple items are moved, the command fails if the destination is a file.

### 2. Bulk Completion (`did done`)
Allow completing multiple tasks in a single command.
- Example: `did done .did/implement/feature-x/*.md`
- This complements the existing `did done -r <DIR>` functionality.

### 3. Bulk Removal (`did rm`)
Safely remove multiple tasks and their associated symlinks.
- Example: `did rm .did/refine/obsolete/*.md`
- Support for the `-r` (recursive) flag to remove directories and their contents.

## Implementation Requirements
- **Argument Handling**: `did` will treat all arguments except the final one (in the case of `mv`) as target paths.
- **Error Handling**: If any operation in a bulk set fails, the command must **stop immediately** at the first error and report the failure.
- **Safety**: Rely on the `-r` flag for explicit recursive intent and Git version control for recovery from accidental deletions.

@bas080

@bas080
