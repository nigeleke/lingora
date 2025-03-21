# lingora

[![MIT License](https://img.shields.io/github/license/nigeleke/lingora?style=plastic)](https://github.com/nigeleke/lingora/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/lingora/acceptance.yml?style=plastic)](https://github.com/nigeleke/lingora/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/lingora?style=plastic)](https://codecov.io/gh/nigeleke/lingora)
![Version](https://img.shields.io/github/v/tag/nigeleke/lingora?style=plastic)

[Site](https://nigeleke.github.io/lingora) \| [GitHub](https://github.com/nigeleke/lingora) \| [API](https://nigeleke.github.io/lingora/api/lingora/index.html) \| [Coverage Report](https://nigeleke.github.io/lingora/coverage/index.html)

__Note: This is a work in progress, and is not yet ready for general use.__

_Despite the test suite, not all functionality has been tested, and there are likely to be bugs._

**lingora** is a free and open-source localization management program that analyses
fluent translation files highlighting discrepancies between reference and target
language files.

**lingora** is designed primarily to be used as a command line tool, but also provides
a graphical user interface.

linux \| macos \| windows

## Status

- Packaged builds have not been tested; links are provided in the latest github build, so
  check the latest `Build Dioxus Desktop App` action for those, or (preferably at the moment)
  build from source.

- There will be ongoing development, particularly in cross-checks between reference and target files.

- Please report any issues or feature requests on the [GitHub Issues](https://github.com/nigeleke/lingora/issues)

## Operation

**lingora** compares the entries of a _reference_ translation file against one or more _target_ translation files.

A `Lingora.toml` file can be used to define the _reference_ translation file, and _target_ translation files (or search
paths).

If the `Lingora.toml` file exists in the current working directory then it will be used. An explicit config
file can be provided using the `--config=path/to/your-config.toml` command line argument. If no config file exists
then sensible defaults will be used.

It is recommended that projects provide an explicit `Lingora.toml` file minimally specifying the _reference_ translation
file so that all other files are compared against it, rather than the locale of a user's workstation, which would
vary from user to user.

By default (i.e., no `toml` file exists, or is specified) **lingora** will look for the translation files in `./i18n/`
and it will use `<current_system_locale>.ftl` as the _reference_ translation file.

Command line arguments can be used to override config file settings, with `-r path/to/reference_file.ftl` and
`-t path/to/target_file.ftl` command line arguments. `-t` may also specify a folder, in which case all `*.ftl`
files under that folder will be used as _targets_. The _reference_ and _target_ file names are expected to use the `<language>-<locale>`
naming convention.

## Additional functionality

- **lingora** can create a `config.rs` source file containing a function to create an [I18nConfig](https://docs.rs/dioxus-i18n/0.4.2/dioxus_i18n/use_i18n/struct.I18nConfig.html)
  for the [dioxus-i18n](https://crates.io/crates/dioxus-i18n/) crate.

- A **GUI** interface, to browse files can be invoked with the `--output=gui` or `-o gui` command line argument.

## Command line arguments

- To list help:

    ```bash
    lingora --help
    ```

- The default run looks at all ftl files in the `./i18n/` folder, looking for a `<current_system_locale>.ftl` for the reference file.
  Only one `<current_system_locale>.ftl` must exist for this command to run successfully. Output is to `stdout`.

    ```bash
    lingora
    # or, by redirecting stdout:
    lingora > i18n-errors.txt
    ```

- To get the result of an analysis, but not see the details use the `--output=silent` option. This command does not output anything
  unless there are inconsistencies in or between the reference and/or target folders, in which case an error return of
  `Error: IntegrityErrorsDetected` is displayed.

    ```bash
    lingora -o silent
    ```

- To specify the reference locale: note this looks for target ftl files in the `./i18n/` folder.

    ```bash
    lingora -r ./i18n/en/en-GB.ftl
    ```

- To specify reference and target(s): note `-t ...` can be a folder, in which case a deep search for all `.ftl` files within the folder is performed.

    ```bash
    lingora -r ./i18n/en/en-GB.ftl -t ./i18n/en/en-AU.ftl -t ./i18n/it/it.ftl
    ```

- To output an opininated I18nConfig initialisation function: Output is to the provide path.
  The output settings can be modified in the [Lingora.toml](src/config/default_lingora.toml) configuration file.

    ```bash
    lingora --dioxus-i18n=path/to/your_i18n_config.rs
    ```

- To use a config file, other than [./Lingora.toml](src/config/default_lingora.toml). Note, if `Lingora.toml` exists
  in the current working directory, then it will be used by default without specifying explicitly in this
  command line argument.

    ```bash
    lingora --config=path/to/your_config.toml
    ```

- The override the default root folder (`./i18n/`). Note any default root folder is
  not used if any targets (`--target=...` / `-t ...`) are provided.

    ```bash
    lingora --root=path/to/your_root_folder
    ```

- To run the desktop application, use the `--output=gui` option:
    ```bash
    lingora -o gui
    ```

## Developmemt

```bash
cargo test
cargo llvm-cov
```

| Module | Description | Notes |
| --- | --- | --- |
| root | Root module | |
| [config](src/config) | Interpret command line arguments and `Lingora.toml` file  | 1 |
| [domain](src/domain) | Domain logic | 2 |
| [domain/fluent](src/domain/fluent) | Fluent file handling | |
| [domain/integrity](src/domain/integrity) | Integrity checking data structures | |
| [gui](src/gui) | Graphical user interface | 3 |
| [output](src/output) | Output handling | 4 |

1. The `config` module is responsible for interpreting the command line arguments and the `Lingora.toml` file.
   The `Lingora.toml` file can be in the current working directory, or specified with the `--config=path/to/your-config.toml` argument.
   `arguments.rs` is the structure populated by clap.
   `interim_settings.rs` is built from the `Lingora.toml` config file sourced from:
     - the --config=path/to/your-config.toml argument, or, if not provided.
     - the `Lingora.toml` file in the current working directory, or, if not provided.
     - default settings, which are described in the [default_lingora.toml](src/config/default_lingora.toml) file.
   `settings.rs` is the final settings used by the program, which are built from `interim_settings.rs` and `arguments.rs`.

2. The `domain` module is responsible for the core logic of the program.
   It compares the reference and target files, and creates the resultant integrity checks.
   `analysis.rs` is the resultant analysis of all files, as formed from their respective integrity checks and cross-checks.

3. The `gui` module is responsible for the graphical user interface.
   `components` contains the various components used in the GUI.
   `state.rs` is the user interaction and subseqent current selections.

4. The `output` module provides formatted output of the analysis and output of the I18nConfig function call.

## Build

```bash
cargo binstall dioxus-cli
dx build --release
# dx bundle // Still in development
```
