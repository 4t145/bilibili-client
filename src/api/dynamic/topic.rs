use serde::{Deserialize, Serialize};

use crate::{api::{CommonResp, Request}, reqwest_client::ReqwestClient};

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicTopicRequest<'r> {
    pub topic_name: &'r str,
    pub offset_dynamic_id: u64
}

impl ReqwestClient {
    pub async fn dynamic_topic<'r>(&self, topic_name: &'r str, offset_dynamic_id: u64) -> crate::reqwest_client::ClientResult<DynamicTopicResponse> {
        self.send(&DynamicTopicRequest { topic_name, offset_dynamic_id }, crate::api::api_vc_url()).await?.into()
    }
}

impl<'r> Request<'r> for DynamicTopicRequest<'r> {
    type Body = ();

    type Query = &'r Self;

    type ContentType = ();

    type Response = CommonResp<DynamicTopicResponse>;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "topic_svr/v1/topic_svr/topic_history";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        crate::api::RequestParts::query_from_request(self)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicTopicResponse {
    pub has_more: u8,
    pub cards: Vec<CardItem>,
    pub offset: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardItem {
    pub desc: Desc,
    pub card: String,
    // pub extend_json: String,
    // pub display: Display,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Desc {
    pub uid: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub rid: i64,
    pub acl: i64,
    pub view: i64,
    pub repost: i64,                                                                                                                                                        
    pub comment: i64,
    pub like: i64,
    pub is_liked: i64,
    pub dynamic_id: i64,
    pub timestamp: i64,
    pub pre_dy_id: i64,
    pub orig_dy_id: i64,
    pub orig_type: i64,
    pub user_profile: UserProfile,
    pub uid_type: i64,
    // pub recommend_info: RecommendInfo,
    pub stype: i64,
    pub r_type: i64,
    pub inner_id: i64,
    pub topic_board: String,
    pub topic_board_desc: String,
    pub status: i64,
    pub dynamic_id_str: String,
    pub pre_dy_id_str: String,
    pub orig_dy_id_str: String,
    pub rid_str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub info: Info,
    // pub card: Card,
    pub vip: Vip,
    // pub pendant: Pendant,
    pub rank: String,
    pub sign: String,
    // pub level_info: LevelInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub uid: i64,
    pub uname: String,
    pub face: String,
    pub face_nft: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub official_verify: OfficialVerify,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vip {
    #[serde(rename = "vipType")]
    pub vip_type: i64,
    #[serde(rename = "vipDueDate")]
    pub vip_due_date: i64,
    #[serde(rename = "vipStatus")]
    pub vip_status: i64,
    #[serde(rename = "themeType")]
    pub theme_type: i64,
    pub label: Label,
    pub avatar_subscript: i64,
    pub nickname_color: String,
    pub role: i64,
    pub avatar_subscript_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: i64,
    pub bg_color: String,
    pub border_color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: i64,
    pub name: String,
    pub image: String,
    pub expire: i64,
    pub image_enhance: String,
    pub image_enhance_frame: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelInfo {
    pub current_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecommendInfo {
    pub is_attention: i64,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Display {
//     pub topic_info: TopicInfo,
//     pub emoji_info: EmojiInfo,
//     pub relation: Relation,
//     pub up_act_button: UpActButton,
//     pub show_tip: ShowTip,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicInfo {
    pub topic_details: Vec<TopicDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicDetail {
    pub topic_id: i64,
    pub topic_name: String,
    pub is_activity: i64,
    pub topic_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiInfo {
    pub emoji_details: Vec<EmojiDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmojiDetail {
    pub emoji_name: String,
    pub id: i64,
    pub package_id: i64,
    pub state: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub attr: i64,
    pub text: String,
    pub url: String,
    pub meta: Meta,
    pub mtime: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub status: i64,
    pub is_follow: i64,
    pub is_followed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpActButton {
    pub report_title: String,
    pub founder_report_title: String,
    pub top_title: String,
    pub top_confirm_title: String,
    pub top_cancel_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShowTip {
    pub del_tip: String,
}