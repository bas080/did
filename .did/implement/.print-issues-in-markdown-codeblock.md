# Wrap issue content in markdown codeblocks when printing

When `did` prints the content of an issue file (e.g., via `did show`), it should wrap the content in a markdown codeblock to preserve formatting and improve readability when the output is integrated into other markdown-based documents or tools. This is mandatory for all issue content printed.

## Requirements
- The output must be wrapped in a markdown codeblock.
- The codeblock language should be determined by the file extension of the issue file (e.g., `.md` -> `markdown`, `.rs` -> `rust`).
- Default to `text` if no extension is available.

## Acceptance Criteria
- [ ] `did show <issue>` always wraps the output in a markdown codeblock.
- [ ] The language tag is correctly applied based on the file extension.

## Test Plan
- [ ] Run `did show some-issue.md` and verify it's wrapped in \`\`\`markdown ... \`\`\`.
- [ ] Run `did show some-file.rs` and verify it's wrapped in \`\`\`rust ... \`\`\`.
- [ ] Run `did show some-file.txt` and verify it's wrapped in \`\`\`text ... \`\`\`.
