# Agent Instructions & Core Rule

Welcome! This repository uses `did`, a filesystem-native task and dependency tracker written in Rust.

---

## CORE RULE: All Work Begins with a `did` Issue

**IMPORTANT**: All work—whether fixing a bug, adding a feature, or performing refactoring—MUST be started by first creating or picking up a `did` issue:

```bash
did add refine/<issue-name>/issue.md -m "Detailed description of the issue"
```

Detailed instructions and repository established way of working are provided dynamically via `did show` through `.did/.hooks/show`.
