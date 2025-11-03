### Summary

This issue covers creating a robust and reusable client for interacting with the Google Gemini API. It includes setting up HTTP requests, JSON serialization/deserialization, and securely storing the user's API key instead of hardcoding it or leaving it in a plain text file.

### Breakdown of Tasks

-   **[ ] Add Dependencies:**

    -   Add `reqwest` (with `json` feature), `serde`, `serde_json`, and `tokio` to `src-tauri/Cargo.toml`.

    -   Add the `keyring` crate for secure credential storage.

-   **[ ] Secure API Key Management:**

    -   Create two Tauri commands: `save_api_key(key: String)` and `get_api_key() -> Option<String>`.

    -   The `save_api_key` command should use the `keyring` crate to store the API key in the OS's native secret manager.

    -   The `get_api_key` command should retrieve it. This prevents the key from ever being exposed to the frontend directly after being saved.

    -   Create a simple settings page in the Svelte UI for the user to input and save their API key.

-   **[ ] Create Cognition Module:**

    -   Create a new file: `src-tauri/src/modules/cognition.rs`.

    -   Define the necessary `serde` structs to represent the Gemini API's request and response bodies.

-   **[ ] Implement API Client:**

    -   Inside `cognition.rs`, create an `async` function `ask_gemini(prompt: String) -> Result<String, Error>`.

    -   This function should:

        1.  Retrieve the API key using the `keyring` service. Return an error if it's not set.

        2.  Create a `reqwest::Client`.

        3.  Construct the request body using the prompt and the defined structs.

        4.  Send the POST request to the Gemini API endpoint.

        5.  Await the response and handle potential HTTP errors.

        6.  Deserialize the JSON response and extract the generated text content.

        7.  Return the text content or an error.

-   **[ ] Expose to Tauri:**

    -   Create a new Tauri command `invoke_llm(prompt: String)` that calls `ask_gemini` and returns the result to the frontend. This will be used for testing the entire pipeline.

