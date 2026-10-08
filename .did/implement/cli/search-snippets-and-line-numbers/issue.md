# Feature Specification: `did query` Line Number & Snippet Context Output

## Overview
Currently, `did query <QUERY>` (alias `did search <QUERY>`) matches task paths and file contents, but only outputs matching task paths line-by-line.

## Proposed Solution
Enhance `did query` / `did search` to display matching line snippets and context by default when matching task file content, similar to `grep` or `ripgrep` output.

### Output Modes
1. **Content-match (Default)**: By default, `did query` will output the matching lines from the file content. If the output is a TTY, these snippets should be passed through the markdown renderer (see `refine/integrate-markdown-renderer.md`) to ensure code blocks and formatting are preserved and visually separated.
2. **Line-numbers (`-n` / `--line-number`)**: Adds the line number to the matching snippet output:
   ```
   path/to/task.md:12: Matching line snippet containing QUERY
   ```

This provides immediate context for search results while allowing for precise line-referencing when needed.
