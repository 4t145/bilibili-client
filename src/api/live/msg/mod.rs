use self::send::LiveSendReq;


#[repr(u32)]
pub enum LiveDanmakuColor {
    White = 0xffffff,
    Purple = 0xe33fff
}

pub mod send;
#[derive(Debug, Clone)]
pub enum LiveDanmaku {
    Emoticon(String),
    Text(String)
}

impl LiveDanmaku {
    #[inline]
    pub fn text(msg: impl Into<String>) -> Self {
        Self::Text(msg.into())
    }

    pub fn as_send_req(self, roomid: u64, jct: String) -> send::LiveSendReq {
        let rnd = rand::random();
        match self {
            LiveDanmaku::Emoticon(e) => {
                LiveSendReq {
                    csrf: jct,
                    msg: e,
                    roomid,
                    rnd,
                    color: LiveDanmakuColor::White as u32,
                    fontsize: 25,
                    dm_type: Some(1),

                }
            },
            LiveDanmaku::Text(t) => {
                LiveSendReq {
                    csrf: jct,
                    msg: t,
                    roomid,
                    rnd,
                    color: LiveDanmakuColor::White as u32,
                    fontsize: 25,
                    dm_type: None

                }
            },
        }
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
