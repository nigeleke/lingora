# lingora-core

This is the **shared core library** of the Lingora localization management tool.

## Purpose

`lingora-core` analyzes Fluent translation files (`.ftl`). It defines data structures, parsing utilities, locale inference, and the comparison engine that detects discrepancies between a canonical document and target locale documents.

This crate is **not** intended for direct end-user use - it is an internal dependency shared by:

- `lingora-cli` — the command-line tool
- `lingora-tui` — the terminal user interface

## Main Components

- **Fluent document model** - parsed representation of `.ftl` files with entries, attributes, and message structure
- **Locale inference** - automatic BCP 47 locale detection from file paths/names (e.g. `fr.ftl` → `fr`, `en-GB.errors.ftl` → `en-GB`)
- **Canonical vs target comparison** - core discrepancy detection logic (missing translations, extraneous keys)
- **Primary/variant locale handling** - support for main language documents and regional variants
- **Error reporting** - structured diagnostics for translation issues

## Usage

This crate is not published separately on crates.io and is workspace-internal.

In other Lingora crates, add to `Cargo.toml`:

```toml
[dependencies]
lingora-core = { path = "../core" }
```
