use crate::modules::io_controller;
use rdev::{Button, EventType, Key};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fs,
    path::PathBuf,
    thread,
    time::Duration,
};
use tauri::Manager;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MacroError {
    #[error("File system error: {0}")]
    FileSystem(String),
    #[error("Deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("I/O Controller error: {0}")]
    Io(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub events: Vec<TimedEvent>,
}

// Custom serializable representation of EventType
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum SerializableEventType {
    MouseMove { x: f64, y: f64 },
    ButtonPress { button: String },
    ButtonRelease { button: String },
    KeyPress { key: String },
    KeyRelease { key: String },
    Wheel { delta_x: i64, delta_y: i64 },
}

impl From<&EventType> for SerializableEventType {
    fn from(event_type: &EventType) -> Self {
        match event_type {
            EventType::MouseMove { x, y } => SerializableEventType::MouseMove { x: *x, y: *y },
            EventType::ButtonPress(button) => SerializableEventType::ButtonPress {
                button: format!("{:?}", button).to_lowercase(),
            },
            EventType::ButtonRelease(button) => SerializableEventType::ButtonRelease {
                button: format!("{:?}", button).to_lowercase(),
            },
            EventType::KeyPress(key) => SerializableEventType::KeyPress {
                key: format!("{:?}", key),
            },
            EventType::KeyRelease(key) => SerializableEventType::KeyRelease {
                key: format!("{:?}", key),
            },
            EventType::Wheel { delta_x, delta_y } => SerializableEventType::Wheel {
                delta_x: *delta_x as i64,
                delta_y: *delta_y as i64,
            },
        }
    }
}

impl From<SerializableEventType> for EventType {
    fn from(ser: SerializableEventType) -> Self {
        match ser {
            SerializableEventType::MouseMove { x, y } => EventType::MouseMove { x, y },
            SerializableEventType::ButtonPress { button } => {
                EventType::ButtonPress(parse_button(&button))
            }
            SerializableEventType::ButtonRelease { button } => {
                EventType::ButtonRelease(parse_button(&button))
            }
            SerializableEventType::KeyPress { key } => EventType::KeyPress(parse_key(&key)),
            SerializableEventType::KeyRelease { key } => EventType::KeyRelease(parse_key(&key)),
            SerializableEventType::Wheel { delta_x, delta_y } => {
                EventType::Wheel { delta_x, delta_y }
            }
        }
    }
}

fn parse_button(s: &str) -> Button {
    match s.to_lowercase().as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => Button::Left,
    }
}

fn parse_key(s: &str) -> Key {
    // Parse the key string back to Key enum
    // This is a simplified parser - you may need to expand this
    match s {
        "KeyA" => Key::KeyA,
        "KeyB" => Key::KeyB,
        "KeyC" => Key::KeyC,
        "KeyD" => Key::KeyD,
        "KeyE" => Key::KeyE,
        "KeyF" => Key::KeyF,
        "KeyG" => Key::KeyG,
        "KeyH" => Key::KeyH,
        "KeyI" => Key::KeyI,
        "KeyJ" => Key::KeyJ,
        "KeyK" => Key::KeyK,
        "KeyL" => Key::KeyL,
        "KeyM" => Key::KeyM,
        "KeyN" => Key::KeyN,
        "KeyO" => Key::KeyO,
        "KeyP" => Key::KeyP,
        "KeyQ" => Key::KeyQ,
        "KeyR" => Key::KeyR,
        "KeyS" => Key::KeyS,
        "KeyT" => Key::KeyT,
        "KeyU" => Key::KeyU,
        "KeyV" => Key::KeyV,
        "KeyW" => Key::KeyW,
        "KeyX" => Key::KeyX,
        "KeyY" => Key::KeyY,
        "KeyZ" => Key::KeyZ,
        "Space" => Key::Space,
        "Escape" => Key::Escape,
        "Backspace" => Key::Backspace,
        "Tab" => Key::Tab,
        "ShiftLeft" => Key::ShiftLeft,
        "ShiftRight" => Key::ShiftRight,
        "ControlLeft" => Key::ControlLeft,
        "ControlRight" => Key::ControlRight,
        "MetaLeft" => Key::MetaLeft,
        "MetaRight" => Key::MetaRight,
        _ => {
            // Try to parse as Unknown(u32)
            if let Some(num_str) = s.strip_prefix("Unknown(").and_then(|s| s.strip_suffix(")")) {
                if let Ok(num) = num_str.parse::<u32>() {
                    return Key::Unknown(num);
                }
            }
            Key::Unknown(0)
        }
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimedEvent {
    // Store only the event_type, not the full Event, to avoid serialization issues
    // with UnicodeInfo and other non-serializable fields
    #[serde(serialize_with = "serialize_event_type", deserialize_with = "deserialize_event_type")]
    pub event_type: EventType,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac<f64>")]
    pub time_since_previous: Duration,
}

fn serialize_event_type<S>(event_type: &EventType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ser: SerializableEventType = event_type.into();
    ser.serialize(serializer)
}

fn deserialize_event_type<'de, D>(deserializer: D) -> Result<EventType, D::Error>
where
    D: Deserializer<'de>,
{
    let ser = SerializableEventType::deserialize(deserializer)?;
    Ok(ser.into())
}

/// Returns the path to the macros directory, creating it if it doesn't exist.
fn get_macros_dir(app_handle: &tauri::AppHandle) -> Result<PathBuf, MacroError> {
    let config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| MacroError::FileSystem(format!("Failed to get config dir: {}", e)))?;
    let macros_dir = config_dir.join("nyx-agent/macros");

    fs::create_dir_all(&macros_dir)
        .map_err(|e| MacroError::FileSystem(format!("Failed to create macros dir: {}", e)))?;

    Ok(macros_dir)
}

/// Loads a macro from a JSON file.
pub fn load_macro(name: &str, app_handle: &tauri::AppHandle) -> Result<Macro, MacroError> {
    let macros_dir = get_macros_dir(app_handle)?;
    let file_path = macros_dir.join(format!("{}.json", name));

    log::info!("Loading macro from: {:?}", file_path);
    let json_string = fs::read_to_string(file_path)
        .map_err(|e| MacroError::FileSystem(format!("Failed to read macro file: {}", e)))?;

    let macro_data: Macro = serde_json::from_str(&json_string)?;

    Ok(macro_data)
}

/// Executes the events in a given Macro struct.
pub fn play_macro(macro_data: &Macro) -> Result<(), MacroError> {
    log::info!("Playing macro: {}", macro_data.name);
    for timed_event in &macro_data.events {
        // Wait for the specified duration
        thread::sleep(timed_event.time_since_previous);

        // Execute the event
        if let Err(e) = io_controller::send_event(&timed_event.event_type) {
            let error_msg = format!("Failed to send event during macro playback: {}", e);
            log::error!("{}", error_msg);
            // We can choose to abort or continue on error. For now, we'll log and continue.
        }
    }
    log::info!("Finished playing macro: {}", macro_data.name);
    Ok(())
}

/// Lists all available macro files in the macros directory.
pub fn list_macros(app_handle: &tauri::AppHandle) -> Result<Vec<String>, MacroError> {
    let macros_dir = get_macros_dir(app_handle)?;
    let mut macro_names = Vec::new();

    let entries = fs::read_dir(macros_dir)
        .map_err(|e| MacroError::FileSystem(format!("Could not read macros directory: {}", e)))?;

    for entry in entries {
        let entry =
            entry.map_err(|e| MacroError::FileSystem(format!("Could not read directory entry: {}", e)))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                macro_names.push(stem.to_string());
            }
        }
    }
    Ok(macro_names)
}

