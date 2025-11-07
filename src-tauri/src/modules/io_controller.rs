use rdev::{Button, EventType, Key};
use std::sync::Mutex;
use std::{thread, time};
use lazy_static::lazy_static;

// Using lazy_static and a Mutex to ensure that the rdev::simulate function,
// which is not thread-safe, is only called by one thread at a time.
lazy_static! {
    static ref EVENT_LOCK: Mutex<()> = Mutex::new(());
}

/// A centralized function to send system events.
/// This function wraps the rdev::simulate call in a mutex to prevent potential race conditions
/// and OS-level errors.
///
/// # Arguments
/// * `event_type` - The `rdev::EventType` to be simulated.
///
/// # Returns
/// A `Result` which is `Ok(())` on success, or an `Err(String)` on failure.
pub fn send_event(event_type: &EventType) -> Result<(), String> {
    let _lock = EVENT_LOCK.lock().map_err(|e| format!("Failed to acquire event lock: {}", e))?;
    rdev::simulate(event_type).map_err(|e| format!("Failed to simulate event: {:?}", e))?;
    Ok(())
}

// --- Helper Functions for Common Actions ---

/// Moves the mouse to the specified screen coordinates.
pub fn move_mouse(x: f64, y: f64) -> Result<(), String> {
    send_event(&EventType::MouseMove { x, y })
}

/// Simulates a click with a specified mouse button.
pub fn click(button: Button) -> Result<(), String> {
    send_event(&EventType::ButtonPress(button))?;
    send_event(&EventType::ButtonRelease(button))
}

/// Simulates pressing a single key.
pub fn press_key(key: Key) -> Result<(), String> {
    send_event(&EventType::KeyPress(key))
}

/// Simulates releasing a single key.
pub fn release_key(key: Key) -> Result<(), String> {
    send_event(&EventType::KeyRelease(key))
}

/// Simulates typing a string by pressing and releasing each character's corresponding key.
pub fn type_string(text: &str) -> Result<(), String> {
    for c in text.chars() {
        let key = char_to_key(c);
        press_key(key)?;
        release_key(key)?;
    }
    Ok(())
}

// --- Utility Functions ---

/// Converts a string representation of a mouse button to a `rdev::Button`.
/// Defaults to `Button::Left` if the string is unrecognized.
fn string_to_button(button_str: &str) -> Button {
    match button_str.to_lowercase().as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => Button::Left,
    }
}

/// Converts a string representation of a key to a `rdev::Key`.
/// This is a simplified mapping and can be expanded.
fn char_to_key(c: char) -> Key {
    match c {
        'a' => Key::KeyA, 'b' => Key::KeyB, 'c' => Key::KeyC, 'd' => Key::KeyD,
        'e' => Key::KeyE, 'f' => Key::KeyF, 'g' => Key::KeyG, 'h' => Key::KeyH,
        'i' => Key::KeyI, 'j' => Key::KeyJ, 'k' => Key::KeyK, 'l' => Key::KeyL,
        'm' => Key::KeyM, 'n' => Key::KeyN, 'o' => Key::KeyO, 'p' => Key::KeyP,
        'q' => Key::KeyQ, 'r' => Key::KeyR, 's' => Key::KeyS, 't' => Key::KeyT,
        'u' => Key::KeyU, 'v' => Key::KeyV, 'w' => Key::KeyW, 'x' => Key::KeyX,
        'y' => Key::KeyY, 'z' => Key::KeyZ,
        ' ' => Key::Space,
        // Add more mappings as needed
        _ => Key::Unknown(0),
    }
}



// --- Tauri Commands ---

#[tauri::command]
pub fn execute_mouse_move(x: f64, y: f64) -> Result<(), String> {
    move_mouse(x, y)
}

#[tauri::command]
pub fn execute_mouse_click(button_str: String) -> Result<(), String> {
    let button = string_to_button(&button_str);
    click(button)
}

#[tauri::command]
pub fn execute_key_press(key_char: char) -> Result<(), String> {
    let key = char_to_key(key_char);
    press_key(key)
}

#[tauri::command]
pub fn execute_key_release(key_char: char) -> Result<(), String> {
    let key = char_to_key(key_char);
    release_key(key)
}

#[tauri::command]
pub fn execute_type_string(text: String) -> Result<(), String> {
    type_string(&text)
}

/// A validation command to test that the I/O controller is working correctly.
/// When called, it moves the mouse to (100, 100) and types "hello".
#[tauri::command]
pub fn test_io() -> Result<(), String> {
    println!("Testing I/O controller...");
    move_mouse(100.0, 100.0)?;
    thread::sleep(time::Duration::from_millis(200)); // Pause for visibility
    type_string("hello")?;
    println!("Test complete.");
    Ok(())
}

