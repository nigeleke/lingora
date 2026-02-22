
# lingora-cli

This is the **command-line interface** for Lingora.

## Purpose

`lingora-cli` non-interactive checks of translation integrity for:

- Local development checks
- Pre-commit hooks
- CI/CD pipelines (see the GitHub Action)

It reports missing translations, redundant keys, and validates `dioxus-i18n` `t!`, `te!`, `tid!` macro calls in Rust files.

## Installation

```bash
cargo +nightly install lingora-cli
```
