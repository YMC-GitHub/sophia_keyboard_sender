//! sophia_keyboard_sender: 支持指定窗口发送、延迟控制的键盘事件发送器
//! 基于 keyboard-codes crate，为 sophia 重构提供基础
//!
//! # 特性
//! - `global`: 全局键盘模拟功能（默认启用）
//! - `window_target`: 窗口目标功能
//! - `command_parser`: 命令解析器功能（默认启用）
//! - `smart`: 智能输入包装函数（可选）
//!
//! # 示例
//! ```
//! use sophia_keyboard_sender::*;
//!
//! // 明确API使用（推荐用于生产环境）
//! type_string("Hello World").unwrap();
//! key_click(Key::Enter, None).unwrap();
//! press_combination(&[Modifier::Control], Key::C, None).unwrap();
//!
//! // 特殊键的便捷使用方式
//! key_click(Key::Tab, None).unwrap();      // Tab键
//! key_click(Key::Enter, None).unwrap();    // 回车键
//! key_click(Key::Space, None).unwrap();    // 空格键
//!
//! // 智能输入（需要启用 smart 特性）
//! # #[cfg(feature = "smart")]
//! # {
//! type_auto("Hello World").unwrap();      // 自动检测为文本
//! type_auto("enter").unwrap();           // 自动检测为回车键
//! type_auto("ctrl+c").unwrap();          // 自动检测为快捷键
//! # }
//! ```

// 模块声明
pub mod core;
pub mod error;
pub mod parser;
pub mod smart;
pub mod types;

// 重新导出主要类型和函数
pub use error::{KeyboardSenderError, Result};
pub use types::{Key, Modifier, WindowHandle};

// 重新导出 sleep-utils 的功能
pub use sleep_utils::{parse_sleep_duration, sleep, smart_sleep};

// 根据特性条件导出
#[cfg(feature = "global")]
pub use core::global::*;

#[cfg(feature = "window_target")]
pub use core::window_target::*;

#[cfg(feature = "command_parser")]
pub use parser::command::*;

/// 智能输入函数（需要启用 `smart` 特性）
///
/// 自动检测输入类型：
/// - 文本 → 字符串输入
/// - 单键 → 按键点击  
/// - 快捷键 → 组合键
/// - 修饰符 → 修饰键点击
///
/// # 示例
/// ```
/// # #[cfg(feature = "smart")]
/// # {
/// use sophia_keyboard_sender::type_auto;
///
/// type_auto("Hello World").unwrap();  // 文本
/// type_auto("enter").unwrap();        // 回车
/// type_auto("ctrl+s").unwrap();       // 保存
/// type_auto("a").unwrap();            // 字母
/// # }
/// ```
#[cfg(feature = "smart")]
pub fn type_auto(input: &str) -> Result<()> {
    crate::smart::type_auto(input)
}

/// 批量智能输入（需要启用 `smart` 特性）
#[cfg(feature = "smart")]
pub fn type_multiple(inputs: &[&str]) -> Result<()> {
    crate::smart::type_multiple(inputs)
}

/// 带延迟的智能输入（需要启用 `smart` 特性）
#[cfg(feature = "smart")]
pub fn type_with_delay(text: &str, delay: std::time::Duration) -> Result<()> {
    crate::smart::type_with_delay(text, delay)
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_functions() -> Result<()> {
        smart_sleep(10)?;
        smart_sleep("10ms")?;
        Ok(())
    }

    #[test]
    #[cfg(feature = "command_parser")]
    fn test_command_parser() -> Result<()> {
        send("key:a")?;
        send("char:b")?;
        send("text:hello")?;
        send("shortcut:ctrl+c")?;
        Ok(())
    }

    #[test]
    fn test_special_keys() -> Result<()> {
        // 测试特殊键的使用方式（替代原来的便捷函数）
        #[cfg(feature = "global")]
        {
            key_click(Key::Tab, None)?;
            key_click(Key::Enter, None)?;
            key_click(Key::Space, None)?;
            key_click(Key::Escape, None)?;
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "smart")]
    fn test_smart_functions() -> Result<()> {
        // 测试智能功能可用性
        Ok(())
    }
}
