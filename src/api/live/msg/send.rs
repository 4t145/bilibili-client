use crate::api::{/* CommonResp, */ Api, CommonResp};
use serde::{Serialize, Deserialize};



pub struct LiveSend;

#[derive(Serialize)]
pub struct LiveSendReq {
    pub roomid: u64,
    pub msg: String,
    pub csrf: String,
    pub rnd: u32,
    pub color: u32,
    pub fontsize: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_type: Option<u8>
}






pub(crate) fn gen(roomid:u64, msg: String, bili_jct: String) -> LiveSendReq {
    let rnd = rand::random();
    LiveSendReq {
        csrf: bili_jct,
        msg,
        roomid,
        rnd,
        color: super::LiveDanmakuColor::White as u32,
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
        color: super::LiveDanmakuColor::White as u32,
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
    type Response = CommonResp<LiveSendRespData>;
    const METHOD: reqwest::Method = reqwest::Method::POST;
    const URL: &'static str = "https://api.live.bilibili.com/msg/send";
}