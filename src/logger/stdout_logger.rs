use super::{Log, LogLevel};
use qrcode::{QrCode, render::unicode};

pub struct StdoutLogger {
    pub level: LogLevel
}
impl StdoutLogger {
    pub fn new() -> Self {
        Self {
            level: LogLevel::Info
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level
    }
}

impl super::Logger for StdoutLogger {
    type Log = String;

    fn text(&mut self, log: Log<Self::Log>) {
        if self.level.should_output(&log.level) {
            // match log.level {
            //     LogLevel::Debug => todo!(),
            //     LogLevel::Error => todo!(),
            //     LogLevel::Warn => todo!(),
            //     LogLevel::Info => todo!(),
            //     LogLevel::Silent => todo!(),
            // }
            println!("[{}] {}", log.level, log.content)
        }
    }

    fn qrcode<'b>(&mut self, bytes: impl Into<&'b [u8]>) {
        let bytes:&'b [u8] = bytes.into();
        let code = QrCode::new(bytes).unwrap();
        let w = code.width();
        let show = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
        println!("[QRCODE]:");
        let sepline = "=".repeat(w);
        println!("{:^}", sepline);
        println!("{show}");
        println!("{:^}", sepline);
    }

    fn level(&self) -> &LogLevel {
        &self.level
    }

    fn level_mut(&mut self) -> &mut LogLevel {
        &mut self.level
    }
}