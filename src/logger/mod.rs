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
            LogLevel::Critical => "Critical",
            LogLevel::Error => "Error",
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
        })
    }
}

impl LogLevel {
    #[inline]
    fn should_output(&self, log_level: &LogLevel) -> bool {
        *self as u8 <= *log_level as u8
    }
}

pub struct Log<'c> {
    pub content: &'c str,
    pub level: LogLevel
}

pub trait Logger {
    fn log(&mut self, log: Log);
    fn level(&self) -> &LogLevel;
    fn level_mut(&mut self) -> &mut LogLevel;
    
    fn qrcode<'b>(&mut self, bytes: &'b [u8]);

    #[inline]
    fn critical<'c>(&mut self, content: &'c str) {
        self.log(Log{
            content,
            level: LogLevel::Critical
        })
    }

    #[inline]
    fn error<'c>(&mut self, content: &'c str) {
        self.log(Log{
            content,
            level: LogLevel::Error
        })
    }

    #[inline]
    fn warn<'c>(&mut self, content: &'c str) {
        self.log(Log{
            content,
            level: LogLevel::Warn
        })
    }

    #[inline]
    fn info<'c>(&mut self, content: &'c str) {
        self.log(Log{
            content,
            level: LogLevel::Info
        })
    }

    #[inline]
    fn debug<'c>(&mut self, content: &'c str) {
        self.log(Log{
            content,
            level: LogLevel::Debug
        })
    }
}

