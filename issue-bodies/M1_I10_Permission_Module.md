### Summary

This issue covers the design and implementation of a robust permission system, which is critical for an agent that can perform sensitive operations. This module will act as a gatekeeper for all actions that modify the user's system, such as file I/O and command execution. The design will be inspired by `cursor-agent`'s permission model.

### Breakdown of Tasks

-   **[ ] Define Core Data Structures:**

    -   Create a new module: `src-tauri/src/modules/permissions.rs`.

    -   Define a `PermissionRequest` struct containing the `operation_type` (e.g., "delete_file") and `details` (a JSON object).

    -   Define a `PermissionStatus` enum (`GRANTED`, `DENIED`, `NEEDS_CONFIRMATION`).

    -   Define a `PermissionOptions` struct to hold configuration (`yolo_mode`, `command_allowlist`, `command_denylist`, `delete_file_protection`).

-   **[ ] Implement the PermissionManager:**

    -   Create a `PermissionManager` struct that holds the `PermissionOptions`.

    -   Implement a method `request_permission(&self, request: PermissionRequest) -> bool`.

    -   This method will contain the core logic:

        1.  Check against the `command_denylist` first.

        2.  If in `yolo_mode`, check against the `command_allowlist` and `delete_file_protection` rules.

        3.  If not automatically decided, return `NEEDS_CONFIRMATION`.

-   **[ ] Integrate with the Orchestrator:**

    -   The Orchestrator will own the `PermissionManager`.

    -   All tool calls that are security-sensitive must first go through `PermissionManager`.

    -   If confirmation is needed, the Orchestrator will emit a Tauri event to the Svelte frontend.

-   **[ ] Create Frontend Permission UI:**

    -   Develop a simple, modal-like component in Svelte that listens for the permission request event.

    -   The modal should display the operation and details, with "Allow" and "Deny" buttons.

    -   The user's choice will emit an event back to the Rust backend. The Orchestrator will then either proceed with or cancel the tool call.

