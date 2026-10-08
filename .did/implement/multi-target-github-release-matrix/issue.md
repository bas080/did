Refine the GitHub Actions release workflow matrix to define specific runner OSes, Rust target triples (Linux x86_64, Windows MSVC, macOS Intel/ARM, iOS, Android), and asset naming conventions.


## Decision
Multi-target release builds will publish single binary artifacts whenever possible for the target system (e.g., standalone binaries for Linux, macOS, and Windows). For systems where standalone binaries are not the standard or are impractical, appropriate archives will be used.

The initial release matrix will prioritize the following three popular targets:
- **Linux**: x86_64
- **macOS**: Intel/ARM (Universal)
- **Windows**: MSVC x86_64
