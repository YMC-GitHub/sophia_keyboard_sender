use thiserror::Error;

/// 错误类型
#[derive(Error, Debug)]
pub enum KeyboardSenderError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unsupported key: {0}")]
    UnsupportedKey(String),
    #[error("Unsupported modifier: {0}")]
    UnsupportedModifier(String),
    #[error("Invalid duration format: {0}")]
    InvalidDuration(String),
    #[error("Invalid window handle: {0}")]
    InvalidWindowHandle(String),
    #[error("Command parse error: {0}")]
    CommandParseError(String),
    #[error("Feature not enabled: {0}")]
    FeatureNotEnabled(String),
    #[error("Windows API error")]
    WindowsError,
}

pub type Result<T> = std::result::Result<T, KeyboardSenderError>;
