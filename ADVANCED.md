# did - Advanced Documentation & Reference

This document covers advanced topics, architecture details, environment variables, and lifecycle hook specifications for `did`. For quick start instructions, see [README.md](README.md).

---

## Software Factories & Lifecycle Hooks Specifications

In `did`, any directory in `.did/` can contain a `.hooks/` directory with hook scripts named after CLI subcommands (`add`, `show`, `status`, `close`, `open`, `test`, `blocks`, `mv`, `rm`, `query`, `help`).

When a `did` subcommand executes:
1. **Hook Discovery**: `did` traverses from the target path up to `.did/` looking for matching `.hooks/<cmd>` files.
2. **Hook Execution**: If executable, hooks run before the subcommand completes, receiving context via environment variables (`DID_EVENT`, `DID_TARGET`, `DID_DEST`, `DID_OLD`, `DID_NEW`, `DID_REPO_ROOT`, `DID_STATE_DIR`).
3. **Quality Gates**: If a hook exits with a non-zero exit code, `did` aborts the operation.

### Supported Lifecycle Hooks

| Hook | CLI Trigger | Description |
| :--- | :--- | :--- |
| `add` | `did add <PATH>` | Executed during task creation before creating file. |
| `show` | `did show <PATH>` | Executed during task display to output guidelines or context. |
| `status` | `did status [PATH]` | Executed before status listing. |
| `close` | `did close <PATH>` / `did done` | Executed before resolving a task. |
| `open` | `did open <PATH>` / `did undone` | Executed before reopening a task. |
| `blocks` | `did blocks <TARGET> <DEST>` / `did link` | Executed before symlink creation. |
| `mv` | `did mv <OLD> <NEW>` | Executed before moving or renaming tasks. |
| `rm` | `did rm <PATH>` | Executed before deleting task files or directories. |
| `query` | `did query <QUERY>` | Executed before searching tasks and content. |
| `test` | `did test` | Executed during repository health check. |
| `help` | `did help [TOPIC]` | Executed when invoking help or running without subcommands to display way-of-working instructions. |

---

## Complete Environment Variable Reference

| Variable | Description |
| :--- | :--- |
| `DID_STATUS_PATH` | Specifies a default subtree path under `.did/` when running `did status` without an explicit path argument. |
| `DID_STATUS_LIMIT` | Sets the maximum number of items returned by `did status` or `did query` before displaying a truncation notice on `stderr`. |
| `DID_LOG_PATH` | Activates XML execution telemetry logging. Relative paths are resolved relative to the parent directory where `.did/` lives. |
| `DID_DEBUG` | Enables verbose diagnostic logging on `stderr`. |
| `DID_COLOR` | Controls colored terminal rendering (`1` to force, `0` to disable). |
| `DID_NO_COLOR` | Disables colored terminal output when set (`1`). |
| `DID_BOX` | Controls border box containers around extra outputs (`1` to force, `0` to disable). |
| `DID_NO_BOX` | Disables border box containers when set (`1`), rendering `---` section breaks instead. |
| `DID_THEME` | Configures markdown and syntax highlighting themes (`dark`, `light`/`github`, `solarized`, `mocha`). |
| `DID_SYNTAX_THEME` | Overrides syntax highlighting theme specifically (`InspiredGitHub`, `Solarized (dark)`, `base16-ocean.dark`, `base16-mocha.dark`). |
| `DID_RELATED_LIMIT` | Sets maximum number of related items displayed in `did show` (defaults to `5`). |
| `EDITOR` | Specifies the text editor to invoke when running `did add PATH` without a `-m` message flag (defaults to `vi`). |

---

## Telemetry & Logging Architecture

When `DID_LOG_PATH` is set, `did` appends formatted XML log entries for every command invocation:
```xml
<invocation timestamp="2026-10-09T00:00:00Z">
  <command>status</command>
  <args></args>
</invocation>
```
Relative log paths are resolved relative to the parent directory of `.did/`.
