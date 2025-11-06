// src/smart/mod.rs
//! 智能输入包装函数（可选特性）
//!
//! 提供自动类型检测的便捷函数，适合简单脚本和快速原型开发。
//! 对于生产环境，推荐使用明确的API以获得更好的类型安全和性能。

#[cfg(not(feature = "smart"))]
use crate::error::{KeyboardSenderError, Result};

#[cfg(feature = "smart")]
mod implementation;

#[cfg(feature = "smart")]
pub use implementation::*;

/// 智能输入功能需要启用 `smart` 特性
#[cfg(not(feature = "smart"))]
pub fn type_auto(_input: &str) -> Result<()> {
    Err(KeyboardSenderError::FeatureNotEnabled("smart".to_string()))
}

/// 智能输入功能需要启用 `smart` 特性  
#[cfg(not(feature = "smart"))]
pub fn type_multiple(_inputs: &[&str]) -> Result<()> {
    Err(KeyboardSenderError::FeatureNotEnabled("smart".to_string()))
}

/// 智能输入功能需要启用 `smart` 特性
#[cfg(not(feature = "smart"))]
pub fn type_with_delay(_text: &str, _delay: std::time::Duration) -> Result<()> {
    Err(KeyboardSenderError::FeatureNotEnabled("smart".to_string()))
}
