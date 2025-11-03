### Summary

This issue focuses on establishing the core behavior of Nyx as a background-first application. The agent should not run in a conventional window but as a service that reveals its UI only when summoned by a global hotkey. The implementation should be heavily inspired by the `krona-lite` example project.

### Breakdown of Tasks

-   **[ ] Configure Tauri for a Background Window:**

    -   In `tauri.conf.json`, modify the main window configuration to be:

        -   `"decorations": false` (frameless window).

        -   `"transparent": true` (allows for a floating, non-rectangular UI later).

        -   `"visible": false` (starts hidden).

        -   `"skipTaskbar": true` (to prevent it from showing in the taskbar/dock).

-   **[ ] Implement the Global Hotkey Plugin:**

    -   Add `tauri-plugin-global-shortcut` to `src-tauri/Cargo.toml`.

    -   In `src-tauri/src/main.rs`, initialize the plugin in the `tauri::Builder`.

-   **[ ] Create the Show/Hide Logic:**

    -   Inside the global shortcut handler in `main.rs`, get a handle to the main window.

    -   Check the window's visibility using `window.is_visible()`.

    -   If visible, call `window.hide()`.

    -   If hidden, call `window.show()` and then `window.set_focus()` to ensure it's ready for input.

-   **[ ] Implement a Basic Command Bridge (for testing):**

    -   Create a simple Tauri command `greet(name: &str) -> String` in Rust.

    -   On the Svelte frontend (`+page.svelte`), create a basic button that invokes this command to confirm that the webview and backend are communicating correctly when the window is shown.

-   **[ ] Prevent Accidental Exit:**

    -   Implement a handler for the `RunEvent::ExitRequested` event to call `api.prevent_exit()`. This ensures the background process doesn't close when the user tries to close the (hidden) window. Quitting will be handled explicitly via a system tray menu.

-   **[ ] Validation:**

    -   When the app starts, no window should be visible, and it should not appear in the taskbar.

    -   Pressing the registered global hotkey (e.g., `Ctrl+Shift+G`) toggles the visibility of the Svelte UI window.

    -   The command bridge works as expected when the window is visible.

