# Agent Instructions & Established Way of Working

Welcome! This repository uses `did`, a filesystem-native task and dependency tracker written in Rust.

---

## Established Way of Working

### 1. Deep Planning Mode & Requirements Clarification
- **Zero Doubt Requirement**: Before writing code or modifying plans, perform exploratory research and ask clarifying questions to test every assumption.
- **Iterative Refinement**: Formulate precise questions to ensure 100% certainty on user expectations. Update plans via `set_plan` once requirements are approved.

### 2. Filesystem-Native Issue Lifecycle (`.did/`)
- **Directory Hierarchy Standard**: Directly under `.did/`, ONLY `implement`, `refine`, and `wont` (plus reserved `.hooks`) exist. Sub-categories (e.g. `github-actions`, `features`, `tasks`) are placed inside `implement/` or `refine/`.
- **Staging in `.did/refine/`**: New, unrefined, or open proposal issues stay in `.did/refine/`.
- **Promotion to `.did/implement/`**: Move issues to `.did/implement/` only when they are **100% ready**, with explicit requirements, edge cases, and concrete test plans.
- **Demotion Back to `.did/refine/` on Vagueness**: When evaluating `did status implement`, if any issue contains vagueness, ambiguity, or open questions, **move it back to `.did/refine/`** and append explicit clarifying questions to the issue document.
- **Documenting Rejected Proposals in `.did/wont/`**: When a feature proposal, command idea, or design decision is rejected, document it inside `.did/wont/<PROPOSAL>/issue.md` explaining the decision and rationale for not adopting it.
- **Discovering Bugs/Features**: Whenever you discover a bug, issue, or needed feature while working, create a `did` issue for it:
  ```bash
  did add refine/describe-feature/issue.md -m "Detailed issue description"
  ```

### 3. Hook Directory Sanctity (`.hooks/`)
- `.hooks/` directories inside `.did/` subtrees are strictly reserved for hook files (`show`, `status`, `done`, `add`, `link`, `mv`, or with extensions like `show.sh`, `show.md`).
- **Never** place issue/task files directly inside `.hooks/`. Place issue files describing hook features in a separate directory (e.g., `dot-hooks/`).
- Hook files starting with `.hooks/` are hidden in status listings and do not act as blocking sub-items for sibling tasks.

### 4. Testing, Quality Control, and Coverage
- **Automated Integration Tests**: Every code modification or feature must be accompanied by tests in `tests/cli_tests.rs`.
- **Verification Commands**:
  - Run all tests: `cargo test`
  - Linting: `cargo clippy -- -D warnings`
  - Code coverage check: `cargo llvm-cov --fail-under-lines 76`

### 5. `did` CLI Core Usage Commands
- **Check Actionable Tasks**: `did status`
- **Check All Tasks (including blocked/closed)**: `did status -a`
- **Inspect Task Content**: `did show <PATH>`
- **Link Dependencies**: `did link <TARGET> <DEST_DIR>`
- **Complete Task**: `did done <PATH>` (renames to `.task` and updates inbound symlinks)
- **Telemetry Logging**: Activate XML execution logging by setting `DID_LOG_PATH='./target/did.log'`.

### 6. CI & Release Conventions
- **GitHub Actions Workflows**:
  - `test.yml`: Runs tests, clippy, and `cargo-llvm-cov` checks on all branch pushes and PRs.
  - `release-linux.yml`: Pushes to `main`/`master` update the `development` binary release asset. Version tag pushes (`v*`) update the `latest` version release asset.
