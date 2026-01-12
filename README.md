# lingora

[![MIT License](https://img.shields.io/github/license/nigeleke/lingora?style=plastic)](https://github.com/nigeleke/lingora/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/lingora/acceptance.yml?style=plastic)](https://github.com/nigeleke/lingora/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/lingora?style=plastic)](https://codecov.io/gh/nigeleke/lingora)
![Version](https://img.shields.io/github/v/tag/nigeleke/lingora?style=plastic)

[Site](https://nigeleke.github.io/lingora) \| [GitHub](https://github.com/nigeleke/lingora) \| [API](https://nigeleke.github.io/lingora/api/lingora/index.html) \| [Coverage Report](https://nigeleke.github.io/lingora/coverage/index.html)

__Note: This is a work in progress, and is not yet ready for general use.__

**lingora** is a free and open-source localization management program that analyses
fluent translation file for missing or redundant translations. It also supports users
of the [dioxus-i18n](https://github.com/dioxus-community/dioxus-i18n) crate.

**lingora** is designed primarily to be used as a command line tool, but also provides
a terminal user interface.

linux \| macos \| windows

## Terminology

A _Canonical_ locale file the master copy against which all other files are compared.

_Primary_ locale files are the files in other languages that provide their translations 
from the _Canonical_ file.

## Configuration

A `Lingora.toml` configuraton file is the preferred way to define the locations of 
the fluent files within the development environment. Each of the settings within the 
file can be overidden (or provided solely) by command line arguments at runtime.

The file primarily defines:

- The _Canonical_ file.
  
- The _Primary_ files.
  
- The paths for _all_ fluent files to be analysed. This can be a root __folder__
  if all translation files are provided under one location.

If the `Lingora.toml` file exists in the current working directory then it will be used. An explicit config
file can be provided using the `--config=path/to/your-config.toml` command line argument. If no config file exists
then sensible defaults will be used.

It is recommended that projects provide an explicit `Lingora.toml` file minimally specifying the _Canonical__ translation
file so that all other files are compared against it, rather than the locale of a user's workstation, which would
vary from user to user.

By default (i.e., no `toml` file exists, or is specified) **lingora** will look for the translation files in `./i18n/`
and it will use `<current_system_locale>.ftl` as the _Canonical__ translation file.

Command line arguments can be used to override config file settings, with `--canonical path/to/canonical_file.ftl` and
`--primary path/to/primary_file.ftl` command line arguments. The _canonical_ and _primary_ file names are expected to
use the `<language>-<locale>` or `<language>-<script>-<locale>` naming convention.

## Dioxus

**lingora** provides additional functionality for users of the [dioxus-i18n](https://crates.io/crates/dioxus-i18n/) crate.

- automate creation a [Rust](https://rust-lang.org/) source file (`config.rs`) containing a function to
  create an [I18nConfig](https://docs.rs/dioxus-i18n/0.5.0/dioxus_i18n/use_i18n/struct.I18nConfig.html)
  struct.
 
- scan [Rust](https://rust-lang.org/) source files for their use of `dioxus_i18n::t!`, `te!` and `tid!`
  macros, ensuring, if possible, that the translation exists in the canonical file.

## Runtime

Lingora comprises two programs:

- a command-line program, intended for quick summarisations of the integrity of the translation
  and [Rust](https://rust-lang.org/) source files.
  ```bash
  lingora-cli --help
  ```

- a terminal user interface enabling browsing of the translation files and identifiers. 
  ```bash
  lingora-tui --help
  ```

## Developmemt

```bash
cargo test
cargo llvm-cov
```

| Package | Description |
| ------- | ----------- |
| common  | The main file analysis functionality, shared by the cli & tui |
| cli     | The command line interface |
| tui     | The terminal user browsing application |

## Build

```bash
# TODO!
```
