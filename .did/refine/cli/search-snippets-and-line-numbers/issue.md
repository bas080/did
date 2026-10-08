# Feature Specification: `did query` Line Number & Snippet Context Output

## Overview
Currently, `did query <QUERY>` (alias `did search <QUERY>`) matches task paths and file contents, but only outputs matching task paths line-by-line.

## Proposed Solution
Enhance `did query` / `did search` to support displaying matching line numbers and snippet context (similar to `grep -n` or `ripgrep` output) when matching task file content:
```
path/to/task.md:12: Matching line snippet containing QUERY
```

- Add an optional flag (e.g. `-n` / `--line-number` or default content matching snippet output) to display the filename, line number, and matching content line.

@bas080
