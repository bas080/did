# Agent Instructions & Core Guidelines

Welcome! This repository uses `did`, a filesystem-native task and dependency tracker written in Rust.

---

## Core Rules

1. **Use `did` CLI Tool**: All issue tracking and state operations MUST be performed using the `did` CLI binary.
2. **Issue Creation First**: All work—whether fixing a bug, adding a feature, or performing refactoring—MUST start by creating or picking up a `did` issue:
   ```bash
   did add refine/<issue-name>/issue.md -m "Detailed issue description"
   ```
3. **Query Help & Guidelines**: For CLI and repository guidelines, inspect `did help`:
   ```bash
   did help
   ```
4. **Visual Output Verification & Aesthetic Perfection**: Any change to the visual output of the CLI MUST be visually inspected across every component and every aspect of those components to make sure it is just right. You MUST test the view variations by providing relevant flags (e.g. `--tree`, `--blocked`, `-a`), obsessing over visual polish, and iterating until the output formatting and colors are clean and perfect.
5. **Work Completion Verification**: Work is ONLY considered done if `did test` executes cleanly and exits zero with no health violations:
   ```bash
   did test
   ```

Detailed instructions and workspace way-of-working guidelines are offloaded to `.did/.hooks/` and rendered dynamically when relevant via `did show` and `did status`.
