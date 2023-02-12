use http_api_util::Api;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::str::FromStr;

use crate::api::CommonResp;


pub struct UserInfo;

impl Api for UserInfo {
    type Request = UserInfoRequest;

    type Response = CommonResp<UserInfoResponse>;

    const METHOD: http::Method = http::Method::GET;

    const CONST_URL: Option<&'static str> = None;

    fn dynamic_url(request: &Self::Request) -> http::Uri {
        http::Uri::from_str(&format!("https://api.bilibili.com/x/space/acc/info?mid={}", request.mid)).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct UserInfoRequest {
    #[serde(skip)]
    pub mid: u64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserInfoResponse {
    pub mid: i64,
    pub name: String,
    pub sex: String,
    pub face: String,
    // pub face_nft: i64,
    // pub face_nft_type: i64,
    pub sign: String,
    // pub rank: i64,
    pub level: i64,
    // pub jointime: i64,
    // pub moral: i64,
    // pub silence: i64,
    // pub coins: i64,
    // pub fans_badge: bool,
    // pub fans_medal: Option<FansMedal>,
    // pub official: Option<Official>,
    // pub vip: Option<Vip>,
    // pub pendant: Option<Pendant>,
    // pub nameplate: Option<Nameplate>,
    // pub user_honour_info: Option<UserHonourInfo>,
    // pub is_followed: bool,
    // pub top_photo: String,
    // pub theme: Theme,
    // pub sys_notice: Theme,
    // pub live_room: LiveRoom,
    // pub birthday: String,
    // pub school: Value,
    // pub profession: Option<Profession>,
    // pub tags: Value,
    // pub series: Series,
    // pub is_senior_member: i64,
    // pub mcn_info: Value,
    // pub gaia_res_type: i64,
    // pub gaia_data: Value,
    // pub is_risk: bool,
    // pub elec: Option<Elec>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FansMedal {
    pub show: bool,
    pub wear: bool,
    pub medal: Option<Medal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medal {
    pub uid: i64,
    pub target_id: i64,
    pub medal_id: i64,
    pub level: i64,
    pub medal_name: String,
    pub medal_color: i64,
    pub intimacy: i64,
    pub next_intimacy: i64,
    pub day_limit: i64,
    pub medal_color_start: i64,
    pub medal_color_end: i64,
    pub medal_color_border: i64,
    pub is_lighted: i64,
    pub light_status: i64,
    pub wearing_status: i64,
    pub score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Official {
    pub role: i64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vip {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub status: i64,
    pub due_date: i64,
    pub vip_pay_type: i64,
    pub theme_type: i64,
    pub label: Label,
    pub avatar_subscript: i64,
    pub nickname_color: String,
    pub role: i64,
    pub avatar_subscript_url: String,
    pub tv_vip_status: i64,
    pub tv_vip_pay_type: i64,
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
    pub use_img_label: bool,
    pub img_label_uri_hans: String,
    pub img_label_uri_hant: String,
    pub img_label_uri_hans_static: String,
    pub img_label_uri_hant_static: String,
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
pub struct Nameplate {
    pub nid: i64,
    pub name: String,
    pub image: String,
    pub image_small: String,
    pub level: String,
    pub condition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserHonourInfo {
    pub mid: i64,
    pub colour: Value,
    pub tags: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiveRoom {
    #[serde(rename = "roomStatus")]
    pub room_status: i64,
    #[serde(rename = "liveStatus")]
    pub live_status: i64,
    pub url: String,
    pub title: String,
    pub cover: String,
    pub roomid: i64,
    #[serde(rename = "roundStatus")]
    pub round_status: i64,
    pub broadcast_type: i64,
    pub watched_show: WatchedShow,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchedShow {
    pub switch: bool,
    pub num: i64,
    pub text_small: String,
    pub text_large: String,
    pub icon: String,
    pub icon_location: String,
    pub icon_web: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profession {
    pub name: String,
    pub department: String,
    pub title: String,
    pub is_show: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub user_upgrade_status: i64,
    pub show_upgrade_window: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elec {
    pub show_info: ShowInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShowInfo {
    pub show: bool,
    pub state: i64,
    pub title: String,
    pub icon: String,
    pub jump_url: String,
}