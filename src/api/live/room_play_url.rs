use serde::{Deserialize, Serialize};

use crate::{
    api::{CommonResp, Request},
    reqwest_client::{Client, ClientResult},
};
#[derive(Debug, Clone, Serialize)]
pub struct RoomPlayUrlRequest {
    pub cid: u64,
    pub platform: StreamPlatform,
    pub qn: StreamQuality,
}

impl RoomPlayUrlRequest {
    pub fn new(room_id: u64) -> Self {
        RoomPlayUrlRequest {
            cid: room_id,
            platform: Default::default(),
            qn: Default::default(),
        }
    }
    pub fn platform(self, platform: StreamPlatform) -> Self {
        Self { platform, ..self }
    }

    pub fn qn(self, qn: StreamQuality) -> Self {
        Self { qn, ..self }
    }
}

#[derive(Default, Debug, Clone, Serialize)]
pub enum StreamQuality {
    #[serde(rename = "80")]
    Fluent,
    #[default]
    #[serde(rename = "150")]
    Hd,
    #[serde(rename = "400")]
    BlueLight,
    #[serde(rename = "10000")]
    Raw,
    #[serde(rename = "20000")]
    FourK,
    #[serde(rename = "30000")]
    Dolby,
    #[serde(untagged)]
    Custom(String),
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StreamPlatform {
    H5,
    #[default]
    Web,
}

#[derive(Debug, Deserialize)]
pub struct RoomPlayUrlResponse {
    pub current_quality: i32,
    pub accept_quality: Vec<String>,
    pub current_qn: i32,
    pub quality_description: Vec<QualityDescription>,
    pub durl: Vec<Durl>,
}

#[derive(Debug, Deserialize)]
pub struct Durl {
    pub url: String,
    pub order: u32,
    // stream_type: u32,
    // p2p_type: u32,
}

#[derive(Debug, Deserialize)]
pub struct QualityDescription {
    pub qn: i32,
    pub desc: String,
}

impl<'r> Request<'r> for RoomPlayUrlRequest {
    type Body = ();

    type Query = &'r Self;

    type ContentType = ();

    type Response = CommonResp<RoomPlayUrlResponse>;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "room/v1/Room/playUrl";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::query_from_request(self)
    }
}

impl Client {
    pub async fn get_room_play_url(
        &self,
        request: &RoomPlayUrlRequest,
    ) -> ClientResult<RoomPlayUrlResponse> {
        self.send(request, crate::api::api_live_url()).await?.into()
    }
}
