### Summary

To support multiple Large Language Models (LLMs) like Gemini, OpenAI, and local models via Ollama, we need to create a flexible abstraction layer. This issue involves designing a generic `LLMProvider` trait in Rust and implementing the first provider for the Google Gemini API. This is inspired by the factory pattern seen in `cursor-agent`.

### Breakdown of Tasks

-   **[ ] Define the `LLMProvider` Trait:**

    -   In `src-tauri/src/modules/cognition.rs`, define an `async` trait named `LLMProvider`.

    -   It should have a primary method like `generate_plan(&self, context: &str, available_tools: Vec<ToolInfo>) -> Result<Plan, Error>`.

    -   Define shared structs for `ToolInfo` (name, description, parameters) and `Plan` (a sequence of tool calls).

-   **[ ] Implement the Gemini Provider:**

    -   Create a `GeminiProvider` struct that implements the `LLMProvider` trait.

    -   It will hold the API key (retrieved securely via the `keyring` module from Issue #5).

    -   Implement the `generate_plan` method, which will:

        1.  Construct the specific prompt for the Gemini API, including the user's goal and the list of available tools.

        2.  Use the `reqwest` crate to make the HTTP request to the Gemini API endpoint.

        3.  Parse the JSON response from the API.

        4.  Convert the API response into the standardized `Plan` struct.

-   **[ ] Create an Agent Configuration:**

    -   In the `Knowledge` module, create a way to store and retrieve the currently selected LLM provider and its associated API key.

    -   For now, this can be a simple JSON file.

-   **[ ] Implement a Provider Factory:**

    -   Create a factory function `create_llm_provider() -> Box<dyn LLMProvider>` in the `Cognition` module.

    -   This function will read the agent's configuration and instantiate the correct provider (e.g., `GeminiProvider`).

    -   The Orchestrator will call this factory to get the active LLM brain for the cognitive loop.

