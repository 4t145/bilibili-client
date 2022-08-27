use rand::RngCore;
use crate::api_trait::JsonApi;
pub struct LiveSend;

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LiveSendReq {
    roomid: u64,
    msg: String,
    csrf: String,
    rnd: u32,
    color: u32,
    fontsize: u8,
}

#[repr(u32)]
pub enum LiveDanmakuColor {
    White = 0xffffff
}

pub struct LiveSendReqGenerator {
    rng: rand::rngs::ThreadRng,
}

impl LiveSendReqGenerator {
    pub fn new() -> Self{
        Self {
            rng: rand::thread_rng()
        }
    }
    pub fn gen(&mut self, roomid:u64, msg: String, bili_jct: String) -> LiveSendReq {
        let rnd = self.rng.next_u32();
        LiveSendReq::new(bili_jct, msg, roomid, rnd)
    }
}

impl LiveSendReq {
    #[inline]
    pub fn new(bili_jct: String, msg: String, roomid: u64, rnd: u32) -> Self {
        Self {
            csrf: bili_jct,
            msg,
            roomid,
            rnd,
            color: LiveDanmakuColor::White as u32,
            fontsize: 25,
        }
    }
}

impl JsonApi for LiveSend  {
    type Request = LiveSendReq;
    type Response = ();

    const METHOD: reqwest::Method = reqwest::Method::POST;

    const URL: &'static str = "https://api.live.bilibili.com/send";

}