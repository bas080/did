# Issue: Native Windows CI Test Runner Support

@bas080 Should we add `windows-latest` to the matrix in `.github/workflows/test.yml` after adapting test shell script execution harnesses and symlink path assertions for Windows?

## Goal
Enable native `windows-latest` job execution in `.github/workflows/test.yml` so all integration tests in `tests/cli_tests.rs` run and pass natively on Windows CI runners in addition to `ubuntu-latest`.

## Requirements
- Update shell execution test harnesses in `tests/cli_tests.rs` to handle Windows shell commands (`cmd.exe` / `powershell`).
- Ensure relative symlinks created in test fixtures behave consistently on Windows.
- Add `windows-latest` to `strategy.matrix.os` in `.github/workflows/test.yml`.
