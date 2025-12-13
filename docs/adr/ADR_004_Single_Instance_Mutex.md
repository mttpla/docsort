---
Title: Guarantee single running instance using Windows named mutex
Status: accepted
Superseded by:
Date: 2025-12-12 23:36
tags:
  - adr
  - windows
  - concurrency
  - single-instance
---

# Context:

The application must avoid running multiple instances concurrently due to:
- startup race conditions
- manual execution by the user
- crash/restart scenarios

The solution must be robust, OS-native, and automatically cleaned up on process termination.

# Decision:

A Windows named mutex (Local namespace) will be used to enforce single-instance execution per user session.

The mutex will be acquired immediately at process startup; failure to acquire it will cause the application to exit silently.

# Consequences:

Positive:
- Kernel-level guarantee of exclusivity
- Automatic cleanup on crash or logout
- Correct behavior under fast user switching
- No filesystem artifacts or stale locks

Negative:
- Windows-specific implementation required

> Based on discussion: https://chatgpt.com/share/693dd8b3-6664-8007-88ea-3ffbea862e01