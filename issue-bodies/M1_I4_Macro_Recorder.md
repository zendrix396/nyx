### Summary

This issue covers the implementation of a "dumb" macro recorder. It will listen for global mouse and keyboard events and log them into a sequence. This sequence can then be saved to a file for later playback.

### Breakdown of Tasks

-   **[ ] Define Data Structures:**

    -   In a new module (e.g., `src-tauri/src/modules/macro_engine.rs`), define structs and enums for storing macro data.

    -   The data needs to be serializable with `serde`.

    -   Example structure:

        ```rust

        #[derive(Serialize, Deserialize)]

        pub struct Macro {

            pub name: String,

            pub events: Vec<TimedEvent>,

        }



        #[derive(Serialize, Deserialize)]

        pub struct TimedEvent {

            pub event: rdev::Event,

            pub time_since_previous: std::time::Duration,

        }

        ```

-   **[ ] Implement Event Listener:**

    -   Using the `rdev` crate, set up a global event listener in a separate thread to avoid blocking the main application.

    -   Use a `channel` (like `std::sync::mpsc::channel`) to send captured `rdev::Event`s from the listener thread back to the main thread.

-   **[ ] Manage Recording State:**

    -   In the `Orchestrator` (or a main state manager), implement a state enum, e.g., `AppState { IDLE, RECORDING }`.

    -   Create Tauri commands `start_recording` and `stop_recording`.

    -   When `start_recording` is called, change the state to `RECORDING` and begin collecting events received from the listener thread's channel. Calculate the time delta between events.

-   **[ ] Save Macro to File:**

    -   The `stop_recording` command should take a `name: String` as an argument.

    -   It should package the collected `TimedEvent`s into a `Macro` struct.

    -   Serialize the `Macro` struct to a JSON string using `serde_json`.

    -   Save the JSON string to a file in a designated macros directory (e.g., `~/.config/nyx-agent/macros/[name].json`).

-   **[ ] Frontend UI:**

    -   Create a simple UI in Svelte with "Start Recording" and "Stop Recording" buttons.

    -   The stop button should prompt for a macro name.

