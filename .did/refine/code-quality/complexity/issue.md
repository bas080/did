# Feature Proposal: Rust Code Complexity Tooling Analysis

## Overview
Explores Rust code complexity tools (such as `cargo-cyclomatic`, `cargo-geiger`, or Clippy cognitive complexity lints `#![deny(clippy::cognitive_complexity)]`).

## Analysis & Recommendations
1. **Built-in Clippy Lints**: Rust includes `clippy::cognitive_complexity` and `clippy::cyclomatic_complexity` out of the box through `cargo clippy`.
2. **Recommendation**: Enable `clippy::cognitive_complexity` with custom thresholds in `Cargo.toml` or `src/lib.rs` / `src/main.rs` to detect overly complex functions automatically during `cargo clippy`.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `cargo clippy -- -D warnings` or cyclomatic complexity linter check be integrated into `.did/.hooks/test`?
