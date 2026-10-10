# Show Related Items in `did show`

## Overview
Enhance `did show` to automatically discover and list related issues based on the title of the current task. This helps users find connected work without manual searching.

## Proposed Implementation
When `did show <path>` is executed:
1. **Title Extraction**: Extract the title from the issue file (typically the first non-empty line). If no title is found, fall back to using the filename (without extension) as the query.
2. **Query Execution**: Use the extracted title/filename as a query for `did query` to find other issues with similar terms in their path or content.
3. **Filtering**: Remove the current issue from the result set to avoid self-reference.
4. **Capped Output**: Display a "Related Items" section containing a limited number of the most relevant results. The limit should be configurable via `DID_RELATED_LIMIT` (defaulting to 5).
5. **UI Integration**: The section should be clearly separated from the issue content, utilizing the "Boxing" and "Rich Visuals" capabilities defined in `refine/integrate-markdown-renderer.md` and `implement/cli/output-usability/`.

## Requirements
- [ ] Extract title correctly from the issue file.
- [ ] Integrate with the existing `did query` logic.
- [ ] Implement a cap on the number of related items shown.
- [ ] Ensure the "Related Items" section is visually distinct.
- [ ] The feature should be silent when the output is not a TTY.

## Acceptance Criteria
- [ ] Running `did show` on an issue displays a list of related issues based on its title.
- [ ] The list is capped at the defined limit (`DID_RELATED_LIMIT`).
- [ ] The current issue is not listed as a related item.
- [ ] No related items are shown if no matches are found.

## Test Plan
- [ ] Create two issues with similar titles (e.g., "Auth: JWT Implementation" and "Auth: Session Management").
- [ ] Run `did show` on one and verify the other appears in the "Related Items" section.
- [ ] Set `DID_RELATED_LIMIT=2` and create 5 similar issues; verify only 2 are shown.
- [ ] Run `did show` on an issue with a unique title and verify no related items are listed.
- [ ] Run `did show` on an empty file and verify it uses the filename for the query.
