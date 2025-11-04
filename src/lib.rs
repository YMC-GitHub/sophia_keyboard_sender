//! sophia_keyboard_sender: 支持指定窗口发送、延迟控制的键盘事件发送器
//! 基于 keyboard-codes crate，为 sophia 重构提供基础
//!
//! # 特性
//! - `global`: 全局键盘模拟功能（默认启用）
//! - `window_target`: 窗口目标功能（默认启用）
//! - `command_parser`: 命令解析器功能（默认启用）
//! - `convenience`: 便捷函数
//!
//! # 示例
//! ```
//! use sophia_keyboard_sender::*;
//!
//! // 基本使用
//! key_click(Key::A, None).unwrap();
//!
//! // 命令解析
//! send("key:a").unwrap();
//! send("shortcut:ctrl+c").unwrap();
//! ```

// 模块声明
pub mod error;
pub mod types;

pub mod convenience;
pub mod core;
pub mod parser;

// 重新导出主要类型和函数
pub use error::{KeyboardSenderError, Result};
pub use types::{Key, Modifier, WindowHandle};

// 根据特性条件导出
#[cfg(feature = "global")]
pub use core::global::*;

#[cfg(feature = "window_target")]
pub use core::window_target::*;

#[cfg(feature = "command_parser")]
pub use parser::command::*;

#[cfg(feature = "command_parser")]
pub use parser::duration::*;

#[cfg(feature = "convenience")]
pub use convenience::shortcuts::*;

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "command_parser")]
    fn test_command_parser() -> Result<()> {
        // 测试向后兼容格式（这些应该能工作）
        send("key:a")?;
        send("char:b")?;
        send("text:hello")?;

        // 测试带 action 参数的格式
        send("action:key_click,key:a")?;
        send("action:char,char:!")?;
        send("action:text,text:world")?;

        // 测试快捷键
        send("shortcut:ctrl+c")?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "convenience")]
    fn test_convenience_functions() -> Result<()> {
        send_tab()?;
        send_enter()?;
        send_escape()?;
        Ok(())
    }

    #[test]
    #[cfg(feature = "command_parser")]
    fn test_parse_functions() -> Result<()> {
        use crate::parser::{parse_duration, parse_hwnd, parse_key, parse_modifier};

        // 测试持续时间解析
        assert_eq!(
            parse_duration("100ms")?,
            std::time::Duration::from_millis(100)
        );
        assert_eq!(parse_duration("2s")?, std::time::Duration::from_secs(2));

        // 测试键名解析
        assert_eq!(parse_key("a")?, Key::A);
        assert_eq!(parse_key("enter")?, Key::Enter);

        // 测试修饰符解析
        assert_eq!(parse_modifier("ctrl")?, Modifier::Control);
        assert_eq!(parse_modifier("shift")?, Modifier::Shift);

        // 测试窗口句柄解析
        assert_eq!(parse_hwnd("123456")?, 123456);
        assert_eq!(parse_hwnd("0x1A2B")?, 0x1A2B);
        assert_eq!(parse_hwnd("")?, 0);

        Ok(())
    }

    #[test]
    #[cfg(feature = "command_parser")]
    fn test_shortcut_function() -> Result<()> {
        shortcut("ctrl+c")?;
        shortcut("alt+f4")?;
        Ok(())
    }

    #[test]
    #[cfg(feature = "command_parser")]
    fn test_parse_command_params() -> Result<()> {
        use crate::parser::parse_command_params;

        let params = parse_command_params("key_click:a,hwnd:123456,duration:50ms");
        assert_eq!(params.get("key_click"), Some(&"a".to_string()));
        assert_eq!(params.get("hwnd"), Some(&"123456".to_string()));
        assert_eq!(params.get("duration"), Some(&"50ms".to_string()));

        let params = parse_command_params("shortcut:ctrl+alt+delete");
        assert_eq!(params.get("shortcut"), Some(&"ctrl+alt+delete".to_string()));

        Ok(())
    }
}
