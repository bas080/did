Active enforcement in 'did link' to detect dependency graph cycles (e.g. A -> B and B -> A) and reject link creation with an error to prevent infinite traversal loops.


## Open Questions for Refinement (@bas080)
1. @bas080 Should cycle detection in `did link` abort link creation with exit code 1 or log a warning notice?
