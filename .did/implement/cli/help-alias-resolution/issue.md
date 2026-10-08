# Feature Proposal: Alias Resolution in `did help <ALIAS>`

## Overview
Currently, running `did help link` or `did help query` displays help for the target subcommand. However, running `did help ln` or `did help search` or `did help move` returns an error:
```
error: unrecognized help topic or subcommand 'ln'
```

## Proposed Change
In `cmd_help`, before searching subcommands for a help topic, resolve known subcommand aliases (`ln` -> `link`, `search` -> `query`, `move` -> `mv`, `remove` -> `rm`). This allows users and agents invoking help via command aliases to receive the correct subcommand help output seamlessly.

@bas080
