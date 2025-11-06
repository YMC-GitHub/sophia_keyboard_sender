// src/parser/command.rs
//! 命令解析功能
use crate::error::{KeyboardSenderError, Result};
#[allow(unused_imports)]
use crate::types::{Key, Modifier, WindowHandle};
use std::collections::HashMap;

#[cfg(feature = "command_parser")]
use lazy_static::lazy_static;
#[cfg(feature = "command_parser")]
use regex::Regex;

use sleep_utils::parse_sleep_duration;

/// 解析窗口句柄
pub fn parse_hwnd(hwnd_str: &str) -> Result<WindowHandle> {
    if hwnd_str.is_empty() {
        return Ok(0);
    }

    if let Some(stripped) = hwnd_str.strip_prefix("0x") {
        WindowHandle::from_str_radix(stripped, 16)
    } else {
        hwnd_str.parse()
    }
    .map_err(|_| KeyboardSenderError::InvalidWindowHandle(hwnd_str.to_string()))
}

/// 解析命令参数
pub fn parse_command_params(command: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    #[cfg(feature = "command_parser")]
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+):([^,]+)").unwrap();
        }

        for cap in RE.captures_iter(command) {
            params.insert(cap[1].to_string(), cap[2].to_string());
        }
    }

    params
}

/// 发送快捷键
pub fn shortcut(shortcut_str: &str) -> Result<()> {
    #[cfg(not(any(feature = "global", feature = "window_target")))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "global or window_target".to_string(),
    ));

    #[cfg(any(feature = "global", feature = "window_target"))]
    {
        use keyboard_codes::{parse_shortcut_with_aliases};
        
        let parsed = parse_shortcut_with_aliases(shortcut_str)
            .map_err(|e| KeyboardSenderError::ParseError(e.to_string()))?;

        // 使用现有的组合键功能
        crate::core::press_combination(&parsed.modifiers, parsed.key, None)
    }
}

/// 执行文本命令
pub fn send(command: &str) -> Result<()> {
    #[cfg(not(feature = "command_parser"))]
    return Err(KeyboardSenderError::FeatureNotEnabled("command_parser".to_string()));

    #[cfg(feature = "command_parser")]
    {
        use keyboard_codes::{parse_keyboard_input, KeyboardInput};

        let params = parse_command_params(command);

        let action = params.get("action").or_else(|| params.get("type"));
        let key_str = params.get("key");
        let char_str = params.get("char");
        let text_str = params.get("text");
        let shortcut_str = params.get("shortcut");
        let hwnd_str = params.get("hwnd").map(|s| s.as_str()).unwrap_or("0");
        let duration_str = params.get("duration");

        let hwnd = parse_hwnd(hwnd_str)?;
        let duration = duration_str.and_then(|dur| parse_sleep_duration(dur).ok());

        // 根据参数执行相应操作
        if let Some(shortcut_cmd) = shortcut_str {
            shortcut(shortcut_cmd)?;
        } else if let Some(action_type) = action {
            match action_type.as_str() {
                "key_down" | "keydown" => {
                    if let Some(key) = key_str {
                        let keyboard_input = parse_keyboard_input(key)
                            .map_err(|e| KeyboardSenderError::ParseError(e.to_string()))?;
                        
                        if let KeyboardInput::Key(key) = keyboard_input {
                            if hwnd == 0 {
                                #[cfg(feature = "global")]
                                crate::core::key_down(key)?;
                            } else {
                                #[cfg(feature = "window_target")]
                                crate::core::send_key_down_to_window(hwnd, key)?;
                            }
                        }
                    }
                }
                "key_up" | "keyup" => {
                    if let Some(key) = key_str {
                        let keyboard_input = parse_keyboard_input(key)
                            .map_err(|e| KeyboardSenderError::ParseError(e.to_string()))?;
                        
                        if let KeyboardInput::Key(key) = keyboard_input {
                            if hwnd == 0 {
                                #[cfg(feature = "global")]
                                crate::core::key_up(key)?;
                            } else {
                                #[cfg(feature = "window_target")]
                                crate::core::send_key_up_to_window(hwnd, key)?;
                            }
                        }
                    }
                }
                "key_click" | "keyclick" => {
                    if let Some(key) = key_str {
                        let keyboard_input = parse_keyboard_input(key)
                            .map_err(|e| KeyboardSenderError::ParseError(e.to_string()))?;
                        
                        if let KeyboardInput::Key(key) = keyboard_input {
                            if hwnd == 0 {
                                #[cfg(feature = "global")]
                                crate::core::key_click(key, duration)?;
                            } else {
                                #[cfg(feature = "window_target")]
                                crate::core::send_key_click_to_window(hwnd, key, duration)?;
                            }
                        }
                    }
                }
                "char" => {
                    if let Some(char_val) = char_str {
                        if let Some(c) = char_val.chars().next() {
                            if hwnd == 0 {
                                #[cfg(feature = "global")]
                                crate::core::send_char(c)?;
                            } else {
                                #[cfg(feature = "window_target")]
                                crate::core::send_char_to_window(hwnd, c)?;
                            }
                        }
                    }
                }
                "text" => {
                    if let Some(text) = text_str {
                        if hwnd == 0 {
                            #[cfg(feature = "global")]
                            crate::core::type_string(text)?;
                        } else {
                            #[cfg(feature = "window_target")]
                            crate::core::type_string_to_window(hwnd, text)?;
                        }
                    }
                }
                _ => return Err(KeyboardSenderError::CommandParseError(format!("Unknown action: {}", action_type)))
            }
        } else {
            // 向后兼容
            if let Some(key) = key_str {
                let keyboard_input = parse_keyboard_input(key)
                    .map_err(|e| KeyboardSenderError::ParseError(e.to_string()))?;
                
                if let KeyboardInput::Key(key) = keyboard_input {
                    if hwnd == 0 {
                        #[cfg(feature = "global")]
                        crate::core::key_click(key, duration)?;
                    } else {
                        #[cfg(feature = "window_target")]
                        crate::core::send_key_click_to_window(hwnd, key, duration)?;
                    }
                }
            } else if let Some(char_val) = char_str {
                if let Some(c) = char_val.chars().next() {
                    if hwnd == 0 {
                        #[cfg(feature = "global")]
                        crate::core::send_char(c)?;
                    } else {
                        #[cfg(feature = "window_target")]
                        crate::core::send_char_to_window(hwnd, c)?;
                    }
                }
            } else if let Some(text) = text_str {
                if hwnd == 0 {
                    #[cfg(feature = "global")]
                    crate::core::type_string(text)?;
                } else {
                    #[cfg(feature = "window_target")]
                    crate::core::type_string_to_window(hwnd, text)?;
                }
            } else if let Some(shortcut_cmd) = shortcut_str {
                shortcut(shortcut_cmd)?;
            } else {
                return Err(KeyboardSenderError::CommandParseError("No valid command found".to_string()));
            }
        }

        Ok(())
    }
}