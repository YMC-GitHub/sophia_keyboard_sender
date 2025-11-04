# Sophia Keyboard Sender

English | [中文](README.md)

A powerful, cross-platform keyboard event sender with window targeting, delay control, and command parsing capabilities. Built on Windows API and `keyboard-codes` crate.

## Features

- 🎯 **Window Targeting** - Send keyboard events to specific windows
- ⏱️ **Precise Delay Control** - Configurable delay for key press and release
- ⌨️ **Full Keyboard Support** - Support for standard keys, function keys, combinations, etc.
- 🌐 **Unicode Support** - Support for any character input
- 🔧 **Dual Send Modes** - Both global simulation and window message sending
- 📝 **Command Parser** - Execute keyboard operations via text commands
- 🚀 **High Performance** - Built on native Windows API, low latency and high precision
- 🔌 **Modular Design** - Enable only the features you need

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sophia_keyboard_sender = { git = "https://github.com/ymc-github/sophia_keyboard_sender", branch = "main" }
```

### Feature Flags

- `global` - Global keyboard simulation (enabled by default)
- `window_target` - Window targeting capabilities (enabled by default)  
- `command_parser` - Text command parsing (enabled by default)
- `convenience` - Convenience shortcut functions
- `full` - All features enabled

Minimal configuration:
```toml
sophia_keyboard_sender = { git = "...", default-features = false, features = ["global"] }
```

## Quick Start

### Basic Usage

```rust
use sophia_keyboard_sender::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Send single key press
    key_click(Key::A, None)?;

    // Send character
    send_char('!')?;

    // Send string
    type_string("Hello, World!")?;

    // Send key combination (Ctrl+C)
    press_combination(&[Modifier::Control], Key::C, None)?;

    Ok(())
}
```

### Command Parser (Recommended)

```rust
use sophia_keyboard_sender::send;

// Basic key operations
send("key:a")?;                    // Press and release A key
send("key:enter")?;                // Press Enter key
send("char:!")?;                   // Send exclamation mark
send("text:hello world")?;         // Type "hello world"

// Shortcuts
send("shortcut:ctrl+c")?;          // Ctrl+C
send("shortcut:alt+f4")?;          // Alt+F4
send("shortcut:ctrl+shift+esc")?;  // Ctrl+Shift+Esc

// Advanced options
send("key:a,duration:100ms")?;     // Hold key for 100ms
send("key:a,hwnd:123456")?;        // Send to specific window
send("text:test,hwnd:0x1A2B,duration:10ms")?;
```

### Window Targeting

```rust
use sophia_keyboard_sender::*;

// Send to specific window
send_key_click_to_window(123456, Key::Enter, None)?;

// Type text to specific window  
type_string_to_window(123456, "Text to specific window")?;

// Control window focus
set_window_focus(123456, true)?;  // Bring to top and focus
```

### Convenience Functions

```rust
use sophia_keyboard_sender::*;

send_tab()?;        // Send Tab key
send_enter()?;      // Send Enter key  
send_escape()?;     // Send Escape key
send_space()?;      // Send Space key
send_backspace()?;  // Send Backspace key
send_delete()?;     // Send Delete key
```

## API Reference

### Core Functions

#### Global Keyboard Simulation
- `key_down(key: Key)` - Press key down
- `key_up(key: Key)` - Release key up  
- `key_click(key: Key, duration: Option<Duration>)` - Click key with optional hold duration
- `send_char(c: char)` - Send character (Unicode supported)
- `type_string(text: &str)` - Type string
- `press_combination(modifiers: &[Modifier], key: Key, duration: Option<Duration>)` - Send key combination

#### Window Targeting
- `send_key_down_to_window(hwnd: WindowHandle, key: Key)` - Send key down to window
- `send_key_up_to_window(hwnd: WindowHandle, key: Key)` - Send key up to window
- `send_key_click_to_window(hwnd: WindowHandle, key: Key, duration: Option<Duration>)` - Send key click to window
- `send_char_to_window(hwnd: WindowHandle, c: char)` - Send character to window
- `type_string_to_window(hwnd: WindowHandle, text: &str)` - Type string to window
- `set_window_focus(hwnd: WindowHandle, bring_to_top: bool)` - Control window focus

#### Command Parser
- `send(command: &str)` - Execute text command
- `shortcut(shortcut: &str)` - Send keyboard shortcut
- `parse_duration(duration_str: &str)` - Parse duration string
- `parse_key(key_str: &str)` - Parse key name
- `parse_modifier(modifier_str: &str)` - Parse modifier name

### Supported Key Names

#### Alphabet Keys
```rust
"a", "b", "c", ..., "z"  // Maps to Key::A, Key::B, etc.
```

#### Number Keys  
```rust
"0", "1", "2", ..., "9"  // Maps to Key::D0, Key::D1, etc.
```

#### Special Keys
```rust
"enter", "space", "tab", "escape", "backspace", "delete",
"insert", "home", "end", "pageup", "pagedown"
```

#### Arrow Keys
```rust
"up", "down", "left", "right"  // Maps to ArrowUp, ArrowDown, etc.
```

#### Function Keys
```rust
"f1", "f2", ..., "f12"  // Maps to F1, F2, etc.
```

#### Modifier Keys
```rust
"ctrl", "shift", "alt", "meta",
"leftctrl", "rightctrl", "leftshift", "rightshift",
"leftalt", "rightalt", "leftmeta", "rightmeta"
```

## Command Syntax

### Basic Format
```
key:value,key2:value2,key3:value3
```

### Supported Commands

#### Key Operations
```rust
"key:a"                          // Click A key
"key:enter,duration:100ms"       // Hold Enter for 100ms
"key:a,hwnd:123456"              // Send to window 123456
```

#### Character Operations
```rust
"char:!"                         // Send exclamation mark
"char:!,hwnd:0x1A2B"             // Send to specific window
```

#### Text Operations  
```rust
"text:hello world"               // Type text
"text:test,hwnd:123456"          // Type to specific window
"text:hello,duration:10ms"       // Type with delay between chars
```

#### Shortcut Operations
```rust
"shortcut:ctrl+c"                // Ctrl+C
"shortcut:alt+tab"               // Alt+Tab
"shortcut:ctrl+shift+escape"     // Ctrl+Shift+Escape
```

#### Action-based Format
```rust
"action:key_click,key:a"         // Click A key
"action:char,char:!"             // Send character
"action:text,text:hello"         // Type text
"action:key_down,key:shift"      // Press Shift down
"action:key_up,key:shift"        // Release Shift up
```

### Duration Format
- `"100ms"` - 100 milliseconds
- `"2s"` - 2 seconds
- `"500ms"` - 500 milliseconds

### Window Handle Format
- `"123456"` - Decimal window handle
- `"0x1A2B"` - Hexadecimal window handle  
- `"0"` or `""` - Current foreground window

## Use Cases

### Automated Testing
```rust
// Automated form filling
send("text:John Doe")?;
send("key:tab")?;
send("text:john@example.com")?;
send("key:tab")?;
send("text:password123")?;
send("key:enter")?;
```

### Game Automation
```rust
// Game controls with timing
send("key:w,duration:200ms")?;    // Move forward
send("key:space")?;               // Jump
send("shortcut:ctrl+1")?;         // Use item 1
```

### Remote Control
```rust
// Control remote application
set_window_focus(remote_window, true)?;
send("text:git status")?;
send("key:enter")?;
```

### Macro Recording
```rust
// Execute recorded macros
send("shortcut:win+r")?;
send("text:notepad")?;
send("key:enter,duration:500ms")?;
send("text:Hello from macro!")?;
```

## Error Handling

All functions return `Result<T, KeyboardSenderError>`:

```rust
#[derive(Error, Debug)]
pub enum KeyboardSenderError {
    ParseError(String),
    UnsupportedKey(String),
    UnsupportedModifier(String), 
    InvalidDuration(String),
    InvalidWindowHandle(String),
    CommandParseError(String),
    FeatureNotEnabled(String),
    WindowsError,
}
```

## Platform Support

Currently supports **Windows** platform. Linux and macOS support planned for future releases.

## License

This project is licensed under either MIT OR Apache-2.0 dual license.

## Contributing

Issues and Pull Requests are welcome!

## Related Projects

- [keyboard-codes](https://github.com/ymc-github/keyboard-codes) - Cross-platform keyboard key code mapping library
