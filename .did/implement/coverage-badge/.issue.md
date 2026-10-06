# Feature Specification: Static 100% Code Coverage Badge

## Overview
Adds a static 100% code coverage badge to the top of `README.md`. Because CI workflows will eventually enforce a strict 100% coverage gate (`cargo llvm-cov --fail-under-lines 100`), a static badge accurately reflects the repository's required quality standard once 100% coverage is achieved.

---

## Badge Details

- **Type**: Static SVG badge.
- **Location**: Top of `README.md` alongside build status badges.
- **Markdown Snippet**:
  ```markdown
  [![Coverage Status](https://img.shields.io/badge/coverage-100%25-brightgreen.svg)](https://github.com/your-org/did)
  ```
- **Rationale**: Once 100% code coverage is enforced by CI in `.github/workflows/test.yml`, any passing commit on `main` is guaranteed to have 100% coverage, making a static 100% badge accurate and lightweight.
