// src/parser/mod.rs
//! 命令和参数解析功能

#[cfg(feature = "command_parser")]
pub mod command;

// 重新导出
#[cfg(feature = "command_parser")]
pub use command::*;