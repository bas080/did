Allow defining a date in a directory that hides issues in that directory until the current time is after that date. This helps limit the number of actionable items printed on 'did status'.


## Open Questions for Refinement (@bas080)
1. @bas080 Should the hide date be declared in a directory metadata file (e.g. `.hide-until`) or in task frontmatter headers?
2. @bas080 Should `did status -a` display hidden date-restricted tasks with a timestamp notice?
