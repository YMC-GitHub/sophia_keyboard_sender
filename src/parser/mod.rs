//! 命令和参数解析功能

#[cfg(feature = "command_parser")]
pub mod command;

#[cfg(feature = "command_parser")]
pub mod duration;

// 重新导出
#[cfg(feature = "command_parser")]
pub use command::*;

#[cfg(feature = "command_parser")]
pub use duration::*;
