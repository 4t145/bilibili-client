pub mod send;

pub enum LiveDanmaku {
    Emoticon(String),
    Text(String)
}

impl LiveDanmaku {
    #[inline]
    pub fn text(msg: impl Into<String>) -> Self {
        Self::Text(msg.into())
    }
}