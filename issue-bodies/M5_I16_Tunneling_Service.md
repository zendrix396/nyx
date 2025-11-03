### Summary

This issue covers the implementation of a secure tunneling service, allowing the Nyx agent to be accessed remotely or to expose local services to the internet. This is a powerful feature for remote automation and control. This will be managed via the CLI.

### Breakdown of Tasks

-   **[ ] Research and Select a Tunneling Library/Service:**

    -   Investigate options like `ngrok`, `cloudflared`, or a library that interfaces with a development tunnel service.

    -   The chosen solution should be controllable programmatically from Rust.

-   **[ ] Implement Tunnel Management in the Core Agent:**

    -   Create a `TunnelManager` within the Rust backend.

    -   This manager will handle the lifecycle of tunnels: creating, starting, stopping, and listing them.

    -   It will expose these functions over the singleton JSON-RPC service.

-   **[ ] Develop CLI Commands for Tunneling:**

    -   Implement the `nyx tunnel create [--local-port <PORT>]` command. This will tell the agent to create a new tunnel, either exposing the agent's own RPC service or forwarding a specific local port.

    -   Implement the `nyx tunnel list` command to display all active tunnels and their public URLs.

    -   Implement the `nyx tunnel kill <ID>` command to terminate a running tunnel.

-   **[ ] Implement Local Port Forwarding:**

    -   Add the logic to the `TunnelManager` to proxy traffic from the public tunnel URL to a specified local port (`localhost:PORT`).

-   **[ ] Validation:**

    -   `nyx tunnel create` successfully establishes a tunnel and prints the public URL.

    -   The agent's RPC service is accessible through the public URL when no local port is specified.

    -   `nyx tunnel create --local-port 3000` correctly forwards traffic to `localhost:3000`.

    -   `nyx tunnel list` shows the correct information for all active tunnels.

    -   `nyx tunnel kill <ID>` successfully terminates the specified tunnel.

