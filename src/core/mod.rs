// src/core/mod.rs
//! 核心键盘操作功能

#[cfg(feature = "global")]
pub mod global;

#[cfg(feature = "window_target")]
pub mod window_target;

// 重新导出
#[cfg(feature = "global")]
pub use global::*;

#[cfg(feature = "window_target")]
pub use window_target::*;

/// 共享的工具函数
mod utils {
    use crate::types::Key;
    use keyboard_codes::KeyCodeMapper;

    /// 将 Key 转换为 Windows 虚拟键码（VK_CODE）
    pub fn key_to_vk(key: Key) -> u16 {
        key.to_code(keyboard_codes::current_platform()) as u16
    }

    /// 将 Modifier 转换为对应的 Key
    pub fn modifier_to_key(modifier: crate::types::Modifier) -> Key {
        use std::str::FromStr;

        match modifier {
            crate::types::Modifier::Shift
            | crate::types::Modifier::LeftShift
            | crate::types::Modifier::RightShift => Key::from_str("Shift").unwrap_or(Key::Escape),
            crate::types::Modifier::Control
            | crate::types::Modifier::LeftControl
            | crate::types::Modifier::RightControl => {
                Key::from_str("Control").unwrap_or(Key::Escape)
            }
            crate::types::Modifier::Alt
            | crate::types::Modifier::LeftAlt
            | crate::types::Modifier::RightAlt => Key::from_str("Alt").unwrap_or(Key::Escape),
            crate::types::Modifier::Meta
            | crate::types::Modifier::LeftMeta
            | crate::types::Modifier::RightMeta => Key::from_str("Meta").unwrap_or(Key::Escape),
        }
    }
}

#[cfg(feature = "global")]
#[allow(unused_imports)]
pub(crate) use utils::*;

#[cfg(feature = "window_target")]
#[allow(unused_imports)]
pub(crate) use utils::*;
