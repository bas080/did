# Feature Specification: Advanced Fuzzy Search & Relevance Ranking for `did query`

## Overview
Replaces simple substring matching in `did query` with fuzzy search and relevance scoring using top Rust ecosystem crates.

## Rust Ecosystem Crate Options
1. **`nucleo` / `nucleo-picker`**: High-performance, concurrent fuzzy-matching engine used in Helix editor.
2. **`fuzzy-matcher`** (Clangd / Skim algorithm): Provides Smith-Waterman based fuzzy score matching with highlighting.
3. **`skim`**: Rust implementation of fzf fuzzy finder logic.
4. **`tantivy`**: Full-text search engine library (Lucene alternative in Rust) for indexed queries.

## Decision
Integrate `fuzzy-matcher` for `did query` to provide typo-tolerant, relevance-ranked search while maintaining a lean binary.

## Recommended Approach
Integrate `fuzzy-matcher` for `did query`:
- Ranks search results by relevance score (path matches + header matches + content matches).
- Allows typo-tolerant matching (e.g. `did query authjwt` matches `backend/auth/jwt.md`).
