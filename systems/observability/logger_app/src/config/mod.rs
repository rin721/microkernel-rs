use serde::{Deserialize, Serialize};

/// 支持的日志输出格式。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// 带有 ANSI 颜色代码的人类可读文本（开发模式）。
    Text,
    /// 机器可读的 JSON（生产模式）。
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// 文件轮转策略。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Rotation {
    /// 每分钟轮转（适用于测试）。
    Minutely,
    /// 午夜轮转（默认）。
    Daily,
    /// 每小时轮转一次。
    Hourly,
    /// 从不轮转。
    Never,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Daily
    }
}

/// 日志记录通用应用配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    /// 最小日志级别过滤器（例如 `"info"`，`"debug"`，`"warn"`）。
    pub level: String,
    /// 输出格式。
    pub format: LogFormat,
    /// 写入滚动日志文件的目录。
    /// 如果为 `None`，日志仅写入标准输出。
    pub log_dir: Option<String>,
    /// 滚动日志文件的文件名前缀。
    pub file_prefix: String,
    /// 文件轮转策略。
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
