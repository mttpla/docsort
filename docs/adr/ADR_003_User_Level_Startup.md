---
Title: Implement user-level auto-start via Windows Run registry key
Status: accepted
Superseded by:
Date: 2025-12-12 23:34
tags:
  - adr
  - windows
  - startup
  - user-session
---

# Context:

The application must:
- start after user login
- stop automatically at user logout
- run with standard user permissions
- not require administrator rights

A Windows Service is unnecessary and would introduce complexity and permission issues.

# Decision:

The application will register itself in the Windows Run registry key:

HKCU\Software\Microsoft\Windows\CurrentVersion\Run

Registration and removal will be handled via explicit application commands (install/uninstall).

# Consequences:

Positive:
- No administrator privileges required
- Lifecycle naturally bound to the user session
- Behavior consistent with common desktop background agents

Negative:
- Application does not run before login
- Startup behavior is user-profile specific

> Based on discussion: https://chatgpt.com/share/693dd8b3-6664-8007-88ea-3ffbea862e01
