# Sovalune Vector Memory

Vector memory subsystem with Context Weaver for Sovalune AI agents.

## Overview

This crate provides:
- `VectorMemoryStore` - Storage and retrieval of vector embeddings
- `Context Weaver` - Builds context for model prompts based on memory

## Features

- Semantic search using pgvector
- Memory tiers: raw, consolidated, verified
- Confidence scoring and decay mechanism
- Token-budget aware context building

## Usage

```rust
use sovalune_vector_memory::{VectorMemoryStore, ContextWeaver, MemoryFilter};

// Create store
let store = VectorMemoryStore::new(pool);

// Insert memory
let id = store.insert_raw(RawMemoryEntry {
    content: "User prefers dark mode".to_string(),
    embedding: vec![0.1, 0.2, ...],
    metadata: serde_json::json!({"source": "chat"}),
    project_id: project_id,
}).await?;

// Search
let results = store.search(&query_embedding, filter, 10).await?;

// Build context
let weaver = ContextWeaver::new(pool, 1000, 4096);
let context = weaver.build_context(query, embedding, project_id, &history).await?;
```
