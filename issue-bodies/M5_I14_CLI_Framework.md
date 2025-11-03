### Summary

This issue covers the creation of a command-line interface (CLI) for Nyx Agent. This provides a headless way to interact with the agent for scripting, status checks, and managing services like tunneling. The implementation will use the `clap` crate for robust argument parsing.

### Breakdown of Tasks

-   **[ ] Add `clap` Dependency:**

    -   Add `clap` with the `derive` feature to `src-tauri/Cargo.toml`.

-   **[ ] Define Core CLI Structure:**

    -   Create a new module for CLI logic (`src-tauri/src/cli.rs`).

    -   Define a top-level `Cli` struct with `clap`.

    -   Define a `Commands` enum for subcommands like `status`, `tunnel`, and `update`.

-   **[ ] Implement `status` Command:**

    -   The `status` command will connect to the running singleton agent service.

    -   It will request and print the current status (e.g., `IDLE`, `EXECUTING`, current task).

-   **[ ] Implement `tunnel` Subcommand Stub:**

    -   Create a placeholder for the `tunnel` subcommand structure.

    -   It will have its own subcommands like `create`, `list`, `kill`. The logic will be implemented in Issue #16.

-   **[ ] Integrate into `main.rs`:**

    -   Modify the `main` function to check if it's being run with CLI arguments.

    -   If so, parse the arguments using `clap` and dispatch to the appropriate command handler.

    -   If not (e.g., launched from a GUI), start the Tauri application as normal.

-   **[ ] Validation:**

    -   Running `nyx-agent --help` displays the main help message.

    -   Running `nyx-agent status` successfully connects to the agent and prints its status.

    -   Running `nyx-agent tunnel --help` shows the help message for the tunnel subcommand.

