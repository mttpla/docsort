---
Title: Use LLM as Primary Document-to-Markdown Converter
Status: accepted
Superseded by:
Date: 2025-12-21 13:40
tags:
  - adr
  - document-conversion
  - llm
  - architecture
---

# Context:
The DocSort project requires converting heterogeneous document formats (initially PDF and DOC/DOCX) into Markdown as an intermediate representation for downstream processing (classification, rules evaluation, and optional LLM-based reasoning).

Multiple technical approaches were evaluated:
- Pure Rust-based extraction using PDF/DOC crates
- Marker (Python-based document parser)
- Pandoc (deterministic document converter)
- Direct LLM-based document-to-Markdown conversion

Key constraints and goals:
- Minimize operational and packaging complexity (desktop application)
- Preserve future flexibility as LLM document understanding improves
- Avoid premature optimization and heavy parsing logic in the core system
- Maintain testability and architectural boundaries via a DocumentConverter abstraction

# Decision:
The system will adopt a **pure LLM-based approach** as the primary strategy for converting documents (PDF, DOC/DOCX) into Markdown.

A `DocumentConverter` abstraction will be introduced, allowing multiple implementations, but the initial and default implementation will be:

```
DocumentConverter -> LLM-based converter
```

Alternative converters (Marker, Pandoc, Rust-based extraction) are intentionally *not* used in production conversion at this stage, but remain available as:
- Deterministic reference implementations for testing
- Potential future fallbacks if quality requirements change

## Evaluated Alternatives

### Rust-based parsing (PDF/DOC crates)
**Pros:**
- Single binary
- Very fast
- Full control

**Cons:**
- Text-only extraction
- Weak layout and table reconstruction
- Significant heuristic and maintenance burden

### Marker (Python-based)
**Pros:**
- High-quality structural reconstruction
- Excellent for complex PDFs

**Cons:**
- Python runtime and native dependencies
- Complex packaging for desktop distribution
- Operational overhead

### Pandoc
**Pros:**
- Deterministic
- Mature and stable
- Excellent for DOCX

**Cons:**
- Limited PDF layout fidelity
- Not future-facing for multimodal understanding

### LLM-based conversion (Selected)
**Pros:**
- Minimal code and dependency surface
- Rapid time-to-value
- Models continuously improve without code changes
- Multimodal understanding (OCR, layout, semantics)
- Clean separation of concerns

**Cons:**
- Non-deterministic output
- Requires guard-rails and contract-based validation

The decision favors architectural simplicity and long-term adaptability over deterministic precision.

# Consequences:
- Markdown output is treated as a *best-effort artifact*, not a canonical source of truth
- Strong emphasis is placed on guard-rails and validation rather than exact output matching
- Deterministic tools (Pandoc, Rust extractors) are used exclusively for testing and validation
- The DocumentConverter abstraction allows seamless future introduction of hybrid or fallback strategies

> Based on discussion: https://chatgpt.com/share/69481a38-a76c-8007-b03f-9402d084cc0a

