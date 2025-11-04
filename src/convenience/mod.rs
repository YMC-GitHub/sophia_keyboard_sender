//! 便捷函数

#[cfg(feature = "convenience")]
pub mod shortcuts;

// 重新导出
#[cfg(feature = "convenience")]
pub use shortcuts::*;
