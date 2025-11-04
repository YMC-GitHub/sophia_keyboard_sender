//! 常用快捷键函数

use crate::error::Result;
use crate::types::Key;

/// 发送 Tab 键
pub fn send_tab() -> Result<()> {
    crate::core::key_click(Key::Tab, None)
}

/// 发送 Enter 键
pub fn send_enter() -> Result<()> {
    crate::core::key_click(Key::Enter, None)
}

/// 发送 Escape 键
pub fn send_escape() -> Result<()> {
    crate::core::key_click(Key::Escape, None)
}

/// 发送空格键
pub fn send_space() -> Result<()> {
    crate::core::key_click(Key::Space, None)
}

/// 发送退格键
pub fn send_backspace() -> Result<()> {
    crate::core::key_click(Key::Backspace, None)
}

/// 发送删除键
pub fn send_delete() -> Result<()> {
    crate::core::key_click(Key::Delete, None)
}
