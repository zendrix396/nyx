### Summary

To enable the CLI to communicate with the main background agent, we need to ensure only one instance of the agent's core service is running at a time. This issue covers implementing a singleton pattern using a file lock and establishing an inter-process communication (IPC) channel using async pipes (Unix sockets or Windows named pipes).

### Breakdown of Tasks

-   **[ ] Implement File-Based Locking:**

    -   Create a `SingletonManager` in the Rust backend.

    -   On startup, the manager will attempt to acquire an exclusive lock on a file (e.g., `~/.config/nyx-agent/agent.lock`).

-   **[ ] Create Singleton Server Logic:**

    -   If the lock is acquired, the current process becomes the singleton server.

    -   It will create and listen on an async pipe (e.g., `~/.config/nyx-agent/agent.sock`).

    -   It will write the socket path and its process ID into the lock file for clients to discover.

    -   This server will host a JSON-RPC service that exposes core Orchestrator functionality to the CLI.

-   **[ ] Create Client Connection Logic:**

    -   If the lock is already held, the current process is a client.

    -   It will read the lock file to get the socket path of the running server.

    -   It will connect to the server's async pipe.

    -   The CLI commands will use this connection to send JSON-RPC requests to the main agent.

-   **[ ] Handle Stale Locks:**

    -   The client logic should check if the process ID in the lock file is still running.

    -   If the process is not running, the client should attempt to clean up the stale lock file and retry acquiring the lock to become the new server.

-   **[ ] Validation:**

    -   Launching the Nyx agent GUI successfully creates the lock file and starts the IPC server.

    -   Launching a second instance of the GUI or running a CLI command does not start a new agent process.

    -   The CLI can successfully connect to the running agent via the socket and receive a response (e.g., for the `status` command).

