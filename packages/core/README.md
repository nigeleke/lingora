# core

Core functionality is provided through the `AuditEngine`, which will perform an audit based
on provided `LingoraSettings`.


| Module | Description | Notes |
| --- | --- | --- |
| root | Root module | |
| [config](src/config) | Interpret command line arguments and `Lingora.toml` file  | 1 |
| [domain](src/domain) | Domain logic | 2 |
| [domain/fluent](src/domain/fluent) | Fluent file handling | |
| [domain/integrity](src/domain/integrity) | Integrity checking data structures | |
| [tui](src/tui) | Terminal user interface | 3 |
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

3. The `tui` module is responsible for the graphical user interface.
   `components` contains the various components used in the TUI.
   `state.rs` is the user interaction and subseqent current selections.

4. The `output` module provides formatted output of the analysis and output of the I18nConfig function call.
