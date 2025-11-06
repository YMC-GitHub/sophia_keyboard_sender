// src/core/window_target.rs
//! 窗口目标键盘操作功能
#[cfg(not(feature = "window_target"))]
use crate::error::KeyboardSenderError;

use crate::error::Result;

use crate::types::{Key, WindowHandle};
use std::time::Duration;

#[cfg(feature = "window_target")]
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::WindowsAndMessaging::{
        BringWindowToTop, PostMessageA, SetForegroundWindow, WM_CHAR, WM_KEYDOWN, WM_KEYUP,
    },
};

use super::key_to_vk;

/// 将 isize 转换为 HWND
#[cfg(feature = "window_target")]
fn to_hwnd(handle: WindowHandle) -> HWND {
    #[allow(clippy::unnecessary_cast)]
    HWND(handle as isize)
}

/// 向指定窗口发送：按键按下
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

/// 向指定窗口发送：按键释放
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

/// 向指定窗口发送：按键点击
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

/// 向指定窗口发送：字符输入
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

/// 控制窗口焦点
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
                let _ = BringWindowToTop(window_handle);
                SetForegroundWindow(window_handle);
            }
        }
        Ok(())
    }
}
