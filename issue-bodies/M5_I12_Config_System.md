### Summary

This issue covers the creation of a persistent, user-editable configuration system. The Rust backend will manage loading/saving settings to a file, and the Svelte frontend will provide a UI panel for users to modify these settings easily.

### Breakdown of Tasks

-   **[ ] Create Configuration Module in Rust:**

    -   Create a new module `src-tauri/src/modules/configuration.rs`.

    -   Define a `Settings` struct that is serializable with `serde`. It should include fields for:

        -   `llm_provider` (e.g., "gemini", "openai").

        -   `permission_options` (the struct from the Permission module).

        -   `global_hotkey_string`.

    -   Implement functions `load_settings() -> Settings` and `save_settings(settings: &Settings)` that read from and write to a JSON or TOML file in a standard config directory (e.g., `~/.config/nyx-agent/settings.json`).

-   **[ ] Expose Settings to Tauri:**

    -   Create Tauri commands `get_settings() -> Settings` and `update_settings(new_settings: Settings)`.

    -   The `update_settings` command will save the new settings to the file.

    -   Ensure API keys are NOT exposed through these commands; they should be managed separately and securely via the `keyring` module.

-   **[ ] Develop Svelte Settings Panel:**

    -   Create a new Svelte component for the settings UI (`SettingsPanel.svelte`).

    -   On mount, the component will call `get_settings` to populate its state.

    -   Create UI elements (dropdowns, checkboxes, text inputs) to manage:

        -   LLM Provider selection.

        -   Permission settings (YOLO mode, etc.).

        -   Global hotkey configuration.

    -   Add a "Save" button that calls the `update_settings` Tauri command.

