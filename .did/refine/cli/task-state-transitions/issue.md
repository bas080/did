# Feature Proposal: Streamlined Task State Transitions & Demotion

## Overview
During development, moving tasks between `.did/implement/` and `.did/refine/` (e.g. demoting tasks that need further user input or clarification) requires manual path-based operations via `did mv`.

## Proposed Solution
Introduce explicit CLI shortcuts or flags (e.g. `did demote <PATH>` or `did mv --to-refine <PATH>`) to simplify state transitions between `.did/implement/` and `.did/refine/`.

## Open Questions for Refinement (@bas080)
1. @bas080 Should task demotion be supported via a standalone 'did demote' subcommand or a flag on 'did mv'?
