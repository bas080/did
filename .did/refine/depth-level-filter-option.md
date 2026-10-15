# Level / Depth Option for Filtering Output

Add a level/depth option (e.g., `--level <N>` or `--depth <N>`) to `did status` and `did query` to display notes at an exact level, everything above level N, or everything below level N.

## Before & After Output Examples

### Example: `did status --level 2`
```
# Before
section1/topic1/noteA.md
section1/topic2/noteB.md
section2/noteC.md

# After
section1/topic1/noteA.md
section1/topic2/noteB.md
```

## Requirements
- Support filtering by exact depth, maximum depth, or minimum depth (e.g., `-l 2`, `--level 2`).
- Works with `did status` and `did query`.

## Questions
- [ ] @bas080: What syntax and flag names do you prefer for specifying exact, max, or min depth (e.g. `--level 2`, `--depth 2`, or operators like `<=2`, `>=2`)?
