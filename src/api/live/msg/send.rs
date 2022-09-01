use rand::RngCore;
use crate::api::{/* CommonResp, */ Api};
use serde::{Serialize, Deserialize};



pub struct LiveSend;

#[derive(Serialize)]
pub struct LiveSendReq {
    roomid: u64,
    msg: String,
    csrf: String,
    rnd: u32,
    color: u32,
    fontsize: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    dm_type: Option<u8>
}


#[repr(u32)]
pub enum LiveDanmakuColor {
    White = 0xffffff,
    Purple = 0xe33fff
}



pub(crate) fn gen(roomid:u64, msg: String, bili_jct: String) -> LiveSendReq {
    let rnd = rand::random();
    LiveSendReq {
        csrf: bili_jct,
        msg,
        roomid,
        rnd,
        color: LiveDanmakuColor::White as u32,
        fontsize: 25,
        dm_type: None
    }
}

pub(crate) fn gen_emoticon(roomid:u64, emoticon: String, bili_jct: String) -> LiveSendReq {
    let rnd = rand::random();
    LiveSendReq {
        csrf: bili_jct,
        msg: emoticon,
        roomid,
        rnd,
        color: LiveDanmakuColor::White as u32,
        fontsize: 25,
        dm_type: Some(1)
    }
}


#[derive(Deserialize)]
#[serde(untagged)]
pub enum LiveSendRespData {
    Code(i32),
    String(String),
    Object
}

impl Api for LiveSend  {
    type Request = LiveSendReq;
    type Response = serde_json::Value;
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const URL: &'static str = "https://api.live.bilibili.com/msg/send";
}