use crate::{
    api::{content_type::Form, /* CommonResp, */ CommonResp, Request},
    reqwest_client::{Client, ClientResult},
};
use serde::{Deserialize, Serialize};

pub struct LiveSend;

#[derive(Serialize)]
pub struct LiveSendReq<'r> {
    pub roomid: u64,
    pub msg: &'r str,
    pub csrf: &'r str,
    pub rnd: u32,
    pub color: u32,
    pub fontsize: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_type: Option<u8>,
}

impl<'r> Request<'r> for LiveSendReq<'r> {
    type Body = &'r Self;

    type Query = ();

    type ContentType = Form;

    type Response = CommonResp<()>;

    const METHOD: http::Method = http::Method::POST;

    const PATH: &'static str = "msg/send";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::body_from_request(self)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LiveSendRespData {
    Code(i32),
    String(String),
    Object,
}

impl Client {
    pub async fn live_send(&self, send_req: LiveSendReq<'_>) -> ClientResult<()> {
        self.send(&send_req, crate::api::api_live_url())
            .await?
            .into()
    }
}
