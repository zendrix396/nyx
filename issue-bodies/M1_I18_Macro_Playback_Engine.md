### Summary

This issue covers implementing the macro playback functionality that reads saved macro files and executes the recorded events in sequence. This complements the macro recording engine (Issue #4) to provide a complete macro system.

### Breakdown of Tasks

-   **[ ] Define Macro Deserialization:**

    -   In the `macro_engine.rs` module, create a function `load_macro(name: &str) -> Result<Macro, Error>`.

    -   This function will read the JSON file from the macros directory (e.g., `~/.config/nyx-agent/macros/[name].json`).

    -   Deserialize the JSON into a `Macro` struct using `serde_json`.

    -   Return an error if the file doesn't exist or contains invalid data.

-   **[ ] Implement Playback Logic:**

    -   Create a function `play_macro(macro_data: &Macro) -> Result<(), Error>`.

    -   This function should:

        1.  Iterate through the `TimedEvent`s in the macro's event vector.

        2.  For each event, wait for the `time_since_previous` duration using `std::thread::sleep`.

        3.  Use the I/O Controller module to execute the event via `send_event(&event.event)`.

        4.  Handle errors gracefully (log and continue, or abort on critical failures).

-   **[ ] Create Tauri Command:**

    -   Create a Tauri command `play_macro(name: String)` that:

        1.  Calls `load_macro` to retrieve the saved macro.

        2.  Calls `play_macro` to execute it.

        3.  Returns a success or error message to the frontend.

-   **[ ] List Available Macros:**

    -   Create a function `list_macros() -> Vec<String>` that scans the macros directory and returns all available macro names (without the `.json` extension).

    -   Create a Tauri command `list_macros() -> Vec<String>` that exposes this to the frontend.

-   **[ ] Frontend UI for Playback:**

    -   In the Svelte UI, create a component that:

        1.  Calls `list_macros` on mount to populate a list of available macros.

        2.  Displays each macro name with a "Play" button.

        3.  When a "Play" button is clicked, invokes the `play_macro` command.

        4.  Shows a loading state while the macro is playing.

-   **[ ] Validation:**

    -   Record a simple macro (e.g., moving the mouse and typing "hello").

    -   Save it with a name.

    -   Use the list command to verify it appears.

    -   Play the macro and verify it executes the exact sequence of events.

