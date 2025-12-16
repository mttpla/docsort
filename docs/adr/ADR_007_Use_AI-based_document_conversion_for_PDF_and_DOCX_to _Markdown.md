---
Title: Use AI-based document conversion for PDF and DOCX to Markdown
Status: proposed
Superseded by:
Date: 2025-12-16 19:02
tags:
  - adr
  - documents
  - ai
  - markdown
  - conversion
  - pdf
  - docx
---

# Context:

The application must convert heterogeneous documents (PDF and DOCX) into Markdown
for further regex-based, semantic, and contextual analysis.

A previous decision introduced Pandoc as an external conversion engine, which works
well for DOCX but does **not support PDF as an input format**.

PDF documents are a first-class input for the application and cannot be excluded or
delegated to a separate pre-processing pipeline without increasing operational
complexity.

Traditional PDF-to-Markdown conversion tools often:
- lose structural information
- flatten layout semantics
- produce low-quality or noisy Markdown
- require additional external dependencies (e.g. OCR engines)

Conversion quality and structural fidelity are more important than strict determinism
or language-level integration.

# Decision:

The application will use **AI-based document conversion** to transform PDF and DOCX
documents into Markdown.

The system will:
- provide the raw document content (or extracted text where required) to an AI model
- request a structured Markdown representation as output
- normalize and post-process the generated Markdown
- treat the AI model and prompt as replaceable, versioned components

Pandoc may remain an **optional fallback** for DOCX-only conversions but is no longer
the primary conversion engine.

# Consequences:

Positive:
- Unified conversion strategy for both PDF and DOCX
- Significantly improved handling of real-world PDFs (layout, headings, tables)
- Reduced dependency on external binaries and platform-specific tooling
- Greater flexibility to evolve conversion logic via prompt and model tuning

Negative:
- Conversion is non-deterministic by nature
- Requires cost control, rate limiting, and caching strategies
- Output quality depends on prompt design and model behavior
- Requires additional validation and post-processing steps

> Based on discussion: https://chatgpt.com/share/6941a841-312c-8007-a79e-76465bab321b
