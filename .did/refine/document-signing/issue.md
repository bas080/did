# Document Signing Guidance in Advanced Documentation

Propose adding a section to `ADVANCED.md` documenting cryptographic signing patterns for `.did/` task files (e.g. using GPG, Minisign, Signify, or Git commit signing) to verify issue origin and integrity in distributed teams.

## Goal
Provide examples and recommendations for signing task files directly (in file metadata/footers) or via Git commit signatures.

## Requirements
- Add a "Cryptographic Task Signing" section in `ADVANCED.md`.
- Include examples for GPG signatures (`gpg --sign`), Minisign (`minisign -S`), and Git commit signing (`git commit -S`).
- Reference established signing standards (e.g., [Minisign Specification](https://jedisct1.github.io/minisign/), [Git Tools Signing](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)).

## Questions
- [ ] @bas080: Should we recommend inline signature footers or detached `.sig` files when signing `.did/` tasks?
