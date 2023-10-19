use serde::{Deserialize, Serialize};

use crate::{api::{Request, CommonResp}, reqwest_client::Client};

#[derive(Serialize)]
pub struct GetRoomPlayInfoRequest {
    pub room_id: u64,
}

#[derive(Deserialize)]
pub struct RoomPlayInfo {
    pub room_id: u64,
    pub uid: u64,
}


impl<'r> Request<'r> for GetRoomPlayInfoRequest {
    type Body = ();

    type Query = &'r Self;

    type ContentType = ();

    type Response = CommonResp<RoomPlayInfo>;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "xlive/web-room/v2/index/getRoomPlayInfo";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::query_from_request(self)
    }
}

impl Client {
    pub async fn get_room_play_info(&self, room_id: u64) -> crate::reqwest_client::ClientResult<RoomPlayInfo> {
        self.send(&GetRoomPlayInfoRequest { room_id }, crate::api::api_live_url()).await?.into()
    }
}