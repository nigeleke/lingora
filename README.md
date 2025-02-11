# lingora

[![MIT License](https://img.shields.io/github/license/nigeleke/lingora?style=plastic)](https://github.com/nigeleke/lingora/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/language-Rust-blue.svg?style=plastic)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/nigeleke/lingora/acceptance.yml?style=plastic)](https://github.com/nigeleke/lingora/actions/workflows/acceptance.yml)
[![Coverage](https://img.shields.io/codecov/c/github/nigeleke/lingora?style=plastic)](https://codecov.io/gh/nigeleke/lingora)
![Version](https://img.shields.io/github/v/tag/nigeleke/lingora?style=plastic)

  [Site](https://nigeleke.github.io/lingora) \| [GitHub](https://github.com/nigeleke/lingora) \| [API](https://nigeleke.github.io/lingora/api/lingora/index.html) \| [Coverage Report](https://nigeleke.github.io/lingora/coverage/index.html)

Lingora is a free and open-source localization management program that analyses
fluent translation files highlighting discrepancies between reference and target
languages.

Lingora is designed primarily to be used as a command line tool, but also provides
a graphical user interface.

## Instructions

* To get the current options:
  ```bash
  lingora --help
  ```

* The default run looks at all ftl files in the `./i18n/` folder, looking for a `<current system locale>.ftl` for the reference file.
  Only one `<current system locale>.ftl` must exist for this command to run successfully. Output is to `stdout`.
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

* To use a config file, other than [./Lingora.toml](src/config/default_lingora.toml):
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
