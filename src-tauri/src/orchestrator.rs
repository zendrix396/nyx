// src-tauri/src/orchestrator.rs
use serde::Serialize;
use tauri::Emitter;
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AppState {
    IDLE,
    LISTENING,
    RECORDING,
    EXECUTING,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskResult {
    pub success: bool,
    pub message: String,
}

#[derive(Error, Debug)]
pub enum OrchestratorError {
    #[error("Invalid state transition: Cannot go from {from:?} to {to:?}")]
    InvalidStateTransition { from: AppState, to: AppState },

    #[error("Cognition module failed: {0}")]
    CognitionError(String),

    #[error("Tooling module failed: {0}")]
    ToolingError(String),
    
    #[error("Tauri event emission failed: {0}")]
    EventError(#[from] tauri::Error),
}

// Placeholder structs for other modules
pub struct Perception;
pub struct Cognition;
pub struct Tooling;
pub struct Knowledge;

#[allow(dead_code)]
pub struct Orchestrator {
    pub state: AppState,
    app_handle: tauri::AppHandle,
    session_context: Option<String>,
    // Module references will be added here
    perception: Arc<Mutex<Perception>>,
    cognition: Arc<Mutex<Cognition>>,
    tooling: Arc<Mutex<Tooling>>,
    knowledge: Arc<Mutex<Knowledge>>,
}

impl Orchestrator {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            state: AppState::IDLE,
            app_handle,
            session_context: None,
            perception: Arc::new(Mutex::new(Perception)),
            cognition: Arc::new(Mutex::new(Cognition)),
            tooling: Arc::new(Mutex::new(Tooling)),
            knowledge: Arc::new(Mutex::new(Knowledge)),
        }
    }

    fn set_state(&mut self, new_state: AppState) -> Result<(), OrchestratorError> {
        log::info!("State transition: {:?} -> {:?}", self.state, new_state);
        self.state = new_state.clone();
        self.app_handle.emit("app_state_changed", new_state)?;
        Ok(())
    }

    pub fn start_listening(&mut self) -> Result<(), OrchestratorError> {
        if self.state != AppState::IDLE {
            return Err(OrchestratorError::InvalidStateTransition {
                from: self.state.clone(),
                to: AppState::LISTENING,
            });
        }
        self.set_state(AppState::LISTENING)
    }

    pub fn start_recording(&mut self) -> Result<(), OrchestratorError> {
        if self.state != AppState::IDLE {
            return Err(OrchestratorError::InvalidStateTransition {
                from: self.state.clone(),
                to: AppState::RECORDING,
            });
        }
        self.set_state(AppState::RECORDING)
    }
    
    pub fn start_executing(&mut self, task: String) -> Result<(), OrchestratorError> {
        if self.state != AppState::IDLE && self.state != AppState::LISTENING {
            return Err(OrchestratorError::InvalidStateTransition {
                from: self.state.clone(),
                to: AppState::EXECUTING,
            });
        }
        self.session_context = Some(task);
        self.set_state(AppState::EXECUTING)
    }

    pub fn stop(&mut self) -> Result<(), OrchestratorError> {
        self.session_context = None;
        self.set_state(AppState::IDLE)
    }

    pub async fn execute_task(&mut self, task_description: String) -> Result<TaskResult, OrchestratorError> {
        self.start_executing(task_description.clone())?;

        // 1. Call Cognition to get a plan
        self.app_handle.emit("task_progress", "Generating plan...")?;
        log::info!("Cognition: Generating plan for task: '{}'", task_description);
        // let plan = self.cognition.lock().unwrap().generate_plan(&task_description)?; // Real implementation
        tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Placeholder for async work

        // 2. Iterate through plan and execute steps
        log::info!("Tooling: Executing plan steps...");
        for i in 1..=3 {
            self.app_handle.emit("task_progress", format!("Executing step {} of 3...", i))?;
            // self.tooling.lock().unwrap().execute_step(step)?; // Real implementation
            log::info!("Executing step {}", i);
            tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Placeholder
        }

        // 3. Log to Knowledge Base
        log::info!("Knowledge: Logging execution results...");
        // self.knowledge.lock().unwrap().log_task(result)?; // Real implementation

        // 4. Return to IDLE
        self.stop()?;

        Ok(TaskResult {
            success: true,
            message: "Task completed successfully!".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri;
    use std::sync::{Arc, Mutex};

    // Mock AppHandle setup for testing is complex.
    // These tests will focus on the state logic, assuming event emission works.
    
    fn create_orchestrator_for_test() -> Orchestrator {
        // We can't easily mock AppHandle, so we'll need to build a minimal app instance for testing.
        // For pure unit tests, we'd abstract the event emitter behind a trait.
        // Given the scope, we will test the logic, and trust the tauri::Error handling.
        let app = tauri::Builder::default().build(tauri::generate_context!()).unwrap();
        Orchestrator::new(app.handle().clone())
    }

    #[test]
    fn test_initial_state_is_idle() {
        let orchestrator = create_orchestrator_for_test();
        assert_eq!(orchestrator.state, AppState::IDLE);
    }

    #[test]
    fn test_start_listening_from_idle() {
        let mut orchestrator = create_orchestrator_for_test();
        assert!(orchestrator.start_listening().is_ok());
        assert_eq!(orchestrator.state, AppState::LISTENING);
    }

    #[test]
    fn test_fail_start_listening_from_executing() {
        let mut orchestrator = create_orchestrator_for_test();
        orchestrator.state = AppState::EXECUTING;
        assert!(orchestrator.start_listening().is_err());
    }

    #[test]
    fn test_start_recording_from_idle() {
        let mut orchestrator = create_orchestrator_for_test();
        assert!(orchestrator.start_recording().is_ok());
        assert_eq!(orchestrator.state, AppState::RECORDING);
    }

    #[test]
    fn test_start_executing_from_idle() {
        let mut orchestrator = create_orchestrator_for_test();
        let task = "test task".to_string();
        assert!(orchestrator.start_executing(task.clone()).is_ok());
        assert_eq!(orchestrator.state, AppState::EXECUTING);
        assert_eq!(orchestrator.session_context, Some(task));
    }

    #[test]
    fn test_stop_returns_to_idle() {
        let mut orchestrator = create_orchestrator_for_test();
        orchestrator.state = AppState::EXECUTING;
        orchestrator.session_context = Some("a task".to_string());
        
        assert!(orchestrator.stop().is_ok());
        assert_eq!(orchestrator.state, AppState::IDLE);
        assert!(orchestrator.session_context.is_none());
    }

    #[tokio::test]
    async fn test_cognitive_loop_flow() {
        let mut orchestrator = create_orchestrator_for_test();
        let task = "a trivial plan".to_string();
        
        let result = orchestrator.execute_task(task).await;

        assert!(result.is_ok());
        let task_result = result.unwrap();
        assert!(task_result.success);
        assert_eq!(orchestrator.state, AppState::IDLE);
    }
}

