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
