use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{api::{Request, CommonResp}, reqwest_client::Client};

#[derive(Debug, Deserialize, Clone)]
pub struct Host {
    pub host: String,
    pub wss_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DanmuInfoData {
    // max_delay: i32,
    pub token: String,
    pub host_list: Vec<Host>,
}

impl Host {
    pub fn wss(&self) -> Url {
        let host = &self.host;
        let port = self.wss_port;
        Url::parse(&format!("wss://{host}:{port}/sub")).expect("invalid url")
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct GetDanmuInfoRequest {
    id: u64,
    r#type: u8,
}

impl<'r> Request<'r> for GetDanmuInfoRequest {
    type Body = ();

    type Query = &'r Self;

    type ContentType = ();

    type Response = CommonResp<DanmuInfoData>;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "xlive/web-room/v1/index/getDanmuInfo";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::query_from_request(self)
    }
}

impl Client {
    pub async fn get_danmu_info(&self, room_id: u64) -> crate::reqwest_client::ClientResult<DanmuInfoData> {
        self.send(&GetDanmuInfoRequest { id: room_id, r#type: 0 }, crate::api::api_live_url()).await?.into()
    }
}