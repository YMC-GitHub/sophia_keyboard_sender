//! sophia_keyboard_sender: 支持指定窗口发送、延迟控制的键盘事件发送器
//! 基于 keyboard-codes crate，为 sophia 重构提供基础

use keyboard_codes::{Key, KeyCodeMapper, Modifier};
use std::str::FromStr;
use std::time::Duration;
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::KeyboardAndMouse::{
            SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
            KEYEVENTF_UNICODE, VIRTUAL_KEY,
        },
        WindowsAndMessaging::{
            BringWindowToTop, PostMessageA, SetForegroundWindow, WM_CHAR, WM_KEYDOWN, WM_KEYUP,
        },
    },
};

/// 将 Key 转换为 Windows 虚拟键码（VK_CODE）
fn key_to_vk(key: Key) -> u16 {
    // 使用 keyboard-codes 的转换功能
    key.to_code(keyboard_codes::current_platform()) as u16
}

/// 将 Modifier 转换为对应的 Key
fn modifier_to_key(modifier: Modifier) -> Key {
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
pub fn key_down(key: Key) -> Result<(), Box<dyn std::error::Error>> {
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

/// 全局发送：按键释放（模拟物理键盘）
pub fn key_up(key: Key) -> Result<(), Box<dyn std::error::Error>> {
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

/// 全局发送：按键点击（按下 + 释放，带可选延迟）
pub fn key_click(
    key: Key,
    press_duration: Option<Duration>, // 按下后到释放前的延迟
) -> Result<(), Box<dyn std::error::Error>> {
    key_down(key)?;

    // 若指定延迟，则等待一段时间后再释放
    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    key_up(key)?;
    Ok(())
}

/// 全局发送：字符输入（通过 Unicode 编码，支持任意字符）
pub fn send_char(c: char) -> Result<(), Box<dyn std::error::Error>> {
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

/// 全局发送：字符串输入（逐个字符发送）
pub fn type_string(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    for c in text.chars() {
        send_char(c)?;
    }
    Ok(())
}

/// 全局发送：组合键（修饰符 + 按键，带延迟控制）
pub fn press_combination(
    modifiers: &[Modifier],
    key: Key,
    press_duration: Option<Duration>, // 按键按下后的延迟
) -> Result<(), Box<dyn std::error::Error>> {
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

// ------------------------------
// 向指定窗口发送事件（基于窗口消息）
// ------------------------------

/// 向指定窗口发送：按键按下（WM_KEYDOWN 消息）
pub fn send_key_down_to_window(hwnd: HWND, key: Key) -> Result<(), Box<dyn std::error::Error>> {
    let vk = key_to_vk(key);

    unsafe {
        // 发送 WM_KEYDOWN 消息到目标窗口
        let _ = PostMessageA(hwnd, WM_KEYDOWN, WPARAM(vk as _), LPARAM(0));
    }
    Ok(())
}

/// 向指定窗口发送：按键释放（WM_KEYUP 消息）
pub fn send_key_up_to_window(hwnd: HWND, key: Key) -> Result<(), Box<dyn std::error::Error>> {
    let vk = key_to_vk(key);

    unsafe {
        let _ = PostMessageA(hwnd, WM_KEYUP, WPARAM(vk as _), LPARAM(0));
    }
    Ok(())
}

/// 向指定窗口发送：按键点击（带延迟控制）
pub fn send_key_click_to_window(
    hwnd: HWND,
    key: Key,
    press_duration: Option<Duration>,
) -> Result<(), Box<dyn std::error::Error>> {
    send_key_down_to_window(hwnd, key)?;

    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    send_key_up_to_window(hwnd, key)?;
    Ok(())
}

/// 向指定窗口发送：字符输入（WM_CHAR 消息，支持 Unicode）
pub fn send_char_to_window(hwnd: HWND, c: char) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // 发送 WM_CHAR 消息（直接传递字符的 Unicode 值）
        let _ = PostMessageA(hwnd, WM_CHAR, WPARAM(c as _), LPARAM(0));
    }
    Ok(())
}

/// 向指定窗口发送：字符串输入
pub fn type_string_to_window(hwnd: HWND, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    for c in text.chars() {
        send_char_to_window(hwnd, c)?;
    }
    Ok(())
}

/// 控制窗口焦点（可选激活窗口）
pub fn set_window_focus(hwnd: HWND, bring_to_top: bool) -> Result<(), Box<dyn std::error::Error>> {
    if bring_to_top {
        unsafe {
            let _ = BringWindowToTop(hwnd); // 置于顶层
            SetForegroundWindow(hwnd); // 激活窗口（获取焦点）
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_key_operations() -> Result<(), Box<dyn std::error::Error>> {
        // 测试按键点击（带50ms延迟）
        let key_a = Key::from_str("A").unwrap();
        key_click(key_a, Some(Duration::from_millis(50)))?;

        // 测试字符发送
        send_char('!')?;

        // 测试字符串输入
        type_string("test")?;

        // 测试组合键（Ctrl+C）
        let ctrl = Modifier::from_str("Control").unwrap();
        let key_c = Key::from_str("C").unwrap();
        press_combination(&[ctrl], key_c, None)?;

        Ok(())
    }

    // 注意：窗口相关测试需要实际窗口句柄，此处仅展示调用方式
    #[test]
    fn test_window_targeted_operations() -> Result<(), Box<dyn std::error::Error>> {
        let dummy_hwnd = HWND(0); // 实际使用时需替换为有效窗口句柄
                                  // let enter_key = Key::from_str("Enter").unwrap();
                                  // send_key_click_to_window(dummy_hwnd, enter_key, None)?;
                                  // type_string_to_window(dummy_hwnd, "window test")?;
        Ok(())
    }
}
