//! 命令解析功能

use crate::error::{KeyboardSenderError, Result};
use crate::types::{Key, Modifier, WindowHandle};
use std::collections::HashMap;
use std::str::FromStr;

#[cfg(feature = "command_parser")]
use lazy_static::lazy_static;
#[cfg(feature = "command_parser")]
use regex::Regex;

use super::duration::parse_duration;

/// 解析键名 - 支持用户友好的键名
pub fn parse_key(key_str: &str) -> Result<Key> {
    // 将用户友好的键名映射到标准键名
    let normalized = match key_str.to_lowercase().as_str() {
        // 字母键
        "a" => "A",
        "b" => "B",
        "c" => "C",
        "d" => "D",
        "e" => "E",
        "f" => "F",
        "g" => "G",
        "h" => "H",
        "i" => "I",
        "j" => "J",
        "k" => "K",
        "l" => "L",
        "m" => "M",
        "n" => "N",
        "o" => "O",
        "p" => "P",
        "q" => "Q",
        "r" => "R",
        "s" => "S",
        "t" => "T",
        "u" => "U",
        "v" => "V",
        "w" => "W",
        "x" => "X",
        "y" => "Y",
        "z" => "Z",

        // 数字键
        "0" => "D0",
        "1" => "D1",
        "2" => "D2",
        "3" => "D3",
        "4" => "D4",
        "5" => "D5",
        "6" => "D6",
        "7" => "D7",
        "8" => "D8",
        "9" => "D9",

        // 特殊键
        "enter" => "Enter",
        "space" => "Space",
        "tab" => "Tab",
        "escape" => "Escape",
        "backspace" => "Backspace",
        "delete" => "Delete",
        "insert" => "Insert",
        "home" => "Home",
        "end" => "End",
        "pageup" => "PageUp",
        "pagedown" => "PageDown",

        // 方向键
        "up" => "ArrowUp",
        "down" => "ArrowDown",
        "left" => "ArrowLeft",
        "right" => "ArrowRight",

        // 功能键
        "f1" => "F1",
        "f2" => "F2",
        "f3" => "F3",
        "f4" => "F4",
        "f5" => "F5",
        "f6" => "F6",
        "f7" => "F7",
        "f8" => "F8",
        "f9" => "F9",
        "f10" => "F10",
        "f11" => "F11",
        "f12" => "F12",

        // 如果已经是标准格式，直接使用
        _ => key_str,
    };

    Key::from_str(normalized).map_err(|_| KeyboardSenderError::UnsupportedKey(key_str.to_string()))
}

/// 解析修饰符 - 支持用户友好的修饰符名
pub fn parse_modifier(modifier_str: &str) -> Result<Modifier> {
    // 将用户友好的修饰符名映射到标准修饰符名
    let normalized = match modifier_str.to_lowercase().as_str() {
        "ctrl" | "control" => "Control",
        "shift" => "Shift",
        "alt" => "Alt",
        "meta" | "win" | "windows" | "cmd" | "command" => "Meta",
        "leftctrl" | "leftcontrol" => "LeftControl",
        "rightctrl" | "rightcontrol" => "RightControl",
        "leftshift" => "LeftShift",
        "rightshift" => "RightShift",
        "leftalt" => "LeftAlt",
        "rightalt" => "RightAlt",
        "leftmeta" | "leftwin" | "leftwindows" | "leftcmd" => "LeftMeta",
        "rightmeta" | "rightwin" | "rightwindows" | "rightcmd" => "RightMeta",

        // 如果已经是标准格式，直接使用
        _ => modifier_str,
    };

    Modifier::from_str(normalized)
        .map_err(|_| KeyboardSenderError::UnsupportedModifier(modifier_str.to_string()))
}

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

/// 发送快捷键（如 "ctrl+q", "alt+f4"）
pub fn shortcut(shortcut_str: &str) -> Result<()> {
    #[cfg(not(any(feature = "global", feature = "window_target")))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "global or window_target".to_string(),
    ));

    #[cfg(any(feature = "global", feature = "window_target"))]
    {
        let parts: Vec<&str> = shortcut_str.split('+').collect();
        if parts.is_empty() {
            return Err(KeyboardSenderError::ParseError(
                "Empty shortcut".to_string(),
            ));
        }

        let mut modifiers = Vec::new();
        let mut main_key = None;

        for part in parts {
            let part_lower = part.to_lowercase();
            if let Ok(modifier) = parse_modifier(&part_lower) {
                modifiers.push(modifier);
            } else if let Ok(key) = parse_key(&part_lower) {
                main_key = Some(key);
            } else {
                return Err(KeyboardSenderError::UnsupportedKey(part.to_string()));
            }
        }

        if let Some(key) = main_key {
            // 使用条件编译来调用正确的函数
            #[cfg(feature = "global")]
            return crate::core::press_combination(&modifiers, key, None);

            #[cfg(all(not(feature = "global"), feature = "window_target"))]
            {
                // 对于窗口目标，需要分别处理修饰键
                for &modifier in &modifiers {
                    let mod_key = crate::core::modifier_to_key(modifier);
                    crate::core::send_key_down_to_window(0, mod_key)?;
                }

                crate::core::send_key_click_to_window(0, key, None)?;

                for &modifier in modifiers.iter().rev() {
                    let mod_key = crate::core::modifier_to_key(modifier);
                    crate::core::send_key_up_to_window(0, mod_key)?;
                }

                Ok(())
            }
        } else {
            Err(KeyboardSenderError::ParseError(
                "No main key found in shortcut".to_string(),
            ))
        }
    }
}

/// 执行文本命令
pub fn send(command: &str) -> Result<()> {
    #[cfg(not(feature = "command_parser"))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "command_parser".to_string(),
    ));

    #[cfg(feature = "command_parser")]
    {
        let params = parse_command_params(command);

        let action = params.get("action").or_else(|| params.get("type"));
        let key_str = params.get("key");
        let char_str = params.get("char");
        let text_str = params.get("text");
        let shortcut_str = params.get("shortcut");
        let hwnd_str = params.get("hwnd").map(|s| s.as_str()).unwrap_or("0");
        let duration_str = params.get("duration");

        let hwnd = parse_hwnd(hwnd_str)?;
        let duration = if let Some(dur) = duration_str {
            Some(parse_duration(dur)?)
        } else {
            None
        };

        // 根据参数执行相应操作
        if let Some(shortcut_cmd) = shortcut_str {
            shortcut(shortcut_cmd)?;
        } else if let Some(action_type) = action {
            match action_type.as_str() {
                "key_down" | "keydown" => {
                    if let Some(key) = key_str {
                        let key = parse_key(key)?;
                        if hwnd == 0 {
                            #[cfg(feature = "global")]
                            crate::core::key_down(key)?;
                            #[cfg(not(feature = "global"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "global".to_string(),
                            ));
                        } else {
                            #[cfg(feature = "window_target")]
                            crate::core::send_key_down_to_window(hwnd, key)?;
                            #[cfg(not(feature = "window_target"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "window_target".to_string(),
                            ));
                        }
                    } else {
                        return Err(KeyboardSenderError::CommandParseError(
                            "Missing 'key' parameter for key_down action".to_string(),
                        ));
                    }
                }
                "key_up" | "keyup" => {
                    if let Some(key) = key_str {
                        let key = parse_key(key)?;
                        if hwnd == 0 {
                            #[cfg(feature = "global")]
                            crate::core::key_up(key)?;
                            #[cfg(not(feature = "global"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "global".to_string(),
                            ));
                        } else {
                            #[cfg(feature = "window_target")]
                            crate::core::send_key_up_to_window(hwnd, key)?;
                            #[cfg(not(feature = "window_target"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "window_target".to_string(),
                            ));
                        }
                    } else {
                        return Err(KeyboardSenderError::CommandParseError(
                            "Missing 'key' parameter for key_up action".to_string(),
                        ));
                    }
                }
                "key_click" | "keyclick" => {
                    if let Some(key) = key_str {
                        let key = parse_key(key)?;
                        if hwnd == 0 {
                            #[cfg(feature = "global")]
                            crate::core::key_click(key, duration)?;
                            #[cfg(not(feature = "global"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "global".to_string(),
                            ));
                        } else {
                            #[cfg(feature = "window_target")]
                            crate::core::send_key_click_to_window(hwnd, key, duration)?;
                            #[cfg(not(feature = "window_target"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "window_target".to_string(),
                            ));
                        }
                    } else {
                        return Err(KeyboardSenderError::CommandParseError(
                            "Missing 'key' parameter for key_click action".to_string(),
                        ));
                    }
                }
                "char" => {
                    if let Some(char_val) = char_str {
                        if let Some(c) = char_val.chars().next() {
                            if hwnd == 0 {
                                #[cfg(feature = "global")]
                                crate::core::send_char(c)?;
                                #[cfg(not(feature = "global"))]
                                return Err(KeyboardSenderError::FeatureNotEnabled(
                                    "global".to_string(),
                                ));
                            } else {
                                #[cfg(feature = "window_target")]
                                crate::core::send_char_to_window(hwnd, c)?;
                                #[cfg(not(feature = "window_target"))]
                                return Err(KeyboardSenderError::FeatureNotEnabled(
                                    "window_target".to_string(),
                                ));
                            }
                        } else {
                            return Err(KeyboardSenderError::CommandParseError(
                                "Invalid 'char' parameter".to_string(),
                            ));
                        }
                    } else {
                        return Err(KeyboardSenderError::CommandParseError(
                            "Missing 'char' parameter for char action".to_string(),
                        ));
                    }
                }
                "text" => {
                    if let Some(text) = text_str {
                        if hwnd == 0 {
                            #[cfg(feature = "global")]
                            crate::core::type_string(text)?;
                            #[cfg(not(feature = "global"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "global".to_string(),
                            ));
                        } else {
                            #[cfg(feature = "window_target")]
                            crate::core::type_string_to_window(hwnd, text)?;
                            #[cfg(not(feature = "window_target"))]
                            return Err(KeyboardSenderError::FeatureNotEnabled(
                                "window_target".to_string(),
                            ));
                        }
                    } else {
                        return Err(KeyboardSenderError::CommandParseError(
                            "Missing 'text' parameter for text action".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(KeyboardSenderError::CommandParseError(format!(
                        "Unknown action: {}",
                        action_type
                    )))
                }
            }
        } else {
            // 向后兼容：如果没有 action 参数，尝试根据其他参数推断
            if let Some(key) = key_str {
                let key = parse_key(key)?;
                if hwnd == 0 {
                    #[cfg(feature = "global")]
                    crate::core::key_click(key, duration)?;
                    #[cfg(not(feature = "global"))]
                    return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));
                } else {
                    #[cfg(feature = "window_target")]
                    crate::core::send_key_click_to_window(hwnd, key, duration)?;
                    #[cfg(not(feature = "window_target"))]
                    return Err(KeyboardSenderError::FeatureNotEnabled(
                        "window_target".to_string(),
                    ));
                }
            } else if let Some(char_val) = char_str {
                if let Some(c) = char_val.chars().next() {
                    if hwnd == 0 {
                        #[cfg(feature = "global")]
                        crate::core::send_char(c)?;
                        #[cfg(not(feature = "global"))]
                        return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));
                    } else {
                        #[cfg(feature = "window_target")]
                        crate::core::send_char_to_window(hwnd, c)?;
                        #[cfg(not(feature = "window_target"))]
                        return Err(KeyboardSenderError::FeatureNotEnabled(
                            "window_target".to_string(),
                        ));
                    }
                } else {
                    return Err(KeyboardSenderError::CommandParseError(
                        "Invalid 'char' parameter".to_string(),
                    ));
                }
            } else if let Some(text) = text_str {
                if hwnd == 0 {
                    #[cfg(feature = "global")]
                    crate::core::type_string(text)?;
                    #[cfg(not(feature = "global"))]
                    return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));
                } else {
                    #[cfg(feature = "window_target")]
                    crate::core::type_string_to_window(hwnd, text)?;
                    #[cfg(not(feature = "window_target"))]
                    return Err(KeyboardSenderError::FeatureNotEnabled(
                        "window_target".to_string(),
                    ));
                }
            } else if let Some(shortcut_cmd) = shortcut_str {
                shortcut(shortcut_cmd)?;
            } else {
                return Err(KeyboardSenderError::CommandParseError(
                    "No valid command found".to_string(),
                ));
            }
        }

        Ok(())
    }
}
