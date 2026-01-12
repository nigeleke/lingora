- To list help:

    ```bash
    lingora-cli --help
    ```

- The default run looks at all ftl files in the `./i18n/` folder, looking for a `<current_system_locale>.ftl` for the canonical file.
  Only one `<current_system_locale>.ftl` must exist for this command to run successfully. Output is to `stdout`.

    ```bash
    lingora-cli
    # or, by redirecting stdout:
    lingora-cli > i18n-errors.txt
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

- To run the desktop application, use the `--output=tui` option:
    ```bash
    lingora -o tui
    ```
