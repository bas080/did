# Improve `did status` output readability

The `did status` command is central to understanding the project state. Its output should be refined for maximum clarity and scannability.

## Requirements
- Use consistent formatting for different issue states (Refine, Implement, Done, etc.).
- Use color-coding to distinguish between critical paths and optional tasks:
    - Red/Yellow for `refine/` (needs attention).
    - Green for `implement/` (ready for work).
    - Gray/Dim for closed/resolved issues.
- Ensure the summary of open vs closed issues is prominent (e.g., at the top or bottom in a bold summary line).
- Group issues by their directory (Refine, Implement, etc.) with clear headers.

## Acceptance Criteria
- [ ] `did status` output is visually structured with clear group headers.
- [ ] Colors are used meaningfully to highlight state and priority.
- [ ] The summary (e.g., `[X open, Y closed]`) is easy to spot.
- [ ] The output remains readable across different terminal widths.

## Test Plan
- [ ] Run `did status` and verify that issues are grouped by their state (Refine, Implement, etc.).
- [ ] Verify that different states have distinct, helpful colors.
- [ ] Verify that the summary of open/closed issues is prominently displayed.
- [ ] Shrink terminal width and verify that the output still looks reasonable.
