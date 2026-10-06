# Feature Specification: Execution Logging (`.did.log`) for AI Telemetry

## Overview
Sub-issue of `ai-effectiveness-metrics`. Implements opt-in execution logging to track CLI command invocations, stdout/stderr outputs, and exit status codes in `.did.log`. This telemetry provides an audit log for evaluating AI agent effectiveness and tool usage patterns.

---

## Detailed Requirements

### 1. Opt-In Environment Variable (`DID_LOG`)
- Execution logging is **DISABLED BY DEFAULT**.
- Logging is enabled **ONLY** when the `DID_LOG=1` or `DID_LOG=true` environment variable is present in the execution environment.
- If a custom path is provided in `DID_LOG_PATH`, log to that path; otherwise default to `.did.log` in the workspace root.

### 2. Log File Location
- Default log file path: `.did.log` located at the root of the project workspace (alongside `.did/`).

### 3. Log Entry Structure
Each command invocation appends a structured entry to `.did.log`:

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
- Failure to open or append to `.did.log` (e.g. read-only filesystem) must print a warning to `stderr` but **MUST NOT** interrupt or fail the primary `did` command execution.

---

## Concrete Integration Test Requirements (`tests/cli_tests.rs`)

1. **`test_execution_logging_enabled`**:
   - Set `DID_LOG=1`.
   - Run `did status`.
   - Assert `.did.log` exists in workspace root.
   - Assert `.did.log` contains `COMMAND: did status`, `EXIT CODE: 0`, and stdout/stderr output sections.

2. **`test_execution_logging_disabled_by_default`**:
   - Run `did status` without `DID_LOG` set.
   - Assert `.did.log` is NOT created.
