//! 持续时间解析功能

use crate::error::{KeyboardSenderError, Result};
use std::time::Duration;

#[cfg(feature = "command_parser")]
use lazy_static::lazy_static;
#[cfg(feature = "command_parser")]
use regex::Regex;

/// 解析持续时间字符串 (如 "20ms", "1s", "500ms")
pub fn parse_duration(duration_str: &str) -> Result<Duration> {
    #[cfg(not(feature = "command_parser"))]
    return Err(KeyboardSenderError::FeatureNotEnabled(
        "command_parser".to_string(),
    ));

    #[cfg(feature = "command_parser")]
    {
        lazy_static! {
            static ref DURATION_RE: Regex = Regex::new(r"^(\d+)(ms|s)$").unwrap();
        }

        if let Some(caps) = DURATION_RE.captures(duration_str) {
            let value: u64 = caps[1].parse().map_err(|_| {
                KeyboardSenderError::InvalidDuration(format!("Invalid number: {}", &caps[1]))
            })?;
            let unit = &caps[2];

            match unit {
                "ms" => Ok(Duration::from_millis(value)),
                "s" => Ok(Duration::from_secs(value)),
                _ => Err(KeyboardSenderError::InvalidDuration(format!(
                    "Unknown unit: {}",
                    unit
                ))),
            }
        } else {
            Err(KeyboardSenderError::InvalidDuration(
                duration_str.to_string(),
            ))
        }
    }
}
