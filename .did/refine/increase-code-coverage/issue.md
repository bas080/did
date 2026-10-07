# Feature Specification: Increase Code Coverage to 100%

## Overview
Tracks the effort to increase code coverage for `did` from the current baseline threshold of **76.93% line coverage** up to **100% full test coverage**.

---

## Baseline Status
- **Current Line Coverage**: **76.93%** (573/646 lines covered)
- **Current CI Threshold**: Enforced at **76%** (`cargo llvm-cov --fail-under-lines 76`)

---

## Targeted Modules for Coverage Improvement

1. **`src/commands.rs` (Current: 76.09%)**:
   - Add tests for failure cases in `cmd_add` (e.g., $EDITOR failures, read-only paths).
   - Add tests for `cmd_link` failure modes (non-existent target, destination collisions).
   - Add tests for `cmd_show` non-zero exit codes when executable hooks fail.
   - Add tests for `cmd_autocomplete` with invalid shell names.

2. **`src/main.rs` (Current: 53.85%)**:
   - Add tests verifying bare invocation CLI instructions and error handling when `$HOME` or directory lookup fails.

3. **`src/repo.rs` (Current: 90.00%)**:
   - Add unit tests for edge cases in `relative_display_path` and `resolve_path`.


## Open Questions for Refinement (@bas080)
1. @bas080 What should the target minimum code coverage threshold be for CI enforcement (e.g. 85%, 90%, 100%)?
