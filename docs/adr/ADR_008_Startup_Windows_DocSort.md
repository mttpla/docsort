---
Title: Windows Startup Strategy for DocSort
Status: accepted
Superseded by: 
Date: 2025-12-20 19:05
tags:
  - windows
  - startup
  - desktop-app
  - docsort
---

# Context:

DocSort is a Rust-based desktop application designed to run continuously in background and monitor folders for document classification.

A key requirement is to start DocSort automatically at user login on Windows, running with standard user privileges, without requiring administrator rights.

Several Windows-supported startup mechanisms were evaluated:

- Registry-based startup (HKCU Run)
- Startup folder shortcut (.lnk)
- Task Scheduler

During implementation, multiple reliability and security-related issues emerged, especially related to Windows Defender and executable trust.

# Decision:

The chosen solution is to register DocSort at startup by creating a `.lnk` shortcut inside the user Startup folder:

%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup

Key reasons for this decision:

- Runs at user login with user permissions
- Does not require admin privileges
- More reliable than registry-based startup for unsigned executables
- Easier to debug and validate (visible artifact)
- Compatible with Windows Defender heuristics

The application binary is installed in a stable user-level directory:

%LOCALAPPDATA%\DocSort\docsort.exe

The `.lnk` explicitly references the absolute expanded path to the executable.

# Considered Alternatives:

## Registry (HKCU\Software\Microsoft\Windows\CurrentVersion\Run)

Rejected due to the following issues:

- Silent failures if path is invalid, quoted incorrectly, or executable is blocked
- Harder to debug (no visible UI artifact)
- Windows Defender may suppress execution without explicit error
- No easy way to validate execution outside of reboot

## Task Scheduler

Deferred (not selected for now):

- Highly reliable and configurable
- Overkill for current requirements
- Requires more complex setup and maintenance
- Higher cognitive load for end users

# Windows Defender & File Trust Considerations:

Several issues were observed when using registry-based startup:

- Executables downloaded from the internet may be marked with Mark of the Web (Zone.Identifier)
- Unsigned binaries triggered Defender heuristics during auto-start
- Startup execution was blocked silently or terminated immediately

Using a `.lnk` in the Startup folder significantly reduced these issues:

- Defender is more permissive with Startup shortcuts
- Users can manually inspect, unblock, or delete the shortcut
- Improves transparency and trust

When necessary, the executable can be manually unblocked via file properties.

# Consequences:

## Positive:

- Reliable startup behavior across Windows versions
- Clear and inspectable startup mechanism
- Better alignment with Windows security expectations
- Simplified troubleshooting and support

## Negative:

- Requires managing `.lnk` creation programmatically
- Slightly more implementation effort than registry write

> Based on discussion: https://chatgpt.com/share/69472943-5ccc-8007-8cad-94576d1aced5
