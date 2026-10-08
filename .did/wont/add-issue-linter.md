# Implement a linting tool for `.did/` issues

Create a tool (possibly as a `did` subcommand or a hook) that verifies if issues follow the project's standards.

## Requirements
- Check if issues follow the standard template (see `refine/standardize-issue-templates.md`).
- Verify mandatory sections:
    - Description (Title and body)
    - Requirements
    - Acceptance Criteria
- Verify that issues affecting output include "Before" and "After" examples in the `## Expected Output` section.
- Warn or error if the issue is in `.did/implement/` but lacks a "Test Plan" section.

## Acceptance Criteria
- [ ] `did lint` (or similar) can scan `.did/` and report violations.
- [ ] Linter can be integrated into the `did test` or `did status` flow to ensure quality.
- [ ] Clear error messages guiding the user on how to fix the issue.

## Test Plan
- [ ] Create an issue that misses a mandatory section (e.g., Acceptance Criteria) and verify `did lint` reports it.
- [ ] Create an issue in `.did/implement/` without a Test Plan and verify `did lint` reports it.
- [ ] Create an issue that describes an output change but lacks "Before/After" examples and verify `did lint` reports it.
- [ ] Create a perfectly formatted issue and verify `did lint` reports no errors.
