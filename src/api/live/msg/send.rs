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
    const CONST_URL: Option<&'static str> = Some("https://api.live.bilibili.com/msg/send");
}