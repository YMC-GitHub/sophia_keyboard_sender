// src/smart/implementation.rs
use crate::error::Result;
use keyboard_codes::{parse_keyboard_input, parse_shortcut_with_aliases, KeyboardInput};
use std::time::Duration;
use std::str::FromStr;

/// 智能输入 - 自动检测输入类型并分派到合适的函数
///
/// # 支持的输入类型
/// - 文本: `"Hello World"` → 字符串输入
/// - 单键: `"a"`, `"enter"`, `"space"` → 按键点击  
/// - 快捷键: `"ctrl+c"`, `"shift+a"` → 组合键
/// - 修饰符: `"ctrl"`, `"shift"` → 修饰键点击
///
/// # 示例
/// ```
/// # #[cfg(feature = "smart")]
/// # {
/// use sophia_keyboard_sender::smart::type_auto;
///
/// type_auto("Hello World").unwrap();  // 文本输入
/// type_auto("enter").unwrap();        // 回车键
/// type_auto("ctrl+s").unwrap();       // 保存快捷键
/// type_auto("a").unwrap();            // 字母键
/// # }
/// ```
pub fn type_auto(input: &str) -> Result<()> {
    if input.is_empty() {
        return Ok(());
    }

    // 1. 检测并处理快捷键格式 (包含+号)
    if input.contains('+') {
        return type_shortcut_auto(input);
    }

    // 2. 检测单字符（字母、数字、空格等）
    if let Some(c) = detect_single_char(input) {
        return type_char_auto(c);
    }

    // 3. 尝试解析为键盘输入
    if let Ok(keyboard_input) = parse_keyboard_input(input) {
        return match keyboard_input {
            KeyboardInput::Key(key) => crate::core::key_click(key, None),
            KeyboardInput::Modifier(modifier) => {
                // 修饰符转为对应的键进行点击
                let key = crate::core::modifier_to_key(modifier);
                crate::core::key_click(key, None)
            }
        };
    }

    // 4. 默认作为文本处理
    crate::core::type_string(input)
}

/// 批量智能输入多个指令
///
/// # 示例
/// ```
/// # #[cfg(feature = "smart")]
/// # {
/// use sophia_keyboard_sender::smart::type_multiple;
///
/// type_multiple(&["Hello", "tab", "World", "enter"]).unwrap();
/// type_multiple(&["ctrl+a", "ctrl+c", "ctrl+v"]).unwrap();
/// # }
/// ```
pub fn type_multiple(inputs: &[&str]) -> Result<()> {
    for (i, input) in inputs.iter().enumerate() {
        type_auto(input)?;

        // 在指令之间添加小延迟（除了最后一个）
        if i < inputs.len() - 1 {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    Ok(())
}

/// 智能快捷键输入
fn type_shortcut_auto(shortcut: &str) -> Result<()> {
    // 首先尝试完整的快捷键解析
    if let Ok(parsed) = parse_shortcut_with_aliases(shortcut) {
        return crate::core::press_combination(&parsed.modifiers, parsed.key, None);
    }

    // 回退到简单的+分割解析
    let parts: Vec<&str> = shortcut.split('+').collect();
    if parts.len() >= 2 {
        let mut modifiers = Vec::new();
        let mut main_key = None;

        for part in parts {
            if let Ok(keyboard_input) = parse_keyboard_input(part) {
                match keyboard_input {
                    KeyboardInput::Modifier(modifier) => modifiers.push(modifier),
                    KeyboardInput::Key(key) => main_key = Some(key),
                }
            }
        }

        if let Some(key) = main_key {
            return crate::core::press_combination(&modifiers, key, None);
        }
    }

    // 如果都无法解析，作为文本处理（虽然不太可能）
    crate::core::type_string(shortcut)
}

/// 智能字符输入
fn type_char_auto(c: char) -> Result<()> {
    match c {
        // 字母 - 转为大写键名
        'a'..='z' | 'A'..='Z' => {
            let key_name = c.to_uppercase().to_string();
            if let Ok(key) = keyboard_codes::Key::from_str(&key_name) {
                crate::core::key_click(key, None)
            } else {
                crate::core::send_char(c)
            }
        }
        // 数字 - 转为D0-D9格式
        '0'..='9' => {
            let key_name = format!("D{}", c);
            if let Ok(key) = keyboard_codes::Key::from_str(&key_name) {
                crate::core::key_click(key, None)
            } else {
                crate::core::send_char(c)
            }
        }
        // 特殊字符直接发送
        ' ' => crate::core::key_click(crate::types::Key::Space, None),
        '\n' => crate::core::key_click(crate::types::Key::Enter, None),
        '\t' => crate::core::key_click(crate::types::Key::Tab, None),
        _ => crate::core::send_char(c),
    }
}

/// 检测是否为单字符输入
fn detect_single_char(input: &str) -> Option<char> {
    // case: empty char is empty char

    // case: empty char is space
    // // 特殊处理：空格字符
    // if input == " " {
    //   return Some(' ');
    // }

    let trimmed = input.trim();

    // 单字符且不是已知的特殊键名
    if trimmed.chars().count() == 1 {
        let c = trimmed.chars().next().unwrap();
        // 排除看起来像特殊键名的单字符
        if !is_special_key_name(trimmed) {
            return Some(c);
        }
    }

    None
}

/// 检查是否为特殊键名
fn is_special_key_name(input: &str) -> bool {
    matches!(
        input.to_lowercase().as_str(),
        "enter"
            | "space"
            | "tab"
            | "esc"
            | "escape"
            | "backspace"
            | "delete"
            | "insert"
            | "home"
            | "end"
            | "pageup"
            | "pagedown"
            | "up"
            | "down"
            | "left"
            | "right"
            | "f1"
            | "f2"
            | "f3"
            | "f4"
            | "f5"
            | "f6"
            | "f7"
            | "f8"
            | "f9"
            | "f10"
            | "f11"
            | "f12"
    )
}

/// 带延迟控制的智能输入
///
/// # 示例
/// ```
/// # #[cfg(feature = "smart")]
/// # {
/// use sophia_keyboard_sender::smart::type_with_delay;
/// use std::time::Duration;
///
/// // 每个字符间隔50ms
/// type_with_delay("Hello", Duration::from_millis(50)).unwrap();
/// # }
/// ```
pub fn type_with_delay(text: &str, delay: Duration) -> Result<()> {
    for c in text.chars() {
        type_char_auto(c)?;
        std::thread::sleep(delay);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_single_char() {
        assert_eq!(detect_single_char("a"), Some('a'));
        assert_eq!(detect_single_char("1"), Some('1'));
        //assert_eq!(detect_single_char(" "), Some(' '));
        assert_eq!(detect_single_char(" "), None);
        assert_eq!(detect_single_char("enter"), None); // 特殊键名
        assert_eq!(detect_single_char("abc"), None); // 多字符
    }

    #[test]
    fn test_is_special_key_name() {
        assert!(is_special_key_name("enter"));
        assert!(is_special_key_name("ENTER"));
        assert!(is_special_key_name("space"));
        assert!(is_special_key_name("f1"));
        assert!(!is_special_key_name("a"));
        assert!(!is_special_key_name("1"));
        assert!(!is_special_key_name("abc"));
    }
}
