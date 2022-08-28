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

#[macro_export]
macro_rules! danmaku {
    ($msg:expr) => {
        $crate::api::live::msg::LiveDanmaku::text($msg)
    };
    (official=>$eid:expr) => {
        $crate::api::live::msg::LiveDanmaku::Emoticon(format!("official_{}",$eid))
    };
    ($roomid:expr=>$eid:expr) => {
        $crate::api::live::msg::LiveDanmaku::Emoticon(format!("room_{}_{}",$roomid,$eid))
    };
}
