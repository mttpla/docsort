---
Title: Isolate Windows-specific logic using conditional compilation
Status: accepted
Superseded by:
Date: 2025-12-12 23:38
tags:
  - adr
  - rust
  - architecture
  - conditional-compilation
---

# Context:

Development is performed on macOS, while the target runtime environment is Windows.

Some functionality (registry access, startup, mutex handling) is Windows-specific and cannot be compiled or executed on macOS.

# Decision:

All OS-specific code will be isolated using Rust conditional compilation (cfg(windows)).

A platform-agnostic interface will be exposed to the core application logic, with:
- Windows implementations for production
- no-op or mock implementations for non-Windows platforms and tests

# Consequences:

Positive:
- Clean separation between core logic and OS-specific concerns
- Ability to compile and test most of the application on macOS
- Compile-time safety for unsupported platforms

Negative:
- Slight increase in architectural boilerplate
- Need for CI-based Windows integration tests

> Based on discussion: https://chatgpt.com/share/693dd8b3-6664-8007-88ea-3ffbea862e01
