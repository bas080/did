# Implementation Workflow Guidelines (.did/implement)

When picking up an issue or feature to implement, adhere strictly to the following instructions:

1. **Clarity of Requirements**:
   - Only proceed with implementation if the requirements for the issue/feature are crystal clear and unambiguous.
   - If requirements are unclear, vague, or missing details:
     - Ask clarifying questions to resolve assumptions, or
     - Move the issue to the `.did/refine/` directory until requirements are fully explored and specified.

2. **Testing & Quality Assurance**:
   - Any feature change or bug fix MUST be accompanied by unit or integration tests verifying the behavior.
   - Prefer a Test-Driven Development (TDD) approach: write a failing test first to demonstrate the missing feature or bug, then write the minimal code necessary to make the test pass.
