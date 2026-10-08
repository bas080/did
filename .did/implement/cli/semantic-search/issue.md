# Feature Proposal: Semantic (Vector) Search for `did query`

## Overview
While `did query` provides keyword and fuzzy matching, it cannot find tasks based on conceptual similarity. This proposal suggests implementing a semantic search layer that uses embeddings to match queries with related concepts (e.g., "performance" matching "latency").

## Proposed Approach
Given the desire to keep the core binary lean, this should be implemented as a non-core extension:

1. **Plugin Architecture**: Use a separate binary or a plugin that `did query` can call if available.
2. **Embedding Provider**:
   - **Local**: Use a lightweight model (e.g., via `candle`) that is downloaded on demand.
   - **Remote**: Support an optional API key for providers like OpenAI.
3. **Indexing**: Implement a background process to index `.did/` task content into a vector store (e.g., a simple local FAISS index or similar).

## User Interface
- `did query <QUERY> --semantic`: Triggers the semantic search engine instead of the fuzzy matcher.
- The output should rank results by cosine similarity.

