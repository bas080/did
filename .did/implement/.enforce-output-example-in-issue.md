# Enforce output examples in issues affecting output behavior

When an issue describes a change to the output behavior of `did` (e.g., changing how `did status` or `did show` prints information), it MUST include "Before" and "After" output examples to clarify the desired change.

This is a guideline for a way of working, not an automated check; there is no script that will exit non-zero if these are missing.

## Requirements
- Output examples must be wrapped in markdown codeblocks.
- The `did help` hook (specifically the guidance on issue creation) should be updated to mention this requirement.

## Acceptance Criteria
- [ ] `did help` (or the relevant documentation provided by it) mentions the requirement for output examples.
- [ ] Guidelines are established for how to present these examples (e.g., using a `### Expected Output` section).

## Test Plan
- [ ] Run `did help` and verify that the output mentions the requirement for "Before" and "After" examples for output changes.
