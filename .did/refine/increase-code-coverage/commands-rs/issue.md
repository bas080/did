# Sub-Issue: `src/commands.rs` 100% Coverage

## Target
Increase line coverage for `src/commands.rs` from **76.09%** to **100%**.

## Uncovered Statements to Address
- Error handling paths in `cmd_add` when `$EDITOR` exits with failure or writes fail.
- Error handling in `cmd_link` when destination directory creation fails or target is invalid.
- Non-zero exit status reporting in `cmd_show` when executable hooks exit with error.
- Error paths in `cmd_autocomplete` for invalid shell name string parameters.
