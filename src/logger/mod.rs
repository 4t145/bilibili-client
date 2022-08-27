pub mod stdout_logger;
#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Critical = 4,
    Error = 3,
    Warn = 2,
    Info = 1,
    Debug = 0,
}

impl LogLevel {
    #[inline]
    fn should_output(&self, log_level: &LogLevel) -> bool {
        self <= log_level
    }
}

pub struct Log<C: From<String>> {
    pub content: C,
    pub level: LogLevel
}

pub trait Logger {
    type Log: From<String>;
    fn text(&mut self, log: Log<Self::Log>);
    fn qrcode<'b>(&mut self, bytes: impl Into<&'b [u8]>);
}

