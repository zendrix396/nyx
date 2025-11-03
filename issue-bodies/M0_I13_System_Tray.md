### Summary

For a background-first application like Nyx, a system tray icon is essential for user control. This issue covers adding a tray icon and a context menu that provides core application-level actions, such as showing the agent, opening settings, and safely quitting the background process.

### Breakdown of Tasks

-   **[ ] Enable Tray Icon Feature:**

    -   In `src-tauri/Cargo.toml`, add the `tray-icon` feature to the `tauri` dependency.

        ```toml

        tauri = { version = "2.0.0-beta", features = ["tray-icon"] }

        ```

-   **[ ] Create the System Tray:**

    -   In the `.setup()` hook of the `tauri::Builder` in `main.rs`, create a new `SystemTray` instance.

    -   Define a `SystemTrayMenu` with the following items:

        -   "Show/Hide Agent"

        -   A `SystemTrayMenuItem::Separator`.

        -   "Settings..."

        -   "Quit"

-   **[ ] Implement Tray Menu Event Handling:**

    -   Add an `.on_system_tray_event()` handler to the `tauri::Builder`.

    -   Match on the `SystemTrayEvent::MenuItemClick`.

    -   Implement the logic for each menu item:

        -   **Show/Hide Agent:** Get the main window handle and toggle its visibility, similar to the global hotkey logic.

        -   **Settings...:** (Future) This will open a dedicated settings window. For now, it can just log a message.

        -   **Quit:** Get the `AppHandle` and call `app.exit(0)` to gracefully shut down the application.

-   **[ ] Design and Add an Icon:**

    -   Create a simple, monochrome icon suitable for a system tray (e.g., a 32x32 PNG).

    -   Add the icon path to the `SystemTray::new().with_icon(...)` builder.

    -   Ensure the icon is properly included in the application bundle by configuring `tauri.conf.json`.

-   **[ ] Validation:**

    -   On application start, a Nyx icon appears in the system tray (or menu bar on macOS).

    -   Right-clicking the icon displays the correct menu.

    -   The "Show/Hide Agent" menu item correctly toggles the main UI.

    -   The "Quit" menu item successfully terminates the application.

