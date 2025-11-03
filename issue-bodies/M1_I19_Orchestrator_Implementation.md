### Summary

This issue covers the implementation of the Orchestrator, the central component that manages the agent's state machine and cognitive loop. The Orchestrator coordinates all modules (Perception, Cognition, Tooling, Knowledge) and manages the lifecycle of tasks from user input to completion.

### Breakdown of Tasks

-   **[ ] Define Core State Machine:**

    -   Create a new file: `src-tauri/src/orchestrator.rs`.

    -   Define an `AppState` enum with variants: `IDLE`, `LISTENING`, `RECORDING`, `EXECUTING`.

    -   Create an `Orchestrator` struct that holds:

        -   Current `AppState`.

        -   References to the modules (Perception, Cognition, Tooling, Knowledge) - these may be trait objects or shared references depending on architecture.

        -   An ephemeral session context for storing short-term conversation state.

-   **[ ] Implement State Transitions:**

    -   Implement methods on `Orchestrator` for state transitions:

        -   `start_listening()` - transitions from `IDLE` to `LISTENING`.

        -   `start_recording()` - transitions from `IDLE` to `RECORDING`.

        -   `start_executing(task: String)` - transitions from `IDLE` or `LISTENING` to `EXECUTING`.

        -   `stop()` - returns to `IDLE` from any state.

    -   Each transition should validate the current state and log the change.

-   **[ ] Implement the Cognitive Loop:**

    -   Create the main loop method `execute_task(task_description: String) -> Result<TaskResult, Error>`.

    -   This method should:

        1.  Transition to `EXECUTING` state.

        2.  Call the Cognition Module to generate a plan from the task description.

        3.  Iterate through the plan's steps.

        4.  For each step, call the appropriate module (Tooling for tool calls, Perception if needed for context).

        5.  Collect results and handle errors.

        6.  Log the execution to the Knowledge Base.

        7.  Return to `IDLE` state.

-   **[ ] Integrate with Module System:**

    -   The Orchestrator will need to initialize and hold references to:

        -   `PermissionManager` (from Issue #10) for security checks before tool execution.

        -   `ToolLibrary` (from Knowledge module) to discover available tools.

        -   `LLMProvider` (from Cognition module) for plan generation.

    -   Use dependency injection or a factory pattern to construct the Orchestrator with these dependencies.

-   **[ ] Event-Driven Architecture:**

    -   Emit Tauri events for state changes so the frontend can update the UI accordingly.

    -   Emit events for task progress (e.g., "Executing step 3 of 5").

    -   Listen for user input events (hotkey, CLI, voice) and trigger appropriate state transitions.

-   **[ ] Error Handling and Recovery:**

    -   Implement error handling that:

        -   Logs failures to the Knowledge Base for learning.

        -   Can rollback partially executed plans if critical steps fail.

        -   Provides clear error messages to the user via the frontend.

-   **[ ] Validation:**

    -   Create a simple test that initializes the Orchestrator and transitions through all states.

    -   Verify that the cognitive loop can execute a trivial plan (e.g., a single tool call).

    -   Confirm that state changes are properly logged and emitted as events.

