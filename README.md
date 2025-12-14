# DocSort

DocSort is a lightweight, user-level background agent that automatically **classifies documents and moves them into the correct folder**.

It runs after user login, watches one or more directories, converts documents to Markdown, analyzes their content, and applies classification rules to decide where each document belongs.

DocSort is designed to be:
- invisible (no UI, no tray)
- robust
- easy to install
- easy to update
- safe (runs with user permissions only)

---

## What DocSort Does

- Starts automatically after user login (Windows)
- Watches configured folders for new or modified files
- Converts Word and PDF documents to Markdown
- Analyzes content using regex and contextual rules
- Classifies documents
- Moves or copies them to the appropriate destination folders
- Ensures only one instance runs per user session

---

## What DocSort Is *Not*

- Not a Windows Service
- Not a GUI application
- Not a cloud service
- Not an AI SaaS

DocSort is a **local automation tool**.

---

## Architecture Overview

High-level pipeline:

```
User login
   ↓
DocSort starts
   ↓
Filesystem watcher
   ↓
Document detected
   ↓
Pandoc → Markdown
   ↓
Classification logic
   ↓
File routed to destination
```

---

## Installation (Windows)

DocSort is a user-level background agent.

### Install auto-start

```bash
docsort.exe install
```

This registers DocSort in:

```
HKCU\Software\Microsoft\Windows\CurrentVersion\Run
```

DocSort will start automatically after login.

### Uninstall auto-start

```bash
docsort.exe uninstall
```

---

## Running Manually (for debugging)

```bash
docsort.exe run
```

---

## Single Instance Guarantee

DocSort uses a **Windows named mutex** to ensure that only one instance runs per user session.

If another instance is already running, the new one exits immediately and silently.

---

## Development

### Requirements

- Rust (stable)
- run `cargo install cargo-release --version 0.25.15`

```bash
cargo run
```

---

## Running Tests

Most tests are OS-agnostic and can be run on macOS.

```bash
cargo test
```

---

## Building on macOS

### Build for macOS (local)

```bash
cargo build --release
```

Binary:
```
target/release/docsort
```

---

### Cross-compile for Windows from macOS

Install Windows target:
```bash
rustup target add x86_64-pc-windows-msvc
```

Build:
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

Result:
```
target/x86_64-pc-windows-msvc/release/docsort.exe
```

> Note: Final validation and signing should be done on Windows or via CI.

---


## Dependencies

### Pandoc (Required)

DocSort relies on **Pandoc** for document conversion.

Pandoc is used as an external executable and must be available on the system.

#### What Pandoc Is Used For
- DOCX → Markdown conversion
- PDF → text/Markdown extraction (best effort)
  

## Why DocSort Exists

Because manually sorting documents is boring, error-prone, and exactly the kind of task computers should do quietly.

DocSort is designed to work, stay out of the way, and never ask for attention.
