### Summary

To ensure users can easily stay up-to-date, this issue covers implementing a self-update mechanism for Nyx Agent. The agent will be able to check for new releases, download them, and apply the update.

### Breakdown of Tasks

-   **[ ] Create an Update Service:**

    -   In the Rust backend, create an `UpdateService` module.

    -   This service will be responsible for checking a designated URL (e.g., GitHub Releases API) for the latest version of the agent for the current platform.

-   **[ ] Implement Update Check Logic:**

    -   The `UpdateService` will compare the current application version with the latest available version.

-   **[ ] Implement Download and Apply Logic:**

    -   If a new version is available, the service will download the appropriate release asset (e.g., a compressed binary).

    -   It will decompress the asset and replace the current executable with the new one. This should be done in a safe, atomic way (e.g., download to a temp file, validate, then replace).

    -   The service should handle permissions and cleanup of old versions.

-   **[ ] Develop `nyx update` CLI Command:**

    -   Create the `nyx update` command in the CLI.

    -   This command will trigger the `UpdateService` to perform a check.

    -   If an update is available, it will prompt the user to confirm before downloading and applying it, showing progress during the download.

-   **[ ] (Optional) Add Automatic Update Check:**

    -   Add a configuration option to enable/disable automatic background checks for updates.

    -   If enabled, the agent will periodically check for updates and notify the user through the GUI if one is available.

-   **[ ] Validation:**

    -   `nyx update` correctly identifies when the agent is up-to-date.

    -   When an update is available, `nyx update` successfully downloads and applies it.

    -   After an update, the agent restarts and reports the new version number.

