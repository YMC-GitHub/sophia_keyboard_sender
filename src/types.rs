//! 类型定义

/// 窗口句柄类型别名，支持 isize
pub type WindowHandle = isize;

// 重新导出 keyboard-codes 类型
pub use keyboard_codes::{Key, KeyCodeMapper, Modifier};
