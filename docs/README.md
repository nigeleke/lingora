# lingora

[![MIT License](https://img.shields.io/github/license/nigeleke/lingora?style=plastic)](https://github.com/nigeleke/lingora/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/lingora/ci.yml?style=plastic)](https://github.com/nigeleke/lingora/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/lingora?style=plastic)](https://codecov.io/gh/nigeleke/lingora)
![Version](https://img.shields.io/github/v/tag/nigeleke/lingora?style=plastic)

[Site](https://nigeleke.github.io/lingora) \| [GitHub](https://github.com/nigeleke/lingora) \| [API](https://nigeleke.github.io/lingora/api/lingora/index.html) \| [Coverage Report](https://nigeleke.github.io/lingora/coverage/index.html)

__Note: This is considered "complete, but".  It is yet to be used and tested in anger. I expect to add usability changes going forward.__

**lingora** is a free and open-source localization management program that analyses
fluent translation file for missing or redundant translations. It also supports users
of the [dioxus-i18n](https://github.com/dioxus-community/dioxus-i18n) crate.

**lingora** is designed primarily to be used as a command line tool, but also provides
a terminal user interface.

## Latest Release Downloads

cli - [linux](https://github.com/nigeleke/lingora/releases/latest/download/lingora-cli-linux-latest) \| [macos](https://github.com/nigeleke/lingora/releases/latest/download/lingora-cli-macos-latest) \| [windows](https://github.com/nigeleke/lingora/releases/latest/download/lingora-cli-windows-latest.exe) \|
tui - [linux](https://github.com/nigeleke/lingora/releases/latest/download/lingora-tui-linux-latest) \| [macos](https://github.com/nigeleke/lingora/releases/latest/download/lingora-tui-macos-latest) \| [windows](https://github.com/nigeleke/lingora/releases/latest/download/lingora-tui-windows-latest.exe)

## Terminology

A _Canonical_ locale _document_ is the master against which all other _documents_ are compared.

_Primary_ locale _documents_ are _documents_ in other locales that provide their translations 
from the _Canonical_ _document_.

_Variant_ locale _documents_ are _documents_ using the same _language root_ as a _Canonical_ or
_Primary_ _document_, but have different _regions_.

A "_locale_" _document_ is formed from multiple files within the _fluent sources_ paths (Lingora.toml or command line argument).

The _locale_ of a fluent file is determined from path naming. If the file name is a locale
(e.g. `./i18n/en-GB.ftl`, or `./i18n/errors/en-GB.ftl`) then that will be deemed the locale
of its contents. If the file name is more descriptive (e.g. `./i18n/en/en-GB/errors.ftl`),
then its locale will be deemed to be the first parent segment which represents a valid
locale according to BCP 47. In the example's case this is `en-GB`, not `en`.

__Note: A name such as `./i18n/en/en-GB/fr.ftl` will be deemed `french (fr)`, which may not be as intended.__

## Configuration

A `Lingora.toml` configuraton file is the preferred way to define the locations of 
the fluent files within the development environment. Each of the settings within the 
file can be overidden (or provided solely) by command line arguments at runtime.

The `Lingora.toml` defines:

- The _Canonical_ locale.
  
- The _Primary_ locales.
  
- The paths for _all_ fluent files to be analysed. This can be a root __folder__ (e.g. `./i18n/`)
  if all translation files are provided under one location.

If the `Lingora.toml` file exists in the current working directory then it will be used. An explicit config
file can be provided using the `--config=path/to/your-config.toml` command line argument. If no config file exists
then sensible defaults will be used (see [default_lingora.toml](./docs/default_lingora.toml)).

It is recommended that projects provide an explicit `Lingora.toml` file minimally specifying the _Canonical_ translation
file so that all other files are compared against it, rather than the locale of a user's workstation, which would
vary from user to user.

By default (i.e., no `toml` file exists, or is specified) **lingora** will look for the translation files in `./i18n/`
and it will use `<current_system_locale>.ftl` as the _Canonical_ translation file.

Command line arguments can be used to override config file settings, with `--canonical path/to/canonical_file.ftl` and
`--primary path/to/primary_file.ftl` command line arguments. The _canonical_ and _primary_ path names are expected to
use the `<language>-<locale>` or `<language>-<script>-<locale>` naming convention as described above.

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

## Github action

`lingora-cli` can be run in GitHub Actions. Example:

```yml
jobs:
  i18n:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v6

      - name: Run Lingora
        uses: nigeleke/lingora@<version> # E.g. nigeleke/lingora@v0.4.0
        with:
          args: --config=./i18n/Lingora.toml # Optional; none or any cli args can be provided here.
          working-directory: ./i18n # Optional; default "."
          version: v0.4.0 # Optional; default "latest"
```

**Note:** the default _version_ (latest) may result in a later version of `lingora-cli` running than the action version.

**Note:** the action is executed with a default system locale `en_GB.UTF-8`; this _should not_ matter the canonical locale should be defined in config settings rather than defaulted.
  
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

## Build & run

```bash
cargo build
cargo run -p lingora-cli
cargo run -p lingora-tui
```
