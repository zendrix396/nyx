### Summary

With the core modules in place, this issue focuses on building a standardized communication layer for tools based on the Model Context Protocol (MCP). This will decouple the tools from the main agent, allowing them to be developed, run, and even generated independently. We will implement both the MCP Client (within the Orchestrator) and an MCP Server that hosts our first built-in tools.

### Breakdown of Tasks

-   **[ ] Define the MCP JSON-RPC Specification:**

    -   Finalize the exact JSON structure for MCP requests and responses. It should include:

        -   `tool_name` (e.g., `file.search`)

        -   `params` (a JSON object of arguments)

        -   `id` for tracking requests.

    -   Define a standard error response format.

-   **[ ] Create Tooling Module and MCP Client:**

    -   Create a new file: `src-tauri/src/modules/tooling.rs`.

    -   Implement an `MCPClient` struct.

    -   The client will have a method `call(tool_name: &str, params: serde_json::Value) -> Result<serde_json::Value, Error>`.

    -   This method will serialize the request, send it over HTTP to the appropriate tool server's port, and deserialize the response.

-   **[ ] Create Knowledge Base for Tool Discovery:**

    -   Create `src-tauri/src/modules/knowledge.rs`.

    -   Implement a `ToolLibrary` that loads tool definitions from a directory (e.g., `~/.config/nyx-agent/tools/`).

    -   Each tool definition file (e.g., `file_tools.json`) will describe the tool's functions, parameters, and the port its server runs on.

        ```json

        {

          "server_command": "run-file-tool-server",

          "port": 3001,

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

    -   Use a lightweight web server framework like `axum` or `actix-web`.

    -   Implement the logic for basic file operations (`file.search`, `file.read`, `file.write`).

    -   The server will listen on a specific port and respond to MCP JSON-RPC requests.

    -   The Orchestrator will be responsible for launching this tool server as a background process on startup.

-   **[ ] Refactor Cognition Module for MCP Planning:**

    -   Update the `cognition.rs` module.

    -   When generating a plan, the LLM will now be given a list of available MCP tool names and descriptions from the `ToolLibrary`.

    -   The LLM's output should be a JSON array of MCP calls, which the Orchestrator can execute step-by-step using the `MCPClient`.

