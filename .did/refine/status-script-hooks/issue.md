# Feature Proposal: User-Defined Directory Scripts on `did status`

## Overview
Explore how user-defined directory scripts should behave when `did status` is executed across `.did/` state directories.

## Open Questions & Exploration Points

1. **Dynamic Task Filtering**:
   - Should directory scripts be able to intercept `did status` and dynamically hide or expose actionable tasks based on custom logic (e.g. checking git branch, build status, or external API status)?

2. **Pre-Status Assertion Checks**:
   - If a directory script fails (non-zero exit code) during `did status`, should it suppress status listing for that directory or output a warning on `stderr`?

3. **Performance Impact**:
   - Running scripts recursively during `did status` across large task trees could impact execution speed. Explore script caching or selective hook execution.
