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

## Development

### Requirements

- Rust (stable)
  - install Rust using `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- run `git config core.hooksPath scripts/git-hooks`
- run `chmod +x scripts/git-hooks/*`
- run `cargo install cargo-release --version 0.25.15`

## Running Manually (for debugging)

```bash
cargo run
```

## Running Tests

```bash
cargo test
```

## Make a new release

Use cargo-release:

```bash
cargo release patch
cargo release minor
cargo release major
```

By default the command runs in dry-run mode; add --execute --no-publish to perform the release.

This will:
- Update the version in Cargo.toml
- Create a release commit
- Create a Git tag (e.g. v0.3.0)


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
