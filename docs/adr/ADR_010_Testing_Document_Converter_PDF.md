---
Title: Test Strategy for PDF DocumentConverter Validation
Status: accepted
Superseded by:
Date: 2025-12-21 13:40
tags:
  - adr
  - testing
  - pdf
  - llm
  - quality
---

# Context:
The chosen LLM-based document conversion strategy produces non-deterministic Markdown output, even with temperature set to zero. Traditional snapshot or byte-level comparison tests are therefore unsuitable.

Nevertheless, the system must guarantee that:
- Human-readable content is not lost
- No summarization or hallucinated content is introduced
- Downstream classification and routing decisions remain stable

To achieve this, deterministic tools (Pandoc and Rust-based PDF extractors) are used as *validation oracles*, not as production converters.

# Decision:
Testing of the `DocumentConverter` for PDF inputs will be based on **content validation**, not exact Markdown equivalence.

The following testing strategy is adopted.

## 1. Empty and Degenerate Inputs

- Empty PDF files must result in empty or near-empty Markdown
- PDFs with no extractable text must not produce fabricated content
- Converter failures must be explicit (error or empty output), never silent hallucination

## 2. Character Count Validation

- Deterministically extract reference text using Pandoc or a Rust PDF extractor
- Count Unicode characters of the reference text
- Extract plain text from the generated Markdown
- Assert that the Markdown character count is within an acceptable threshold

Example rule:

```
markdown_char_count >= reference_char_count * 0.98
```

This detects truncation, summarization, and incomplete conversions.

## 3. Sentence Presence Validation

- Split reference text into normalized sentences
- Normalize Markdown text (case, whitespace, punctuation)
- Assert that each reference sentence appears in the Markdown output

This ensures that no human-readable content is lost during conversion.

## 4. Negative Content Checks

The Markdown output must not contain:
- Meta commentary (e.g. "Here is the Markdown", "I converted the document")
- Apologies or uncertainty statements
- Content not present in the reference extraction

## 5. Behavioral Validation

Downstream rules and classification logic are executed using:
- Reference text
- LLM-generated Markdown

The resulting decisions must be identical. If decisions diverge, the conversion is considered invalid.

# Consequences:
- Tests are stable and resilient to LLM output variance
- The validation suite detects real regressions rather than formatting noise
- Deterministic tools remain isolated to the test layer
- Converter implementations can be swapped or extended without rewriting tests

> Based on discussion: https://chatgpt.com/share/69481a38-a76c-8007-b03f-9402d084cc0a

