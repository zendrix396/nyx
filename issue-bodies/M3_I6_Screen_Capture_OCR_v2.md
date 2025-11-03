### Summary

This issue focuses on giving Nyx its "sight" by integrating the powerful `olmOCR` vision-language model. This moves beyond simple text extraction to structured document understanding, capable of handling complex layouts, tables, and equations.

Given that `olmOCR` is a Python-based toolkit, we will implement it as a local, background MCP server. The Rust backend will capture the screen, send the image to this local server for processing, and receive the structured OCR data in return.

### Breakdown of Tasks

-   **[ ] Set Up Python Environment for the OCR Tool:**

    -   Create a dedicated directory for the tool (e.g., `src-tauri/tools/ocr_tool/`).

    -   Inside this directory, create a `requirements.txt` file listing all necessary dependencies: `olmocr`, `torch`, `transformers`, `fastapi`, `uvicorn`, `python-multipart`.

    -   Document the process for setting up a virtual environment and installing these dependencies.

-   **[ ] Develop the `olmOCR` MCP Server:**

    -   Create a Python script (`main.py`) inside the tool directory that uses FastAPI.

    -   This script will:

        1.  Load the `allenai/olmOCR-2-7B-1025-FP8` model and processor on startup to avoid repeated loading.

        2.  Create a single POST endpoint (e.g., `/ocr`).

        3.  The endpoint will accept a file upload (the screen capture image).

        4.  It will process the image using the `olmOCR` toolkit's methods to build the correct prompt.

        5.  It will run inference and return the extracted text and metadata (e.g., language, rotation) as a JSON response.

-   **[ ] Integrate the Tool into the Rust Backend:**

    -   **Process Management:** In the `tooling.rs` module, add functionality to the Orchestrator to spawn the Python OCR server as a background child process when the Nyx application starts. The process should be terminated gracefully when Nyx closes.

    -   **HTTP Client:** Update the `perception.rs` module. The `read_current_screen` function will now:

        1.  Capture the screen using the `scrap` crate.

        2.  Save the capture to a temporary file.

        3.  Make a `multipart/form-data` HTTP POST request to the local Python server endpoint (`http://127.0.0.1:PORT/ocr`), sending the image file.

        4.  Await and parse the JSON response.

-   **[ ] Update Tauri Command and Validation:**

    -   The Tauri command `read_current_screen()` will now orchestrate the capture and the HTTP call.

    -   Create a simple test UI in Svelte that invokes the command and displays the formatted JSON response from the `olmOCR` server, confirming the end-to-end pipeline is functional.

