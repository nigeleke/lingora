
# lingora-tui

This is the **terminal user interface** for Lingora - an interactive browser for exploring Fluent translation files and their discrepancies.

## Purpose

While `lingora-cli` is great for automation and quick checks, `lingora-tui` lets you visually navigate:

- The canonical translation set
- Each target locale's translations
- Missing keys (present in canonical but absent in target)
- Redundant keys (present in target but not in canonical)
- Message contents side-by-side for easy comparison

It is intended for use during translation review and debugging of localization issues.

## Installation

```bash
cargo +nightly install lingora+tui
```
