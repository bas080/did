# Task Prioritization Way-of-Working

`did` embraces a filesystem-native philosophy: issue prioritization is defined by team workflow and directory organization rather than rigid CLI features.

---

## Prioritization Methods

### 1. Subtree Folder Categorization
Organize tasks inside dedicated priority directories within `.did/`:
- `.did/must/` (High Priority)
- `.did/should/` (Medium Priority)
- `.did/could/` (Low Priority)

When evaluating actionable work, inspect higher priority subtrees first:
```bash
did status must
did status should
```

### 2. Task Metadata Headers
Include priority metadata at the top of task Markdown files:
```markdown
# Priority: P0 (Critical)
```

This allows searching and filtering by priority using `did search`:
```bash
did search "Priority: P0"
```
