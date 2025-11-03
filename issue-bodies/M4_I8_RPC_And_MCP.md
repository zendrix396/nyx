### Summary

With the core modules in place, this issue focuses on building a standardized and robust communication layer for tools based on the Model Context Protocol (MCP). This will decouple the tools from the main agent, allowing them to be developed, run, and even generated independently. We will implement a formal JSON-RPC protocol over an async pipe (Unix sockets/named pipes) as the transport layer.

### Breakdown of Tasks

-   **[ ] Define the JSON-RPC Specification for MCP:**

    -   Finalize the exact JSON structure for MCP requests and responses, adhering to the JSON-RPC 2.0 standard.

    -   This includes `method` (e.g., `file.search`), `params` (a JSON object), and `id`.

    -   Define a standard error response format.

-   **[ ] Create Tooling Module and RPC Client:**

    -   Create a new file: `src-tauri/src/modules/tooling.rs`.

    -   Implement an `RpcClient` struct that communicates over an async pipe.

    -   The client will have a method `call(tool_name: &str, params: serde_json::Value) -> Result<serde_json::Value, Error>`.

    -   This method will serialize the request, send it to the appropriate tool server, and deserialize the response.

-   **[ ] Create Knowledge Base for Tool Discovery:**

    -   Create `src-tauri/src/modules/knowledge.rs`.

    -   Implement a `ToolLibrary` that loads tool definitions from a directory (e.g., `~/.config/nyx-agent/tools/`).

    -   Each tool definition file will describe its functions, parameters, and how to launch its server process.

        ```json

        {

          "server_command": "run-file-tool-server",

          "tools": [

            {

              "name": "file.search",

              "description": "Searches for files on the local system.",

              "parameters": { "pattern": "string", "path": "string" }

            }

          ]

        }

        ```

-   **[ ] Implement First Built-in MCP Tool Server:**

    -   Create a new Rust binary crate within the workspace: `src-tauri/tools/file-tools/`.

    -   This binary will act as a standalone server that listens for JSON-RPC requests on `stdin`.

    -   Implement the logic for basic file operations (`file.search`, `file.read`, `file.write`).

    -   The Orchestrator will be responsible for launching this tool server as a background process on startup, connecting its `stdin`/`stdout` to the RPC client.

-   **[ ] Refactor Cognition Module for MCP Planning:**

    -   Update the `cognition.rs` module.

    -   When generating a plan, the LLM will now be given a list of available MCP tool names and descriptions from the `ToolLibrary`.

    -   The LLM's output should be a JSON array of MCP calls, which the Orchestrator can execute step-by-step using the `RpcClient`.

