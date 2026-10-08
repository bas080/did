# Feature Specification: Allow `did show` on Blocked Nodes by Default

## Overview
Currently, running `did show <PATH>` on a task file that has open/unresolved sub-items in child directories fails with an error unless the `-a` (`--all`) flag is supplied (`did show -a <PATH>`).

## Proposed Change
Since inspecting task content is non-destructive, `did show <PATH>` should display the task body by default even when blocked, followed by a brief list of blocking unresolved sub-items at the bottom:
```
<Task Content>

[Blocked by unresolved sub-items:]
  - child/subtask.md
```

This eliminates the need to re-run the command with `-a` when inspecting blocked parent tasks.

@bas080
