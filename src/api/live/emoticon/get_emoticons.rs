use crate::{
    api::{content_type::Json, CommonResp, Request},
    reqwest_client::Client,
};
use serde::{Deserialize, Serialize};

/// 推荐使用表单方法提交
pub struct GetEmoticons;

#[derive(Serialize, Clone, Copy, Debug)]
pub struct GetEmoticonsRequest {
    pub room_id: u64,
    pub platform: &'static str,
}

impl GetEmoticonsRequest {
    pub fn new_pc(room_id: u64) -> Self {
        Self {
            room_id,
            platform: "pc",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct EmoticonItem {
    pub emoji: String,
    pub descript: String,
    pub url: String,
    pub is_dynamic: i32,
    pub in_player_area: i32,
    pub width: i32,
    pub height: i32,
    pub identity: i32,
    pub unlock_need_gift: i32,
    pub perm: i32,
    pub unlock_need_level: i32,
    pub emoticon_value_type: i32,
    pub bulge_display: i32,
    pub unlock_show_text: String,
    pub unlock_show_color: String,
    pub emoticon_unique: String,
    pub emoticon_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct GetEmoticonsRespDataItem {
    pub emoticons: Vec<EmoticonItem>,
    pub pkg_descript: String,
    pub pkg_id: i32,
    pub pkg_name: String,
    pub pkg_perm: i32,
    pub unlock_identity: i32,
    pub unlock_need_gift: i32,
}

#[derive(Deserialize, Debug)]
pub struct GetEmoticonsRespData {
    pub data: Vec<GetEmoticonsRespDataItem>,
    pub fans_brand: i32,
}

impl<'r> Request<'r> for GetEmoticonsRequest {
    type Body = &'r Self;
    type Query = ();
    type ContentType = Json;
    type Response = CommonResp<GetEmoticonsRespData>;
    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "xlive/web-ucenter/v2/emoticon/GetEmoticons";
    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::body_from_request(self)
    }
}

impl Client {
    pub async fn get_emoticons_pc(
        &self,
        room_id: u64,
    ) -> crate::reqwest_client::ClientResult<GetEmoticonsRespData> {
        self.send(
            &GetEmoticonsRequest::new_pc(room_id),
            crate::api::api_live_url(),
        )
        .await?
        .into()
    }
}
