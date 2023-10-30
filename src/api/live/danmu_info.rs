use serde::{Deserialize, Serialize};

use crate::{
    api::{CommonResp, Request},
    reqwest_client::Client,
};

#[derive(Serialize, Debug)]
pub struct GetRoomInfoRequest {
    pub room_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct RoomInfo {
    pub room_id: u64,
    pub uid: u64,
}

impl<'r> Request<'r> for GetRoomInfoRequest {
    type Body = ();

    type Query = &'r Self;

    type ContentType = ();

    type Response = CommonResp<RoomInfo>;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "room/v1/Room/get_info";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::query_from_request(self)
    }
}

impl Client {
    pub async fn get_room_play_info(
        &self,
        room_id: u64,
    ) -> crate::reqwest_client::ClientResult<RoomInfo> {
        self.send(
            &GetRoomInfoRequest { room_id },
            crate::api::api_live_url(),
        )
        .await?
        .into()
    }
}
