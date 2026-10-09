# Profile CLI Commands Performance

Propose performance profiling across all `did` CLI subcommands (`add`, `show`, `status`, `close`, `open`, `query`, `blocks`, `mv`, `rm`, `test`) to identify execution bottlenecks.

## Goal
Measure execution latency, memory usage, and file I/O overhead across small and large `.did/` repositories to optimize response times.

## Requirements
- Benchmark command execution times across representative `.did` state sizes.
- Identify bottleneck areas in WalkDir file traversals, hook invocations, and markdown rendering.

## Questions
- [ ] @bas080: What target execution latency threshold should we target for large repositories (e.g. <50ms for `did status`)?
