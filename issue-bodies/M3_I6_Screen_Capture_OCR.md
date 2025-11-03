### Summary

This issue focuses on giving Nyx its "sight." It involves integrating a fast screen capture library and connecting it to a local Optical Character Recognition (OCR) engine. The end goal is a single, efficient function that can return all visible text on the screen, which will later be used to provide context to the Cognition Module.

### Breakdown of Tasks

-   **[ ] Add Dependencies:**

    -   Add `scrap` to `src-tauri/Cargo.toml` for high-performance screen capture.

    -   Research and select a suitable local OCR library. `tesseract-rs` is a common choice, but investigate alternatives like `rust-tess` for performance. Add the chosen library to dependencies.

-   **[ ] Create Perception Module:**

    -   Create a new file: `src-tauri/src/modules/perception.rs`.

    -   This module will house all functions related to understanding the machine's state (screen, audio, etc.).

-   **[ ] Implement Screen Capturing:**

    -   In `perception.rs`, create a function `capture_screen() -> Result<Vec<u8>, Error>`.

    -   This function should use the `scrap` library to capture the primary display and return the raw image bytes (e.g., in BGRA format).

    -   Handle potential errors, such as failure to access the display.

-   **[ ] Implement OCR Processing:**

    -   Create a function `extract_text_from_image(image_bytes: &[u8], width: u32, height: u32) -> Result<String, Error>`.

    -   This function will:

        1.  Take the raw image bytes from `capture_screen`.

        2.  Initialize the chosen OCR engine.

        3.  Feed the image data to the engine.

        4.  Return the extracted text as a single string.

-   **[ ] Create Main Tauri Command:**

    -   Combine the above functions into a single, user-facing Tauri command: `read_current_screen() -> Result<String, String>`.

    -   This command will call `capture_screen`, then pass the result to `extract_text_from_image`, and return the final text to the frontend for display.

    -   This serves as the primary validation point.

-   **[ ] Validation:**

    -   Create a button in the Svelte UI that, when clicked, invokes `read_current_screen`.

    -   The extracted text from the screen should be displayed in a text area or console log, verifying that both screen capture and OCR are working correctly.

