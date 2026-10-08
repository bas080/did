# Improve hook output formatting

The current output of hooks often begins by printing the path to the hook file being executed (e.g., `.hooks/status` or `refine/.hooks/status`). This is redundant and clutters the output. Additionally, the output lacks consistent grouping, making it harder to scan.

## Expected Output

### Example 1: `did status`
**Before**
```
.hooks/status
# Status Listing (`.did/.hooks/status`)
backlog/ai-effectiveness-metrics/issue.md
backlog/coverage-badge/code-coverage-100
backlog/release-android/task.md
backlog/release-ios/task.md
backlog/semantic-search/issue.md
implement/cli/auto-convert-file-to-directory/issue.md
implement/cli/bulk-operations/issue.md
implement/cli/done-recursive/issue.md
implement/cli/handling-empty-state-files/issue.md
implement/cli/search-snippets-and-line-numbers/issue.md
Notice: status limit reached (10/32 items shown). Use status on a specific directory, search, or adjust limit.
[5 blocked, 65 closed]
```

**After**
```
# Status Listing

Backlog
backlog/ai-effectiveness-metrics/issue.md
backlog/coverage-badge/code-coverage-100
backlog/release-android/task.md
backlog/release-ios/task.md
backlog/semantic-search/issue.md

Implement
implement/cli/auto-convert-file-to-directory/issue.md
implement/cli/bulk-operations/issue.md
implement/cli/done-recursive/issue.md
implement/cli/handling-empty-state-files/issue.md
implement/cli/search-snippets-and-line-numbers/issue.md

Notice: status limit reached (10/32 items shown). Use status on a specific directory, search, or adjust limit.
[5 blocked, 65 closed]
```

### Example 2: `did test` (Scripted Output)
**Before**
```
.hooks/test
# Health Check (`.did/.hooks/test`)
error: issue files in .did/refine/ missing @bas080 tag:

  - .did/refine/improve-hook-output-formatting.md

Command exited with code 1
```

**After**
```
# Health Check

Issues found in .did/refine/:
  - .did/refine/improve-hook-output-formatting.md (missing @bas080 tag)

Status: FAILED
```

## Requirements
- **Remove Hook Paths**: Stop printing the location/path of the hook file at the start of the output.
- **Group Output**: Ensure that logically related lines of output are grouped together (e.g., using empty lines to separate sections).
- **Consistent Formatting**: Standardize how hooks present information so the user experience is uniform regardless of which hook is running.

## Acceptance Criteria
- [ ] `did status`, `did help`, and other hook-driven commands no longer print the hook file path (e.g., `.hooks/status` is removed).
- [ ] Hook output is visually structured with clear groupings.
- [ ] The output is cleaner and more focused on the content rather than the execution details.

## Test Plan
- [ ] Run `did status` and verify that the leading `.hooks/status` and `refine/.hooks/status` lines are gone.
- [ ] Run `did help` and verify that the leading `.hooks/help` line is gone.
- [ ] Inspect various hook outputs to ensure they are grouped logically and look consistent.
