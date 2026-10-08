# Standardize issue templates

To ensure consistency and completeness across all issues in the `.did/` directory, a standard template should be adopted.

## Proposed Template
```markdown
# <Title>

<Detailed description of the problem or feature. Explain the "Why" and the "What".>

## Requirements
- <Requirement 1>
- <Requirement 2>

## Acceptance Criteria
- [ ] <Criterion 1>
- [ ] <Criterion 2>

## Test Plan
- [ ] <Step 1 to verify>
- [ ] <Step 2 to verify>

## Expected Output (If applicable)
### Before
```
<current output>
```
### After
```
<desired output>
```
```

## Requirements
- Define a set of required sections for every issue (e.g., Description, Requirements, Acceptance Criteria, Test Plan).
- Provide a template file or a mechanism via `did add` to pre-populate these sections.

## Acceptance Criteria
- [ ] A standard issue template is documented and agreed upon.
- [ ] `did add` optionally supports creating issues from a template.
- [ ] New issues created follow this structure.

## Test Plan
- [ ] Verify that `did add <path>` can use a template (e.g., `did add --template <name> <path>`).
- [ ] Verify that the created file contains the standard sections.
- [ ] Verify that the documentation (e.g., `did help`) reflects the template usage.
