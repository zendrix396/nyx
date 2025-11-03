### Summary

This issue focuses on two core functionalities: allowing the user to show/hide the app with a global hotkey and establishing a basic communication channel between the Svelte frontend and the Rust backend to verify the Tauri bridge is working.

### Breakdown of Tasks

-   **[ ] Implement Global Hotkey:**
    -   Add the `global-hotkey` crate to `src-tauri/Cargo.toml`.
    -   In `src-tauri/src/main.rs`, set up a global hotkey manager.
    -   Register a hotkey (e.g., `Cmd+Shift+G` or `Ctrl+Shift+G`).
    -   The hotkey callback should toggle the visibility of the main application window.

-   **[ ] Create a Rust Backend Command:**
    -   In `src-tauri/src/main.rs`, define a new Tauri command using the `#[tauri::command]` macro.
    -   Name it `greet`. It should take a `name: String` as an argument and return a `String` like `"Hello, [name]! You've been greeted from Rust!"`.
    -   Register this command in the `.invoke_handler()` in the `tauri::Builder`.

-   **[ ] Create a Svelte Frontend Component:**
    -   In `src/src/App.svelte`, create a simple UI with:
        -   An `<input type="text">` field.
        -   A `<button>`.
        -   A `<p>` tag to display the response from Rust.
    -   Import the `invoke` function from `@tauri-apps/api/tauri`.

-   **[ ] Implement Frontend-to-Backend Call:**
    -   When the button is clicked, call the `invoke` function from the Svelte component.
    -   Pass the command name (`greet`) and the value from the input field as the payload.
    -   Update the `<p>` tag with the string returned from the Rust backend.

-   **[ ] Validation:**
    -   The app can be hidden and shown using the global hotkey.
    -   Typing a name into the input field and clicking the button correctly displays the greeting from the Rust backend.