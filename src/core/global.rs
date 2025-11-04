//! 全局键盘模拟功能
#[allow(unused_imports)]
use crate::error::{KeyboardSenderError, Result};
use crate::types::{Key, Modifier};
use std::time::Duration;

#[cfg(feature = "global")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, VIRTUAL_KEY,
};

use keyboard_codes::KeyCodeMapper;

/// 将 Key 转换为 Windows 虚拟键码（VK_CODE）
fn key_to_vk(key: Key) -> u16 {
    key.to_code(keyboard_codes::current_platform()) as u16
}

/// 将 Modifier 转换为对应的 Key
pub fn modifier_to_key(modifier: Modifier) -> Key {
    use std::str::FromStr;

    match modifier {
        Modifier::Shift | Modifier::LeftShift | Modifier::RightShift => {
            Key::from_str("Shift").unwrap_or(Key::Escape)
        }
        Modifier::Control | Modifier::LeftControl | Modifier::RightControl => {
            Key::from_str("Control").unwrap_or(Key::Escape)
        }
        Modifier::Alt | Modifier::LeftAlt | Modifier::RightAlt => {
            Key::from_str("Alt").unwrap_or(Key::Escape)
        }
        Modifier::Meta | Modifier::LeftMeta | Modifier::RightMeta => {
            Key::from_str("Meta").unwrap_or(Key::Escape)
        }
    }
}

/// 全局发送：按键按下（模拟物理键盘，发送到当前焦点窗口）
pub fn key_down(key: Key) -> Result<()> {
    #[cfg(not(feature = "global"))]
    return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));

    #[cfg(feature = "global")]
    {
        let vk = key_to_vk(key);

        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk),
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0), // 按下标志
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
        Ok(())
    }
}

/// 全局发送：按键释放（模拟物理键盘）
pub fn key_up(key: Key) -> Result<()> {
    #[cfg(not(feature = "global"))]
    return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));

    #[cfg(feature = "global")]
    {
        let vk = key_to_vk(key);

        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk),
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP, // 释放标志
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        unsafe {
            SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
        Ok(())
    }
}

/// 全局发送：按键点击（按下 + 释放，带可选延迟）
pub fn key_click(key: Key, press_duration: Option<Duration>) -> Result<()> {
    key_down(key)?;

    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    key_up(key)?;
    Ok(())
}

/// 全局发送：字符输入（通过 Unicode 编码，支持任意字符）
pub fn send_char(c: char) -> Result<()> {
    #[cfg(not(feature = "global"))]
    return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));

    #[cfg(feature = "global")]
    {
        // 按下字符（Unicode 模式）
        let input_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),        // Unicode 模式不使用虚拟键码
                    wScan: c as u16,            // 字符的 Unicode 编码
                    dwFlags: KEYEVENTF_UNICODE, // Unicode 标志
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        // 释放字符
        let input_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP, // 释放 + Unicode 标志
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        unsafe {
            SendInput(&[input_down, input_up], std::mem::size_of::<INPUT>() as i32);
        }
        Ok(())
    }
}

/// 全局发送：字符串输入（逐个字符发送）
pub fn type_string(text: &str) -> Result<()> {
    for c in text.chars() {
        send_char(c)?;
    }
    Ok(())
}

/// 全局发送：组合键（修饰符 + 按键，带延迟控制）
pub fn press_combination(
    modifiers: &[Modifier],
    key: Key,
    press_duration: Option<Duration>,
) -> Result<()> {
    // 按下所有修饰键
    for &modifier in modifiers {
        let mod_key = modifier_to_key(modifier);
        key_down(mod_key)?;
    }

    // 按下主按键
    key_down(key)?;

    // 等待指定延迟（按键保持按下状态）
    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    // 释放主按键
    key_up(key)?;

    // 反向释放修饰键（避免影响后续操作）
    for &modifier in modifiers.iter().rev() {
        let mod_key = modifier_to_key(modifier);
        key_up(mod_key)?;
    }

    Ok(())
}
