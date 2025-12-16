---
Title: Use Pandoc as external document conversion engine
Status: superseded
Superseded by: Use AI-based document conversion for PDF and DOCX to Markdown
Date: 2025-12-12 23:32
tags:
  - documents
  - pandoc
  - markdown
  - conversion
---

# Context:

The application must convert Word (.docx) and PDF documents into Markdown for further regex-based and contextual analysis.

Native libraries in most languages provide incomplete or unreliable results, especially for complex or malformed documents.

Conversion quality and structural correctness are more important than tight language-level integration.

# Decision:

Pandoc will be used as an external executable for document conversion.

The Rust application will:
- invoke Pandoc as a subprocess
- pass input documents and conversion parameters
- consume the generated Markdown output
- treat Pandoc as a replaceable, versioned dependency

# Consequences:

Positive:
- Best-in-class conversion quality for DOCX → Markdown
- Mature handling of real-world document edge cases
- Clear separation of concerns between conversion and analysis
- Independent evolution of conversion logic

Negative:
- Pandoc is GPL licensed and must be evaluated for distribution constraints
- The application is not strictly a single binary, but a small bundle

> Based on discussion: https://chatgpt.com/share/693dd8b3-6664-8007-88ea-3ffbea862e01
