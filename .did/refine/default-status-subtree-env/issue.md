Allow users to define an environment variable (e.g. DID_DEFAULT_SUBTREE or DID_STATUS_PATH) that specifies a default subtree under .did/ (e.g. .did/implement) when running 'did status'. This further reduces the visible items printed on status to focus attention on active areas.


## Open Questions for Refinement (@bas080)
1. @bas080 Should `DID_STATUS_PATH` take precedence over CLI path arguments or act strictly as a fallback when no path parameter is provided?
