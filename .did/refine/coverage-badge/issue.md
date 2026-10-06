# Feature Specification: Code Coverage Badge in `README.md`

## Overview
Adds a dynamic code coverage status badge to the top of `README.md` displaying the project's test coverage percentage.

---

## Dependency Specification

### Hard Dependency: `code-coverage` (`cargo-llvm-cov`)
- **Prerequisite**: This feature **DEPENDS DIRECTLY** on the completion of the `code-coverage` feature (`.did/refine/code-coverage/issue.md`).
- **Data Source**: The badge value is generated from the `lcov.info` report produced by `cargo-llvm-cov --lcov --output-path lcov.info` during GitHub Actions CI workflow execution.

---

## Badge Integration Details

1. **Service**: Use Codecov or Shields.io endpoint linked to GitHub Actions workflow output.
2. **README Placement**: Positioned at the very top of `README.md` alongside build status badges.
3. **Markdown Snippet**:
   ```markdown
   [![Coverage Status](https://img.shields.io/codecov/c/github/your-org/did/main.svg)](https://codecov.io/gh/your-org/did)
   ```
