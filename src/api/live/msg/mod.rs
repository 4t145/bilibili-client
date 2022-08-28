pub mod send;

pub enum LiveDanmaku {
    Emoticon(String),
    Text(String)
}