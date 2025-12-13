---
Title: Choose Rust as primary language for a user-level Windows background agent
Status: accepted
Superseded by:
Date: 2025-12-12 23:30
tags:
  - adr
  - language
  - rust
  - windows
  - cross-platform
---

# Context:

We need to build a small but robust background application that:
- runs on Windows
- starts automatically after user login
- runs with user permissions and terminates at logout
- monitors filesystem changes
- processes documents (Word/PDF)
- is distributed as a single executable (or minimal bundle)
- is developed primarily on macOS with cross-compilation for Windows

The application is not a Windows Service and does not require elevated privileges.

# Decision:

Rust is chosen as the primary implementation language.

Rust will be used to implement:
- the filesystem watcher
- the application lifecycle
- single-instance guarantees
- startup registration
- orchestration of document conversion and analysis

# Consequences:

Positive:
- Native single-file binaries without external runtimes
- Strong cross-compilation support from macOS to Windows
- High reliability for long-running background processes
- Excellent performance for regex and text processing
- Compile-time guarantees for OS-specific code separation

Negative:
- Higher initial development complexity compared to scripting languages
- Need to explicitly manage OS-specific behavior via conditional compilation

> Based on discussion: https://chatgpt.com/share/693dd8b3-6664-8007-88ea-3ffbea862e01
