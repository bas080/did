# Sub-Issue: `src/repo.rs` 100% Coverage

## Target
Increase line coverage for `src/repo.rs` from **90.00%** to **100%**.

## Uncovered Statements to Address
- `Repo::find` directory traversal termination when reaching the root `/` directory.
- `relative_display_path` when path is not a child of `did_dir`.
