Allow configuring an env var where newly added items are added. Otherwise they are added to the .did dir. The filename will be based on the contents capped to a certain length.


## Open Questions for Refinement (@bas080)
1. @bas080 Should the default fallback directory be `.did/` or a configurable environment variable such as `DID_ADD_PATH`?
2. @bas080 How should content slugification cap the generated filename length (e.g. 30 characters maximum)?
