# Feature Specification: Default Status Path Environment Variable

## Overview
Allow users to define an environment variable (e.g. `DID_STATUS_PATH`) that specifies a default subtree under `.did/` (e.g. `.did/implement`) when running `did status`. This reduces the number of items printed by default to focus attention on active areas.

## Proposed Behavior
- If no path argument is provided to `did status`, the tool checks for the `DID_STATUS_PATH` environment variable.
- If set, `did status` operates on that path as if it were passed as a CLI argument.
- Explicit CLI path arguments should always take precedence over the environment variable.

@bas080
