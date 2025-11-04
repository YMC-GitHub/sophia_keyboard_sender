//! 窗口目标键盘操作功能
#[allow(unused_imports)]
use crate::error::{KeyboardSenderError, Result};
use crate::types::{Key, WindowHandle};
use std::time::Duration;

#[allow(unused_imports)]
#[cfg(feature = "window_target")]
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

use keyboard_codes::KeyCodeMapper;

/// 将 isize 转换为 HWND
#[cfg(feature = "window_target")]
pub fn to_hwnd(handle: WindowHandle) -> HWND {
    #[allow(clippy::unnecessary_cast)]
    HWND(handle as isize)
}

/// 将 Key 转换为 Windows 虚拟键码（VK_CODE）
fn key_to_vk(key: Key) -> u16 {
    key.to_code(keyboard_codes::current_platform()) as u16
}

/// 向指定窗口发送：按键按下（WM_KEYDOWN 消息）
pub fn send_key_down_to_window(hwnd: WindowHandle, key: Key) -> Result<()> {
    #[cfg(not(feature = "window_target"))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "window_target".to_string(),
    ));

    #[cfg(feature = "window_target")]
    {
        let vk = key_to_vk(key);
        let window_handle = to_hwnd(hwnd);

        unsafe {
            let _ = PostMessageA(window_handle, WM_KEYDOWN, WPARAM(vk as _), LPARAM(0));
        }
        Ok(())
    }
}

/// 向指定窗口发送：按键释放（WM_KEYUP 消息）
pub fn send_key_up_to_window(hwnd: WindowHandle, key: Key) -> Result<()> {
    #[cfg(not(feature = "window_target"))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "window_target".to_string(),
    ));

    #[cfg(feature = "window_target")]
    {
        let vk = key_to_vk(key);
        let window_handle = to_hwnd(hwnd);

        unsafe {
            let _ = PostMessageA(window_handle, WM_KEYUP, WPARAM(vk as _), LPARAM(0));
        }
        Ok(())
    }
}

/// 向指定窗口发送：按键点击（带延迟控制）
pub fn send_key_click_to_window(
    hwnd: WindowHandle,
    key: Key,
    press_duration: Option<Duration>,
) -> Result<()> {
    send_key_down_to_window(hwnd, key)?;

    if let Some(dur) = press_duration {
        std::thread::sleep(dur);
    }

    send_key_up_to_window(hwnd, key)?;
    Ok(())
}

/// 向指定窗口发送：字符输入（WM_CHAR 消息，支持 Unicode）
pub fn send_char_to_window(hwnd: WindowHandle, c: char) -> Result<()> {
    #[cfg(not(feature = "window_target"))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "window_target".to_string(),
    ));

    #[cfg(feature = "window_target")]
    {
        let window_handle = to_hwnd(hwnd);

        unsafe {
            let _ = PostMessageA(window_handle, WM_CHAR, WPARAM(c as _), LPARAM(0));
        }
        Ok(())
    }
}

/// 向指定窗口发送：字符串输入
pub fn type_string_to_window(hwnd: WindowHandle, text: &str) -> Result<()> {
    for c in text.chars() {
        send_char_to_window(hwnd, c)?;
    }
    Ok(())
}

/// 控制窗口焦点（可选激活窗口）
pub fn set_window_focus(hwnd: WindowHandle, bring_to_top: bool) -> Result<()> {
    #[cfg(not(feature = "window_target"))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "window_target".to_string(),
    ));

    #[cfg(feature = "window_target")]
    {
        let window_handle = to_hwnd(hwnd);

        if bring_to_top {
            unsafe {
                let _ = BringWindowToTop(window_handle); // 置于顶层
                SetForegroundWindow(window_handle); // 激活窗口（获取焦点）
            }
        }
        Ok(())
    }
}
