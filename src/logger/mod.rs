use std::fmt::Display;

pub mod stdout_logger;
#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Critical = 4,
    Error = 3,
    Warn = 2,
    Info = 1,
    Debug = 0,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogLevel::Critical => "Crt",
            LogLevel::Error => "Err",
            LogLevel::Warn => "Wrn",
            LogLevel::Info => "Inf",
            LogLevel::Debug => "Dbg",
        })
    }
}

impl LogLevel {
    #[inline]
    fn should_output(&self, log_level: &LogLevel) -> bool {
        *self as u8 <= *log_level as u8
    }
}

pub struct Log<C: From<String>> {
    pub content: C,
    pub level: LogLevel
}

pub trait Logger {
    type Log: From<String>;
    fn log(&mut self, log: Log<Self::Log>);
    fn qrcode<'b>(&mut self, bytes: impl Into<&'b [u8]>);
    fn level(&self) -> &LogLevel;
    fn level_mut(&mut self) -> &mut LogLevel;

    #[inline]
    fn critical(&mut self, content: impl Into<Self::Log>) {
        self.log(Log{
            content: content.into(),
            level: LogLevel::Critical
        })
    }

    #[inline]
    fn error(&mut self, content: impl Into<Self::Log>) {
        self.log(Log{
            content: content.into(),
            level: LogLevel::Error
        })
    }

    #[inline]
    fn warn(&mut self, content: impl Into<Self::Log>) {
        self.log(Log{
            content: content.into(),
            level: LogLevel::Warn
        })
    }

    #[inline]
    fn info(&mut self, content: impl Into<Self::Log>) {
        self.log(Log{
            content: content.into(),
            level: LogLevel::Info
        })
    }

    #[inline]
    fn debug(&mut self, content: impl Into<Self::Log>) {
        self.log(Log{
            content: content.into(),
            level: LogLevel::Debug
        })
    }
}

