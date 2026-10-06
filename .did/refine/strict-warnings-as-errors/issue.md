# Feature Specification: Enforcing Strict Zero-Warning Rule Across All Test Tooling

## Overview
Evaluates and enforces strict zero-warning guarantees across all Rust compiler and linter tools (`cargo test`, `cargo check`, `cargo clippy`, `cargo llvm-cov`).

---

## Audit of Current Test Tooling State

1. **`cargo clippy`**:
   - **Current State**: Already enforced via `-- -D warnings` in `.github/workflows/test.yml` and pre-commit workflows. Compiler/clippy lints fail the build if warnings exist.
2. **`cargo test` & `cargo check`**:
   - **Current State**: Standard `cargo test` emits compiler warnings to `stderr` but succeeds with exit code `0` as long as tests pass.
   - **Required Fix**: Configure `RUSTFLAGS="-D warnings"` in local commands and GitHub Actions CI steps so any compiler warning (e.g. unused imports, dead code, needless borrows) causes `cargo test` and `cargo check` to fail immediately with a non-zero exit code.

---

## Action Plan to Implement Strict Warnings-as-Errors

1. **GitHub Actions CI Update (`.github/workflows/test.yml`)**:
   - Set environment variable `RUSTFLAGS: "-D warnings"` globally in the workflow:
     ```yaml
     env:
       RUSTFLAGS: "-D warnings"
     ```

2. **Cargo Configuration (`.cargo/config.toml`)**:
   - Create `.cargo/config.toml` to enforce warning errors locally for developers:
     ```toml
     [build]
     rustflags = ["-D", "warnings"]
     ```

3. **Script / Verification Command Update**:
   - Update developer test commands to include `RUSTFLAGS="-D warnings" cargo test`.
