# Feature Proposal: Improving `did show` and `did status` Output Formatting for Humans & AI Agents

## Overview
Proposes output formatting improvements to `did show` and `did status` to maximize clarity, readability, and machine-parseability for human developers and LLM AI agents.

## 1. `did show` Output Improvements
- **Section Headers**: Use clean ASCII borders or explicit markdown section headers to separate ancestor hook output from task file content:
  ```text
  --- Ancestor Hook: .did/.hooks/show ---
  [Hook Guidelines]

  --- Task File: backend/auth/jwt.md ---
  [Task Content]
  ```
- **Diagnostic Hints on Failure**: When a task is blocked, write formatted diagnostic trees to `stderr`:
  ```text
  error: task 'backend/auth.md' is blocked by 2 unresolved sub-items:
    ├─ backend/auth/jwt.md (leaf task)
    └─ backend/auth/session.md (leaf task)
  ```

## 2. `did status` Output Improvements
- **Clean `stdout` Array**: Keep `stdout` as a newline-separated list of actionable relative paths for shell piping.
- **Stderr Summary & Diagnostic Tree**: Write category counters and blocked diagnostics exclusively to `stderr`:
  `[2 actionable, 1 blocked, 5 closed]`
  When `-b` or `-a` is passed, render blocked item dependency links to `stderr`.
