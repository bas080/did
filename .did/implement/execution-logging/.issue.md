# Feature Specification: Execution Logging (`DID_LOG_PATH`) for AI Telemetry

## Overview
Sub-issue of `ai-effectiveness-metrics`. Implements opt-in execution logging to track CLI command invocations, stdout/stderr outputs, and exit status codes. Telemetry logging is activated by setting the `DID_LOG_PATH` environment variable.

---

## Detailed Requirements

### 1. Activation via Environment Variable (`DID_LOG_PATH`)
- Execution logging is **DISABLED BY DEFAULT**.
- Logging is enabled **ONLY** when the `DID_LOG_PATH` environment variable is defined and non-empty.

### 2. Log File Path Resolution
- **Relative Paths**: If `DID_LOG_PATH` is a relative path (e.g., `DID_LOG_PATH=.did.log` or `DID_LOG_PATH=logs/exec.log`), it is automatically resolved relative to the `.did/` state directory root.
- **Absolute Paths**: If `DID_LOG_PATH` is an absolute path (e.g., `/tmp/did.log`), it is used directly.

### 3. Log Entry Structure
Each command invocation appends a structured entry to the target log file:

```
=== [TIMESTAMP_ISO8601] COMMAND: did <ARGS...> ===
CWD: <RELATIVE_CWD>
EXIT CODE: <EXIT_CODE>
--- STDOUT ---
<STDOUT_TEXT>
--- STDERR ---
<STDERR_TEXT>
==================================================
```

### 4. Non-Blocking Execution
- Failure to open or append to `DID_LOG_PATH` (e.g. read-only filesystem or missing directory) must print a warning to `stderr` but **MUST NOT** interrupt or fail the primary `did` command execution.

---

## Concrete Integration Test Requirements (`tests/cli_tests.rs`)

1. **`test_execution_logging_enabled_via_did_log_path`**:
   - Set `DID_LOG_PATH=.did.log`.
   - Run `did status`.
   - Assert `.did/.did.log` exists.
   - Assert log contains `COMMAND: did status`, `EXIT CODE: 0`, and stdout/stderr output sections.

2. **`test_execution_logging_disabled_by_default`**:
   - Run `did status` without `DID_LOG_PATH` set.
   - Assert `.did/.did.log` is NOT created.
