# Sophia Keyboard Sender

English | [中文](README.md)

A keyboard event sender with window targeting and delay control, built on Windows API and `keyboard-codes` crate.

## Features

- 🎯 **Window Targeting** - Send keyboard events to specific windows
- ⏱️ **Precise Delay Control** - Configurable delay for key press and release
- ⌨️ **Full Keyboard Support** - Support for standard keys, function keys, combinations, etc.
- 🌐 **Unicode Support** - Support for any character input
- 🔧 **Dual Send Modes** - Both global simulation and window message sending
- 🚀 **High Performance** - Built on native Windows API, low latency and high precision

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sophia_keyboard_sender = { git = "https://github.com/ymc-github/sophia_keyboard_sender", branch = "main" }
```

## Quick Start

### Basic Usage

```rust
use sophia_keyboard_sender::*;
use keyboard_codes::{Key, Modifier};
use std::time::Duration;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send single key press
    let key_a = Key::from_str("A").unwrap();
    key_click(key_a, None)?;

    // Send character
    send_char('!')?;

    // Send string
    type_string("Hello, World!")?;

    // Send key combination (Ctrl+C)
    let ctrl = Modifier::from_str("Control").unwrap();
    let key_c = Key::from_str("C").unwrap();
    press_combination(&[ctrl], key_c, None)?;

    Ok(())
}
```

### Key Operations with Delay

```rust
// Press and hold for 100ms before release
key_click(Key::from_str("Enter").unwrap(), Some(Duration::from_millis(100)))?;

// Hold combination for 500ms
press_combination(
    &[Modifier::from_str("Control").unwrap()], 
    Key::from_str("V").unwrap(), 
    Some(Duration::from_millis(500))
)?;
```

### Send Events to Specific Window

```rust
use windows::Win32::Foundation::HWND;

// Assume you have a window handle
let hwnd = HWND(0x123456); // Replace with actual window handle

// Activate window
set_window_focus(hwnd, true)?;

// Send key to specific window
send_key_click_to_window(hwnd, Key::from_str("Enter").unwrap(), None)?;

// Type text to specific window
type_string_to_window(hwnd, "Text input to specific window")?;
```

## API Reference

### Global Keyboard Events (Physical Keyboard Simulation)

- `key_down(key: Key)` - Press key down
- `key_up(key: Key)` - Release key up  
- `key_click(key: Key, duration: Option<Duration>)` - Click key (with optional hold duration)
- `send_char(c: char)` - Send character (Unicode supported)
- `type_string(text: &str)` - Type string
- `press_combination(modifiers: &[Modifier], key: Key, duration: Option<Duration>)` - Send key combination

### Window-Targeted Keyboard Events (via Window Messages)

- `send_key_down_to_window(hwnd: HWND, key: Key)` - Send key down to window
- `send_key_up_to_window(hwnd: HWND, key: Key)` - Send key up to window
- `send_key_click_to_window(hwnd: HWND, key: Key, duration: Option<Duration>)` - Send key click to window
- `send_char_to_window(hwnd: HWND, c: char)` - Send character to window
- `type_string_to_window(hwnd: HWND, text: &str)` - Type string to window
- `set_window_focus(hwnd: HWND, bring_to_top: bool)` - Control window focus

## Keyboard Key Support

This library supports full keyboard keys through `keyboard-codes` crate:

### Alphabet Keys
```rust
Key::from_str("A").unwrap()  // A key
Key::from_str("Z").unwrap()  // Z key
```

### Number Keys
```rust
Key::from_str("D0").unwrap() // Main keyboard 0
Key::from_str("Num0").unwrap() // Numpad 0
```

### Function Keys
```rust
Key::from_str("F1").unwrap()  // F1 key
Key::from_str("F12").unwrap() // F12 key
```

### Control Keys
```rust
Key::from_str("Enter").unwrap()    // Enter key
Key::from_str("Escape").unwrap()   // ESC key
Key::from_str("Space").unwrap()    // Space key
Key::from_str("Tab").unwrap()      // Tab key
```

### Arrow Keys
```rust
Key::from_str("ArrowUp").unwrap()    // Up arrow
Key::from_str("ArrowDown").unwrap()  // Down arrow
Key::from_str("ArrowLeft").unwrap()  // Left arrow  
Key::from_str("ArrowRight").unwrap() // Right arrow
```

### Modifier Keys
```rust
Modifier::from_str("Shift").unwrap()     // Shift key
Modifier::from_str("Control").unwrap()   // Ctrl key
Modifier::from_str("Alt").unwrap()       // Alt key
Modifier::from_str("Meta").unwrap()      // Windows key
```

## Use Cases

### Automated Testing
```rust
// Automated form filling
type_string("Test User")?;
key_click(Key::from_str("Tab").unwrap(), None)?;
type_string("test@example.com")?;
key_click(Key::from_str("Enter").unwrap(), None)?;
```

### Game Automation
```rust
// Game shortcuts
press_combination(
    &[Modifier::from_str("Control").unwrap()],
    Key::from_str("Q").unwrap(),
    Some(Duration::from_millis(100))
)?;
```

### Remote Control
```rust
// Send control commands to remote window
set_window_focus(remote_window, true)?;
type_string_to_window(remote_window, "Execute command")?;
key_click_to_window(remote_window, Key::from_str("Enter").unwrap(), None)?;
```

## Important Notes

1. **Administrator Privileges**: Some operations may require administrator privileges
2. **Window Focus**: When sending messages to specific windows, ensure the window exists and can receive messages
3. **Delay Settings**: Reasonable delay settings can improve operation reliability
4. **Error Handling**: All functions return `Result`, error handling is recommended

## License

This project is licensed under either MIT OR Apache-2.0 dual license.

## Contributing

Issues and Pull Requests are welcome!

## Related Projects

- [keyboard-codes](https://github.com/ymc-github/keyboard-codes) - Cross-platform keyboard key code mapping library