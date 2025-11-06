// src/core/global.rs
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

use super::{key_to_vk, modifier_to_key};

/// 全局发送：按键按下
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
                    dwFlags: KEYBD_EVENT_FLAGS(0),
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

/// 全局发送：按键释放
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
                    dwFlags: KEYEVENTF_KEYUP,
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

/// 全局发送：按键点击
pub fn key_click(key: Key, press_duration: Option<Duration>) -> Result<()> {
    key_down(key)?;

    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    key_up(key)?;
    Ok(())
}

/// 全局发送：字符输入
pub fn send_char(c: char) -> Result<()> {
    #[cfg(not(feature = "global"))]
    return Err(KeyboardSenderError::FeatureNotEnabled("global".to_string()));

    #[cfg(feature = "global")]
    {
        let input_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        let input_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
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

/// 全局发送：字符串输入
pub fn type_string(text: &str) -> Result<()> {
    for c in text.chars() {
        send_char(c)?;
    }
    Ok(())
}

/// 全局发送：组合键
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

    // 等待指定延迟
    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    // 释放主按键
    key_up(key)?;

    // 反向释放修饰键
    for &modifier in modifiers.iter().rev() {
        let mod_key = modifier_to_key(modifier);
        key_up(mod_key)?;
    }

    Ok(())
}
