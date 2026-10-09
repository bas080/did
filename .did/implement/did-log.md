# Issue: `did log` Subcommand to View Task Git History

## Goal
Implement a `did log` subcommand that queries Git history to present a chronologically ordered, markdown-formatted log of task events across the `.did/` state directory (or a specific task path).

## Requirements
1. Subcommand syntax: `did log [PATH] [--since <SINCE>] [--until <UNTIL>]`.
2. Hides from main CLI help list if `git` executable is missing from `PATH`. `did help log` remains accessible.
3. Outputs aggregate summary box on `stderr`.
4. Outputs styled Markdown event list on `stdout`.

## Output Examples

### Before
```
$ did log
error: unrecognized subcommand 'log'
```

### After
```
$ did log
--- stderr ---
┌──────────────────────────────────────────────────┐
│ did log: 3 events (2025-03-01 to 2025-03-08)     │
└──────────────────────────────────────────────────┘
--- stdout ---
- **a1b2c3d** Task *created*: `refine/auth.md` by *Alice* on `2025-03-01`
- **e4f5g6h** Task *moved*: `refine/auth.md` -> `implement/auth.md` by *Bob* on `2025-03-05`
- **i7j8k9l** Task *closed*: `implement/auth.md` -> `implement/.auth.md` by *Charlie* on `2025-03-08`
```
