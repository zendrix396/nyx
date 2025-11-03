### Summary

This issue covers giving Nyx its "ears" by integrating a local Speech-to-Text (STT) engine. The goal is to capture audio from the user's microphone, transcribe it in near real-time, and use the text to populate the command input field in the UI. `whisper.cpp` is the primary candidate due to its high performance and accuracy.

### Breakdown of Tasks

-   **[ ] Research FFI for whisper.cpp:**

    -   `whisper.cpp` is a C++ library. Investigate the best way to call it from Rust.

    -   Options include using a pre-existing crate that provides bindings (e.g., `whisper-rs`) or creating custom Foreign Function Interface (FFI) bindings.

    -   The chosen method should allow for streaming audio input for real-time transcription.

-   **[ ] Set Up Audio Input:**

    -   Add an audio capture crate like `cpal` to `src-tauri/Cargo.toml`.

    -   In the `perception.rs` module, implement the logic to:

        1.  Enumerate available microphones.

        2.  Open an input stream from the default microphone.

        3.  Capture audio data into a buffer.

-   **[ ] Implement Transcription Logic:**

    -   In a separate thread, create a loop that continuously reads from the audio buffer.

    -   Pass the audio data to the Whisper STT engine via the chosen FFI method.

    -   The STT engine should be initialized once and kept running to avoid model load delays.

-   **[ ] Bridge to Frontend:**

    -   When the STT engine produces a transcribed text segment, use Tauri Events to send the text from the Rust backend to the Svelte frontend.

    -   The frontend should listen for this event and update the value of its main text input box.

-   **[ ] Manage Listening State:**

    -   Create Tauri commands `start_listening` and `stop_listening`.

    -   These commands will control the audio capture thread.

    -   Add a "microphone" button in the Svelte UI to toggle the listening state.

-   **[ ] Validation:**

    -   The user can click the microphone button to start listening.

    -   As the user speaks, their words should appear in the application's input field.

    -   Clicking the button again should stop the microphone.

