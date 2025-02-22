# lingora

[![MIT License](https://img.shields.io/github/license/nigeleke/lingora?style=plastic)](https://github.com/nigeleke/lingora/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/lingora/acceptance.yml?style=plastic)](https://github.com/nigeleke/lingora/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/lingora?style=plastic)](https://codecov.io/gh/nigeleke/lingora)
![Version](https://img.shields.io/github/v/tag/nigeleke/lingora?style=plastic)

  [Site](https://nigeleke.github.io/lingora) \| [GitHub](https://github.com/nigeleke/lingora) \| [API](https://nigeleke.github.io/lingora/api/lingora/index.html) \| [Coverage Report](https://nigeleke.github.io/lingora/coverage/index.html)

__lingora__ is a free and open-source localization management program that analyses
fluent translation files highlighting discrepancies between reference and target
language files.

__lingora__ is designed primarily to be used as a command line tool, but also provides
a graphical user interface.

  Linux: deb rpm AppImage \| MacOs app dmg \| Windows exe msi

## Operation

__lingora__ compares the entries of a _reference_ translation file against one or more _target_ translation files.

A `Lingora.toml` file can be used to define the _reference_ translation file, and _target_ translation files (or search
paths).

If the `Lingora.toml` file exists in the current working directory then it will be used. An explicit config
file can be provided using the `--config=path/to/your-config.toml` command line argument. If no config file exists
then sensible defaults will be used.

It is recommended that projects provide an explicit `Lingora.toml` file minimally specifying the _reference_ translation
file so that all other files are compared against it, rather than the locale of a user's workstation, which would
vary from user to user.

By default (i.e., no `toml` file exists, or is specified) __lingora__ will look for the translation files in `./i18n/`
and it will use `<current_system_locale>.ftl` as the _reference_ translation file.

Command line arguments can be used to override config file settings, with `-r path/to/reference_file.ftl` and
`-t path/to/target_file.ftl` command line arguments. `-t` may also specify a folder, in which case all `*.ftl`
files under that folder will be used as _targets_. The _reference_ and _target_ file names are expected to use the `<language>-<locale>`
naming convention.

## Additional functionality

* __lingora__ can create a `config.rs` source file containing a function to create an [I18nConfig](https://docs.rs/dioxus-i18n/0.4.2/dioxus_i18n/use_i18n/struct.I18nConfig.html)
  for the [dioxus-i18n](https://crates.io/crates/dioxus-i18n/) crate.

* A __GUI__ interface, to browse files can be invoked with the `--output=gui` or `-o gui` command line argument.

## Command line arguments

* To list help:
  ```bash
  lingora --help
  ```

* The default run looks at all ftl files in the `./i18n/` folder, looking for a `<current_system_locale>.ftl` for the reference file.
  Only one `<current_system_locale>.ftl` must exist for this command to run successfully. Output is to `stdout`.
  ```bash
  lingora
  # or, by redirecting stdout:
  lingora > i18n-errors.txt
  ```

* To get the result of an analysis, but not see the details use the `--output=silent` option.  This command does not output anything
  unless there are inconsistencies in or between the reference and/or target folders, in which case an error return of
  `Error: IntegrityErrorsDetected` is displayed.
  ```bash
  lingora -o silent
  ```

* To specify the reference locale: note this looks for target ftl files in the `./i18n/` folder.
  ```bash
  lingora -r ./i18n/en/en-GB.ftl
  ```

* To specify reference and target(s): note `-t ...` can be a folder, in which case a deep search for all `.ftl` files within the folder is performed.
  ```bash
  lingora -r ./i18n/en/en-GB.ftl -t ./i18n/en/en-AU.ftl -t ./i18n/it/it.ftl
  ```

* To output an opininated I18nConfig initialisation function: Output is to the provide path.
  The output settings can be modified in the [Lingora.toml](src/config/default_lingora.toml) configuration file.
  ```bash
  lingora --dioxus-i18n=path/to/your_i18n_config.rs
  ```

* To use a config file, other than [./Lingora.toml](src/config/default_lingora.toml). Note, if `Lingora.toml` exists
  in the current working directory, then it will be used by default without specifying explicitly in this
  command line argument.
  ```bash
  lingora --config=path/to/your_config.toml
  ```

* The override the default root folder (`./i18n/`). Note any default root folder is
  not used if any targets (`--target=...` / `-t ...`) are provided.
  ```bash
  lingora --root=path/to/your_root_folder
  ```

* To run the desktop application, use the `--output=gui` option:
  ```bash
  lingora -o gui
  ```

## Developmemt

```bash
cargo test
cargo llvm-cov
```

## Build

```bash
cargo binstall dioxus-cli
dx bundle
```
