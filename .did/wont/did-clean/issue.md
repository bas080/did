# Rejected Proposal: `did clean` Subcommand

## Proposal
Add a `did clean` subcommand to purge state directory anomalies.

## Decision & Rationale
**WONTFIX / REJECTED**: The `.did/` state directory should not end up in an invalid state through standard CLI usage. Rather than introducing a manual `did clean` command, state anomalies will be detected by `did test` and optionally repaired via automatic autofixing (`did test --fix`).
