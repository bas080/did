# Refinement Process Guidelines (`.did/refine`)

The `.did/refine/` directory is the staging area for new, vague, or incomplete issues and feature proposals. Before an issue can be placed on the roadmap or picked up for implementation (`.did/implement/`), it must go through a structured refinement process to ensure requirements are crystal clear.

---

## The Issue Refinement Process

Every issue in `.did/refine/` must be systematically expanded until it satisfies all items on the **Refinement Checklist**:

### 1. Clear Problem Statement
- What is the exact user pain point or missing capability?
- Why is this feature or bug fix needed?

### 2. Explicit Functional Requirements
- **CLI Interface**: Exact command syntax, subcommand name, arguments, and flags.
- **Environment Variables**: Names of environment variables (e.g. `DID_STATUS_LIMIT`), defaults, and override rules.
- **Output Specifications**: Exact formatting for `stdout` and `stderr` outputs.
- **Exit Codes**: Explicit exit codes (`0` for success, non-zero for failures/blocked tasks).

### 3. Edge Cases & Failure Modes
- How should the feature behave when paths do not exist, target files are directories, or symlinks are broken?
- How are conflicts, duplicates, or empty result sets handled?

### 4. Dependency & Blocking Analysis
- Does this task depend on other tasks or features (e.g., status limit depending on search)?
- How does it fit into the AST task dependency tree without creating artificial bottlenecks or circular links?

### 5. Concrete Test Cases
- What specific integration and unit test scenarios are required to verify the implementation?

---

## Transitioning Refined Issues

- **Ready for Implementation**: Once an issue meets the Refinement Checklist, move it to `.did/implement/` (or its designated domain folder e.g., `.did/features/` or `.did/issues/`).
- **Roadmap / MoSCoW Prioritization**: Categorize refined issues into MoSCoW priority directories:
  - `.did/must/` (Must Have)
  - `.did/should/` (Should Have)
  - `.did/could/` (Could Have)
