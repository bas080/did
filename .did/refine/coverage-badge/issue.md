# Feature Specification: Static 100% Code Coverage Badge

## Overview
Adds a static 100% code coverage badge to the top of `README.md`.

## Prerequisites
- **100% Code Coverage Required**: The static 100% badge must NOT be added to `README.md` until code coverage actually reaches 100% and is enforced by CI (`cargo llvm-cov --fail-under-lines 100`).

---

## Badge Details

- **Type**: Static SVG badge.
- **Location**: Top of `README.md` alongside build status badges.
- **Markdown Snippet**:
  ```markdown
  [![Coverage Status](https://img.shields.io/badge/coverage-100%25-brightgreen.svg)](https://github.com/bas080/did)
  ```
- **Rationale**: Once 100% code coverage is achieved and enforced by CI in `.github/workflows/test.yml`, any passing commit on `main` is guaranteed to have 100% coverage, making a static 100% badge accurate and lightweight.


## Open Questions for Refinement (@bas080)
1. @bas080 Should the coverage badge be dynamically updated via GitHub Actions or kept as a static badge once 100% coverage is reached?
