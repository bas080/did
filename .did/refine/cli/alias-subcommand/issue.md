# Feature Specification: `did alias` Bash Aliases Subcommand

## Overview
Adds a `did alias` subcommand (or shell alias generator) that prints useful shell alias shortcuts to `stdout` to make command execution faster for developers and agents.

## Suggested Aliases
```bash
alias da='did add'
alias ds='did status'
alias dq='did search'
alias dw='did show'
alias dd='did done'
alias du='did undone'
alias dl='did link'
alias dm='did mv'
```

## Requirements
1. Running `did alias` outputs the bash alias definitions directly to `stdout`.
2. Can be sourced directly in `.bashrc` / `.zshrc` via `source <(did alias)`.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `did alias` support custom user-defined alias mappings via environment variables or strictly hardcoded defaults?
2. @bas080 Should completion scripts automatically source generated aliases?
