# Feature Specification: `did query` Line Number & Snippet Context Output

## Overview
Currently, `did query <QUERY>` (alias `did search <QUERY>`) matches task paths and file contents, but only outputs matching task paths line-by-line.

## Proposed Solution
Enhance `did query` / `did search` to support displaying matching line numbers and snippet context (similar to `grep -n` or `ripgrep` output) when matching task file content.

### Output Modes
To avoid cluttering the output and to reduce token consumption for AI agents, content matching will be optional:

1. **Path-only (Default)**: Only outputs the paths of matching tasks.
2. **Content-match (`-c` / `--content`)**: Outputs the matching lines from the file content.
3. **Line-numbers (`-n` / `--line-number`)**: When combined with `--content`, outputs the line number along with the snippet:
   ```
   path/to/task.md:12: Matching line snippet containing QUERY
   ```

This ensures the output remains lean by default while providing deep inspection capabilities when explicitly requested.

## Open Questions for Refinement (@bas080)
1. @bas080 Should line-number output (-n) implicitly enable content matching (-c), or require both flags to be passed?
