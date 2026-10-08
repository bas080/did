# Verify Checklist Completion for Resolved Issues @bas080

## Overview
Ensure that issues marked as resolved (hidden files starting with `.`) do not contain any unchecked checklist items (`- [ ]`). If a task is considered "done", all its acceptance criteria and sub-tasks must be explicitly checked off. Finding an unchecked item in a resolved file indicates a premature completion and should be flagged as a health violation.

## Proposed Implementation
Enhance the `did test` command to validate the content of resolved issues:

1. **Resolved File Scanning**: During the walk of the `.did/` directory, identify files that are marked as done (filenames starting with `.`).
2. **Pattern Matching**: Read the content of these resolved files and search for the markdown unchecked checkbox pattern: `- [ ]`.
3. **Violation Reporting**: If unchecked items are found in a resolved file:
   - Report a health violation.
   - The violation message should clearly state that the issue was marked done but still has open items: `Violation: Resolved issue '.path/to/issue.md' contains unchecked checklist items`.
   - `did test` should exit with a non-zero code.
4. **Success State**: Resolved files with only checked items (`- [x]`), no checkboxes, or active files (not starting with `.`) are ignored by this specific check.

## Requirements
- [ ] Implement a check for `- [ ]` patterns specifically for resolved (dot) files within `did test`.
- [ ] Report violations in the existing `did test` output format.
- [ ] Ensure that active (non-hidden) issues are *not* flagged by this check (since they are expected to have open items).
- [ ] Maintain performance during the repository-wide health check.

## Acceptance Criteria
- [ ] `did test` fails when a resolved issue (starting with `.`) contains a `- [ ]` item.
- [ ] `did test` succeeds when all resolved issues have their checklist items checked (`- [x]`).
- [ ] `did test` succeeds if an active issue contains `- [ ]` (it should only flag resolved ones).
- [ ] The output clearly identifies the resolved file causing the violation.

## Test Plan
- [ ] Create an issue with `- [ ]` items. Mark it as done (`did done`). Run `did test` and verify it fails.
- [ ] Check off all items in that resolved issue. Run `did test` and verify it now succeeds.
- [ ] Create an active issue with `- [ ]` items. Run `did test` and verify it *succeeds* (or at least is not flagged by this specific check).
- [ ] Mark multiple issues as done, one with unchecked items and others without. Run `did test` and verify only the incorrect one is reported.
