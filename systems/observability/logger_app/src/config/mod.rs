use serde::{Deserialize, Serialize};

/// Supported log output formats.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Human-readable text with ANSI color codes (dev mode).
    Text,
    /// Machine-readable JSON (production mode).
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// File rotation strategy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Rotation {
    /// Rotate every minute (useful for tests).
    Minutely,
    /// Rotate at midnight (default).
    Daily,
    /// Rotate once per hour.
    Hourly,
    /// Never rotate.
    Never,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Daily
    }
}

/// Configuration for the logging Generic App.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    /// Minimum log level filter (e.g., `"info"`, `"debug"`, `"warn"`).
    pub level: String,
    /// Output format.
    pub format: LogFormat,
    /// Directory to write rolling log files into.
    /// If `None`, logs are written only to stdout.
    pub log_dir: Option<String>,
    /// Filename prefix for rolling log files.
    pub file_prefix: String,
    /// File rotation strategy.
    pub rotation: Rotation,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: "info".to_owned(),
            format: LogFormat::default(),
            log_dir: None,
            file_prefix: "microkernel".to_owned(),
            rotation: Rotation::default(),
        }
    }
}
