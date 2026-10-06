# Feature Specification: `did clean-log` / Telemetry Log Management

## Overview
Proposes a `did clean-log` subcommand (or `did log --clear`) to truncate or clear the telemetry log file configured via `DID_LOG_PATH`.

---

## Detailed Requirements

1. **Log Truncation**:
   - Empties the file specified by `DID_LOG_PATH` without deleting file permissions.
2. **Path Resolution**:
   - Resolves `DID_LOG_PATH` relative to the workspace root (parent of `.did/`).
3. **Safeguard**:
   - Prints `Telemetry log cleared.` on `stderr`.
   - If `DID_LOG_PATH` is unset, prints `No DID_LOG_PATH environment variable set.` on `stderr`.
