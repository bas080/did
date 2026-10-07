# Feature Specification: `did help` Command & `help` Lifecycle Hook

## Overview
Adds a `did help [TOPIC]` subcommand and `help` lifecycle hook (`.hooks/help`).
When executed, `did help` runs ancestor `help` hooks top-down, allowing projects to output custom way-of-working instructions alongside CLI documentation.

## Requirements
1. Running `did help` executes ancestor `help` hooks from `.did/` root down to the current working directory.
2. Displays CLI subcommand guides and project state directory instructions for developers and AI agents.


## Open Questions for Refinement (@bas080)
1. @bas080 Should local `help` hooks override clap CLI help output or append custom usage notes after clap help text?
