# Refactoring Issue: Code and Configuration Duplication Detection

## Overview
Tracks areas of code and configuration duplication identified during development.

## Areas for Refactoring
1. **Hook Directory Traversal**: Ensure `run_ancestor_hooks` in `src/commands.rs` serves as the single source of truth for all lifecycle event executions (`add`, `show`, `done`, `status`, `link`, `mv`, `test`, `query`).
2. **CI and Local Cargo Flags**: Consolidate `RUSTFLAGS="-D warnings"` configuration between `.cargo/config.toml` and `.github/workflows/test.yml`.


## Open Questions for Refinement (@bas080)
1. @bas080 Which code duplication tool (e.g. `cargo-duppy` or custom script) should be mandated in CI checks?
