# Generalize tool to notes and knowledge management instead of strictly task/issue focus

Generalize `did` CLI messaging, documentation, help descriptions, and error output to treat `.did` files as notes/documents/items rather than strictly tasks/issues.

## Before & After CLI Output Examples

### `did --help`
```
# Before
filesystem-native issue tracker with hooks for defining local software factories

# After
filesystem-native note and task tracker with hooks for defining local software factories
```

### `did add --help`
```
# Before
Create a task issue

# After
Create a note
```

## Requirements
- Update CLI descriptions, help texts, and error messages to refer to notes/items/documents rather than strictly task issues.
- Update `README.md` and `ADVANCED.md` to reflect `did` as a note & task tracker with hooks.
- Keep backwards compatibility with all subcommands, aliases, flags, and existing functionality.
- Ensure all tests pass.
