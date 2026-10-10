# Integrate markdown terminal renderer

Currently, `did` prints markdown content as raw text to the terminal. To improve readability and the overall user experience, `did` should use a markdown rendering library to "pretty-print" output when writing to a TTY.

## Proposed Tooling
A library like `termimad` is a strong candidate as it allows rendering markdown directly to the terminal with support for:
- Headers (highlighted)
- Lists (indented)
- Code blocks (formatted and visually separated, e.g., with borders or background contrast, with full syntax highlighting for supported languages)
- Bold/Italic text
- High-contrast colors for visual distinction between different content types

## Requirements
- **Automatic Detection**: Render as markdown only when the output is a TTY. If the output is piped or redirected to a file, it must remain as raw markdown text.
- **Consistent Application**: All commands that output markdown or structured lists should pass their content through the renderer. This includes:
  - `did show`: Issue content and ancestor hooks.
  - `did status`: Actionable task lists and summaries.
  - `did help`: Help topics and guides.
  - `did query`: Matching snippets and search results.
  - `did test`: Health check violations and status reports.
  - All ancestor hook outputs across any command.
- **Visual Separation**: Output from ancestor hooks and special sections (like \"Related Items\") must be visually separated from the main command output. This should not be a simple manual print, but a **renderer capability**: the renderer should support wrapping a block of content in a visually distinct box or themed container.
- **Rich Visuals**: The output should be highly colorful and visually engaging, utilizing syntax highlighting for code blocks and distinct colors for different markdown elements to maximize readability.
- **Performance**: The rendering should not introduce noticeable latency to command execution.

## Acceptance Criteria
- [ ] `did show <issue>` renders headers, lists, and code blocks beautifully in the terminal.
- [ ] Code blocks feature full syntax highlighting for recognized languages.
- [ ] Ancestor hook output is clearly wrapped in a visual box or code block, separating it from the task content.
- [ ] `did status` and `did help` are visually enhanced via markdown rendering.
- [ ] Piping output (e.g., `did show <issue> > output.txt`) results in raw markdown, not terminal escape codes.
- [ ] The renderer handles various terminal widths gracefully.

## Related Work & Dependencies
The following features depend on the renderer's capabilities:
- `implement/cli/output-usability/`: Needs the \"Boxing\" capability for hooks and sections.
- `refine/show-related-items/`: Needs the \"Boxing\" capability for the Related Items list.
- `implement/cli/search-snippets-and-line-numbers/`: Needs syntax highlighting for search snippets.

## Test Plan
- [ ] Run `did show` on an issue with complex markdown (headers, lists, code blocks) and verify the visual output.
- [ ] Run `did status` and verify that the structural elements are highlighted.
- [ ] Run `did show <issue> | cat -v` to ensure no ANSI escape codes are present when the output is not a TTY.
- [ ] Test output on different terminal sizes to ensure no broken layouts.
