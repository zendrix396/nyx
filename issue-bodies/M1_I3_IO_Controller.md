### Summary

This issue involves creating a dedicated module within the Rust backend to programmatically control the user's mouse and keyboard. This is the foundational "hands" module that will execute actions. The `rdev` crate is preferred as it can both listen for and send events.

### Breakdown of Tasks

-   **[ ] Add Dependency:**
    -   Add the `rdev` crate to `src-tauri/Cargo.toml`.

-   **[ ] Create I/O Controller Module:**
    -   Create a new file: `src-tauri/src/modules/io_controller.rs`.
    -   Define a public function `send_event(event: &rdev::Event)` that handles the simulation of events.
    -   Implement helper functions for common actions:
        -   `move_mouse(x: f64, y: f64)`
        -   `click(button: rdev::Button)`
        -   `press_key(key: rdev::Key)`
        -   `release_key(key: rdev::Key)`
        -   `type_string(text: &str)` (This will simulate a sequence of key presses and releases).

-   **[ ] Expose to Tauri:**
    -   Create simple Tauri commands that wrap these helper functions. For example, `execute_mouse_click(x, y)` and `execute_keypress(key_name)`.
    -   This allows testing the I/O functionality directly from the frontend if needed.

-   **[ ] Error Handling:**
    -   Ensure that functions within the `io_controller` return a `Result` to handle potential OS-level errors during event simulation.

-   **[ ] Validation:**
    -   Create a temporary test command in Tauri (e.g., `test_io`) that, when called from the frontend, moves the mouse to `(100, 100)` and types "hello". This verifies the controller is working correctly.