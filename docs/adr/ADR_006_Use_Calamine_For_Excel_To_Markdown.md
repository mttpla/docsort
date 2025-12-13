---
Title: Use Rust `calamine` crate to convert Excel files to Markdown
Status: accepted
Superseded by: 
Date: 2025-12-13 08:45
tags:
  - rust
  - document-conversion
  - markdown
  - excel
  - tooling
  - pandoc
---

# Context:

The system requires converting Excel spreadsheets (`.xlsx`) into Markdown files as part of a document processing pipeline.  
The resulting Markdown is intended for downstream uses such as documentation, Git versioning, and LLM ingestion.

Constraints and goals:

- Avoid installing or depending on additional external software (e.g. Excel, LibreOffice, Python, JVM).
- Prefer a single, self-contained binary, similar to Pandoc’s distribution model.
- Ensure cross-platform compatibility (macOS, Linux, Windows).
- Maintain deterministic and reproducible conversions.
- Preserve data values rather than visual formatting.
- Allow full control over how spreadsheet data is mapped to Markdown semantics.

Pandoc does not support direct Excel (`.xlsx`) input. Common alternatives (Excel export, `xlsx2csv`, LibreOffice headless) introduce external runtime dependencies that conflict with the above constraints.

---

# Decision:

We decided to use the Rust crate **`calamine`** to read Excel files directly and implement a custom Excel → Markdown conversion layer.

`calamine` will be responsible solely for parsing spreadsheet files and exposing their data structures (workbooks, sheets, rows, cells).  
The application code will handle:

- Sheet iteration
- Cell value normalization
- Semantic mapping (e.g. tables, key-value sections, headings)
- Markdown generation

Pandoc is **not** used for the Excel → Markdown step, as the semantic decisions must be explicit and domain-aware rather than generic CSV-style conversions.

---

# Consequences:

**Positive:**

- No external software or runtime dependencies are required.
- The application can be distributed as a single Rust binary.
- Conversion logic is fully deterministic and testable.
- Full control over Markdown structure and semantics.
- Cross-platform behavior is consistent.
- The solution aligns with a Unix-style “do one thing well” architecture.

**Neutral / Trade-offs:**

- Conversion logic must be implemented and maintained in code.
- Visual formatting (colors, merged cells, layout) is intentionally ignored.
- Formulas are not recalculated; only stored cell values are read.

**Negative:**

- `calamine` does not evaluate Excel formulas; it relies on the last saved values.
- Advanced Excel features (macros, charts) are out of scope.

Overall, this decision prioritizes simplicity, portability, and semantic correctness over visual fidelity, which is acceptable and desirable for the intended use cases.

---

> Based on discussion: https://chatgpt.com/share/693dd869-e210-8007-9be4-e5282d813a5f
