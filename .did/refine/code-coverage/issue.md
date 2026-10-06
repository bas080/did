# Feature Specification: Code Coverage Tooling with `cargo-llvm-cov`

## Overview
Configures `cargo-llvm-cov` as the official code coverage tool for `did`. `cargo-llvm-cov` leverages LLVM source-based code coverage (`-C instrument-coverage`) built into the Rust compiler to generate accurate statement and branch coverage reports without modifying binaries or causing compiler panics.

---

## Detailed Requirements

### 1. Tooling Standard: `cargo-llvm-cov`
- **Tool Selection**: `cargo-llvm-cov` is selected over `tarpaulin` due to native LLVM coverage accuracy, cross-platform support (Linux, macOS, Windows), and seamless integration with `cargo test`.
- **Installation**:
  ```bash
  cargo install cargo-llvm-cov
  ```

### 2. Execution & Report Generation
- **Local Coverage Check**:
  ```bash
  cargo llvm-cov
  ```
- **HTML Report Generation**:
  ```bash
  cargo llvm-cov --html
  ```
- **LCOV Export (for CI / Badge Generation)**:
  ```bash
  cargo llvm-cov --lcov --output-path lcov.info
  ```

### 3. CI Integration (`.github/workflows/test.yml`)
- Add a job or step to run `cargo-llvm-cov` on pull requests and pushes to `main`.
- Upload `lcov.info` report to Codecov / Coveralls / GitHub Actions artifacts.

### 4. Coverage Thresholds
- Enforce 100% test coverage on core library logic (`src/repo.rs`, `src/commands.rs`, `src/cli.rs`).
