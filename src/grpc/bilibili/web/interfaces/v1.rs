/// 用户信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccInfo {
    /// 用户UID
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sex: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub sign: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountCard {
    ///
    #[prost(string, tag = "1")]
    pub mid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sex: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub rank: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub spacesta: i32,
    ///
    #[prost(string, tag = "7")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub level_info: ::core::option::Option<CardLevelInfo>,
    ///
    #[prost(message, optional, tag = "9")]
    pub pendant: ::core::option::Option<PendantInfo>,
    ///
    #[prost(message, optional, tag = "10")]
    pub nameplate: ::core::option::Option<NameplateInfo>,
    ///
    #[prost(message, optional, tag = "11")]
    pub official: ::core::option::Option<OfficialInfo>,
    ///
    #[prost(message, optional, tag = "12")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    ///
    #[prost(message, optional, tag = "13")]
    pub vip: ::core::option::Option<CardVip>,
    ///
    #[prost(int64, tag = "14")]
    pub fans: i64,
    ///
    #[prost(int64, tag = "15")]
    pub friend: i64,
    ///
    #[prost(int64, tag = "16")]
    pub attention: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityArchiveReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<Arc>,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    ///
    #[prost(message, optional, tag = "4")]
    pub req_user: ::core::option::Option<ReqUser>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub staff: ::prost::alloc::vec::Vec<Staff>,
    ///
    #[prost(message, optional, tag = "6")]
    pub right_relate: ::core::option::Option<OperationRelate>,
    ///
    #[prost(message, optional, tag = "7")]
    pub bottom_relate: ::core::option::Option<OperationRelate>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityArchiveReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub activity_key: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityEpisode {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub aid: i64,
    ///
    #[prost(string, tag = "3")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub cid: i64,
    ///
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub author: ::core::option::Option<Author>,
    ///
    #[prost(message, optional, tag = "8")]
    pub rights: ::core::option::Option<Rights>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityGame {
    ///
    #[prost(message, repeated, tag = "1")]
    pub iframes: ::prost::alloc::vec::Vec<ActivityGameIframe>,
    ///
    #[prost(string, tag = "2")]
    pub disclaimer: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub disclaimer_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityGameIframe {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub height: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityLive {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub now_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "4")]
    pub end_time: i64,
    ///
    #[prost(string, tag = "5")]
    pub hover_pic: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub hover_jump_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub break_cycle: i64,
    ///
    #[prost(message, repeated, tag = "8")]
    pub timeline: ::prost::alloc::vec::Vec<LiveTimeline>,
    ///
    #[prost(message, optional, tag = "9")]
    pub operation_relate: ::core::option::Option<OperationRelate>,
    ///
    #[prost(int64, tag = "10")]
    pub reply_type: i64,
    ///
    #[prost(int64, tag = "11")]
    pub reply_id: i64,
    ///
    #[prost(string, tag = "12")]
    pub hover_pic_close: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub gift_disclaimer: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityLiveTimeInfoReply {
    ///
    #[prost(int64, tag = "1")]
    pub now_time: i64,
    ///
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub end_time: i64,
    ///
    #[prost(message, repeated, tag = "4")]
    pub timeline: ::prost::alloc::vec::Vec<LiveTimeline>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityLiveTimeInfoReq {
    ///
    #[prost(string, tag = "1")]
    pub activity_key: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivitySeasonReply {
    ///
    #[prost(enumeration = "ActivitySeasonStatus", tag = "1")]
    pub status: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub live: ::core::option::Option<ActivityLive>,
    ///
    #[prost(message, optional, tag = "4")]
    pub subscribe: ::core::option::Option<ActivitySubscribe>,
    ///
    #[prost(message, optional, tag = "5")]
    pub game: ::core::option::Option<ActivityGame>,
    ///
    #[prost(message, optional, tag = "6")]
    pub view: ::core::option::Option<ActivityView>,
    ///
    #[prost(message, optional, tag = "7")]
    pub theme: ::core::option::Option<ActivityTheme>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivitySeasonReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub activity_key: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivitySeasonSection {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub r#type: i64,
    ///
    #[prost(message, repeated, tag = "4")]
    pub episodes: ::prost::alloc::vec::Vec<ActivityEpisode>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivitySubscribe {
    ///
    #[prost(bool, tag = "1")]
    pub status: bool,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub button_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub button_selected_title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub season_stat_view: i64,
    ///
    #[prost(int64, tag = "6")]
    pub season_stat_danmaku: i64,
    ///
    #[prost(enumeration = "OrderType", tag = "7")]
    pub order_type: i32,
    #[prost(oneof = "activity_subscribe::Param", tags = "8, 9, 10")]
    pub param: ::core::option::Option<activity_subscribe::Param>,
}
/// Nested message and enum types in `ActivitySubscribe`.
pub mod activity_subscribe {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Param {
        ///
        #[prost(message, tag = "8")]
        ReserveActivityParam(super::ReserveActivityParam),
        ///
        #[prost(message, tag = "9")]
        FavSeasonParam(super::FavSeasonParam),
        ///
        #[prost(message, tag = "10")]
        JumpUrlParam(super::JumpUrlParam),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityTheme {
    ///
    #[prost(string, tag = "1")]
    pub base_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub loading_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub operated_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub default_element_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub hover_element_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub selected_element_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub base_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub info_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub mask_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub page_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub center_logo_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub page_bg_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub decorations2233_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub main_banner_bg_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub main_banner_title_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub like_animation_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub combo_like_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub combo_coin_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "19")]
    pub combo_fav_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "20")]
    pub arrow_btn_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "21")]
    pub share_icon_bg_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "22")]
    pub live_list_location_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub live_list_location_img_active: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "24")]
    pub player_loading_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "25")]
    pub share_img: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "26")]
    pub kv_color: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityView {
    ///
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<Arc>,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub staff: ::prost::alloc::vec::Vec<Staff>,
    ///
    #[prost(message, optional, tag = "5")]
    pub req_user: ::core::option::Option<ReqUser>,
    ///
    #[prost(message, optional, tag = "6")]
    pub right_relate: ::core::option::Option<OperationRelate>,
    ///
    #[prost(message, optional, tag = "7")]
    pub bottom_relate: ::core::option::Option<OperationRelate>,
    ///
    #[prost(message, repeated, tag = "8")]
    pub sections: ::prost::alloc::vec::Vec<ActivitySeasonSection>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arc {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub videos: i64,
    ///
    #[prost(int32, tag = "3")]
    pub type_id: i32,
    ///
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub copyright: i32,
    ///
    #[prost(string, tag = "6")]
    pub pic: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub pubdate: i64,
    ///
    #[prost(int64, tag = "9")]
    pub ctime: i64,
    ///
    #[prost(string, tag = "10")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub state: i32,
    ///
    #[prost(int32, tag = "12")]
    pub access: i32,
    ///
    #[prost(int32, tag = "13")]
    pub attribute: i32,
    ///
    #[prost(string, tag = "14")]
    pub tag: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "15")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int64, tag = "16")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "17")]
    pub mission_id: i64,
    ///
    #[prost(int64, tag = "18")]
    pub order_id: i64,
    ///
    #[prost(string, tag = "19")]
    pub redirect_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "20")]
    pub forward: i64,
    ///
    #[prost(message, optional, tag = "21")]
    pub rights: ::core::option::Option<Rights>,
    ///
    #[prost(message, optional, tag = "22")]
    pub author: ::core::option::Option<Author>,
    ///
    #[prost(message, optional, tag = "23")]
    pub stat: ::core::option::Option<Stat>,
    ///
    #[prost(string, tag = "24")]
    pub report_result: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "25")]
    pub dynamic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "26")]
    pub first_cid: i64,
    ///
    #[prost(message, optional, tag = "27")]
    pub dimension: ::core::option::Option<Dimension>,
    ///
    #[prost(message, repeated, tag = "28")]
    pub staff_info: ::prost::alloc::vec::Vec<StaffInfo>,
    ///
    #[prost(int64, tag = "29")]
    pub season_id: i64,
    ///
    #[prost(message, repeated, tag = "30")]
    pub desc_v2: ::prost::alloc::vec::Vec<DescV2>,
    ///
    #[prost(bool, tag = "31")]
    pub is_chargeable_season: bool,
    ///
    #[prost(bool, tag = "32")]
    pub is_blooper: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Author {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Card {
    ///
    #[prost(message, optional, tag = "1")]
    pub card: ::core::option::Option<AccountCard>,
    ///
    #[prost(message, optional, tag = "2")]
    pub space: ::core::option::Option<Space>,
    ///
    #[prost(bool, tag = "3")]
    pub following: bool,
    ///
    #[prost(int64, tag = "4")]
    pub archive_count: i64,
    ///
    #[prost(int32, tag = "5")]
    pub article_count: i32,
    ///
    #[prost(int64, tag = "6")]
    pub follower: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardLevelInfo {
    ///
    #[prost(int32, tag = "1")]
    pub cur: i32,
    ///
    #[prost(int32, tag = "2")]
    pub min: i32,
    ///
    #[prost(int32, tag = "3")]
    pub now_exp: i32,
    ///
    #[prost(int32, tag = "4")]
    pub next_exp: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardVip {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub due_remark: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub access_status: i32,
    ///
    #[prost(int32, tag = "4")]
    pub vip_status: i32,
    ///
    #[prost(string, tag = "5")]
    pub vip_status_warn: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub theme_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickActivitySeasonReq {
    ///
    #[prost(enumeration = "OrderType", tag = "1")]
    pub order_type: i32,
    ///
    #[prost(string, tag = "5")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub action: i64,
    #[prost(oneof = "click_activity_season_req::Param", tags = "2, 3, 4")]
    pub param: ::core::option::Option<click_activity_season_req::Param>,
}
/// Nested message and enum types in `ClickActivitySeasonReq`.
pub mod click_activity_season_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Param {
        ///
        #[prost(message, tag = "2")]
        ReserveActivityParam(super::ReserveActivityParam),
        ///
        #[prost(message, tag = "3")]
        FavSeasonParam(super::FavSeasonParam),
        ///
        #[prost(message, tag = "4")]
        JumpUrlParam(super::JumpUrlParam),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescV2 {
    ///
    #[prost(string, tag = "1")]
    pub raw_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    ///
    #[prost(int64, tag = "3")]
    pub biz_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    ///
    #[prost(int64, tag = "1")]
    pub width: i64,
    ///
    #[prost(int64, tag = "2")]
    pub height: i64,
    ///
    #[prost(int64, tag = "3")]
    pub rotate: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavSeasonParam {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub page: ::core::option::Option<ReplyPage>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub replies: ::prost::alloc::vec::Vec<Reply>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JumpUrlParam {
    ///
    #[prost(string, tag = "1")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveTimeline {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub end_time: i64,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub h5_cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameplateInfo {
    ///
    #[prost(int32, tag = "1")]
    pub nid: i32,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub image_small: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub level: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub condition: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialInfo {
    ///
    #[prost(int32, tag = "1")]
    pub role: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationRelate {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub relate_item: ::prost::alloc::vec::Vec<RelateItem>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub ai_relate_item: ::prost::alloc::vec::Vec<Relate>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    ///
    #[prost(int64, tag = "1")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub page: i32,
    ///
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub part: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub duration: i64,
    ///
    #[prost(string, tag = "6")]
    pub vid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub weblink: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub dimension: ::core::option::Option<Dimension>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendantInfo {
    ///
    #[prost(int32, tag = "1")]
    pub pid: i32,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub expire: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonStyle {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relate {
    ///
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<Arc>,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub season_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelateItem {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    ///
    #[prost(int64, tag = "1")]
    pub attribute: i64,
    ///
    #[prost(int64, tag = "3")]
    pub special: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    ///
    #[prost(int64, tag = "1")]
    pub rpid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "4")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub root: i64,
    ///
    #[prost(int64, tag = "6")]
    pub parent: i64,
    ///
    #[prost(int64, tag = "7")]
    pub dialog: i64,
    ///
    #[prost(int32, tag = "8")]
    pub count: i32,
    ///
    #[prost(int32, tag = "9")]
    pub rcount: i32,
    ///
    #[prost(int32, tag = "10")]
    pub floor: i32,
    ///
    #[prost(int32, tag = "11")]
    pub state: i32,
    ///
    #[prost(int32, tag = "12")]
    pub fans_grade: i32,
    ///
    #[prost(int32, tag = "13")]
    pub attr: i32,
    ///
    #[prost(int64, tag = "14")]
    pub ctime: i64,
    ///
    #[prost(string, tag = "15")]
    pub rpid_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub root_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub parent_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub dialog_str: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "19")]
    pub like: i32,
    ///
    #[prost(int32, tag = "20")]
    pub hate: i32,
    ///
    #[prost(int32, tag = "21")]
    pub action: i32,
    ///
    #[prost(message, optional, tag = "22")]
    pub member: ::core::option::Option<ReplyMember>,
    ///
    #[prost(message, optional, tag = "23")]
    pub content: ::core::option::Option<ReplyContent>,
    ///
    #[prost(message, repeated, tag = "24")]
    pub replies: ::prost::alloc::vec::Vec<Reply>,
    ///
    #[prost(int32, tag = "25")]
    pub assist: i32,
    ///
    #[prost(message, optional, tag = "26")]
    pub folder: ::core::option::Option<ReplyFolder>,
    ///
    #[prost(message, optional, tag = "27")]
    pub up_action: ::core::option::Option<ReplyUpAction>,
    ///
    #[prost(message, optional, tag = "28")]
    pub label: ::core::option::Option<ReplyLabel>,
    ///
    #[prost(string, tag = "29")]
    pub raw_input: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "30")]
    pub show_follow: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyContent {
    ///
    #[prost(int64, tag = "1")]
    pub rp_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub vote: ::core::option::Option<ReplyVote>,
    ///
    #[prost(string, repeated, tag = "5")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "6")]
    pub ip: i32,
    ///
    #[prost(int32, tag = "7")]
    pub plat: i32,
    ///
    #[prost(string, tag = "8")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub version: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "10")]
    pub members: ::prost::alloc::vec::Vec<ReplyMemberInfo>,
    ///
    #[prost(map = "string, message", tag = "11")]
    pub emote: ::std::collections::HashMap<::prost::alloc::string::String, ReplyEmote>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyEmote {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub package_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub state: i64,
    ///
    #[prost(int64, tag = "4")]
    pub r#type: i64,
    ///
    #[prost(int64, tag = "5")]
    pub attr: i64,
    ///
    #[prost(string, tag = "6")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub meta: ::core::option::Option<ReplyEmoteMeta>,
    ///
    #[prost(int64, tag = "9")]
    pub ctime: i64,
    ///
    #[prost(int64, tag = "10")]
    pub mtime: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyEmoteMeta {
    ///
    #[prost(enumeration = "ReplyEmoteMetaSize", tag = "1")]
    pub size: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyFansDetail {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub medal_id: i32,
    ///
    #[prost(string, tag = "3")]
    pub medal_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub score: i32,
    ///
    #[prost(int32, tag = "5")]
    pub level: i32,
    ///
    #[prost(int32, tag = "6")]
    pub intimacy: i32,
    ///
    #[prost(int32, tag = "7")]
    pub status: i32,
    ///
    #[prost(int32, tag = "8")]
    pub received: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyFolder {
    ///
    #[prost(bool, tag = "1")]
    pub has_folded: bool,
    ///
    #[prost(bool, tag = "2")]
    pub is_folded: bool,
    ///
    #[prost(string, tag = "3")]
    pub rule: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyLabel {
    ///
    #[prost(int64, tag = "1")]
    pub rpid: i64,
    ///
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text_color_night_mode: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub bg_color_night_mode: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub position: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyLevelInfo {
    ///
    #[prost(int32, tag = "1")]
    pub cur: i32,
    ///
    #[prost(int32, tag = "2")]
    pub min: i32,
    ///
    #[prost(int32, tag = "3")]
    pub now_exp: i32,
    ///
    #[prost(int32, tag = "4")]
    pub next_exp: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyMember {
    ///
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<ReplyMemberInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub fans_detail: ::core::option::Option<ReplyFansDetail>,
    ///
    #[prost(int32, tag = "3")]
    pub following: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyMemberInfo {
    ///
    #[prost(int32, tag = "1")]
    pub role: i32,
    ///
    #[prost(string, tag = "2")]
    pub mid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub sex: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub rank: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub display_rank: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub level_info: ::core::option::Option<ReplyLevelInfo>,
    ///
    #[prost(message, optional, tag = "10")]
    pub pendant: ::core::option::Option<PendantInfo>,
    ///
    #[prost(message, optional, tag = "11")]
    pub nameplate: ::core::option::Option<NameplateInfo>,
    ///
    #[prost(message, optional, tag = "12")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    ///
    #[prost(message, optional, tag = "13")]
    pub vip: ::core::option::Option<ReplyVip>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyPage {
    ///
    #[prost(int64, tag = "1")]
    pub acount: i64,
    ///
    #[prost(int64, tag = "2")]
    pub count: i64,
    ///
    #[prost(int64, tag = "3")]
    pub num: i64,
    ///
    #[prost(int64, tag = "4")]
    pub size: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyUpAction {
    ///
    #[prost(bool, tag = "1")]
    pub like: bool,
    ///
    #[prost(bool, tag = "2")]
    pub reply: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyVip {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub due_date: i64,
    ///
    #[prost(string, tag = "3")]
    pub due_remark: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub access_status: i32,
    ///
    #[prost(int32, tag = "5")]
    pub vip_status: i32,
    ///
    #[prost(string, tag = "6")]
    pub vip_status_warn: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub theme_type: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub label: ::core::option::Option<VipLabel>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyVote {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cnt: i32,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "5")]
    pub deleted: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqUser {
    ///
    #[prost(bool, tag = "1")]
    pub favorite: bool,
    ///
    #[prost(bool, tag = "2")]
    pub like: bool,
    ///
    #[prost(bool, tag = "3")]
    pub dislike: bool,
    ///
    #[prost(int64, tag = "4")]
    pub multiply: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveActivityParam {
    ///
    #[prost(int64, tag = "1")]
    pub reserve_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub oid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rights {
    ///
    #[prost(int32, tag = "1")]
    pub bp: i32,
    ///
    #[prost(int32, tag = "2")]
    pub elec: i32,
    ///
    #[prost(int32, tag = "3")]
    pub download: i32,
    ///
    #[prost(int32, tag = "4")]
    pub movie: i32,
    ///
    #[prost(int32, tag = "5")]
    pub pay: i32,
    ///
    #[prost(int32, tag = "6")]
    pub hd5: i32,
    ///
    #[prost(int32, tag = "7")]
    pub no_reprint: i32,
    ///
    #[prost(int32, tag = "8")]
    pub autoplay: i32,
    ///
    #[prost(int32, tag = "9")]
    pub ugc_pay: i32,
    ///
    #[prost(int32, tag = "10")]
    pub is_cooperation: i32,
    ///
    #[prost(int32, tag = "11")]
    pub ugc_pay_preview: i32,
    ///
    #[prost(int32, tag = "12")]
    pub arc_pay: i32,
    ///
    #[prost(int32, tag = "13")]
    pub free_watch: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonEpisode {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub section_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub cid: i64,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub attribute: i64,
    ///
    #[prost(message, optional, tag = "8")]
    pub arc: ::core::option::Option<Arc>,
    ///
    #[prost(message, optional, tag = "9")]
    pub page: ::core::option::Option<Page>,
    ///
    #[prost(string, tag = "10")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "11")]
    pub badge_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonSection {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub id: i64,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub r#type: i64,
    ///
    #[prost(message, repeated, tag = "5")]
    pub episodes: ::prost::alloc::vec::Vec<SeasonEpisode>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonStat {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub view: i32,
    ///
    #[prost(int32, tag = "3")]
    pub danmaku: i32,
    ///
    #[prost(int32, tag = "4")]
    pub reply: i32,
    ///
    #[prost(int32, tag = "5")]
    pub fav: i32,
    ///
    #[prost(int32, tag = "6")]
    pub coin: i32,
    ///
    #[prost(int32, tag = "7")]
    pub share: i32,
    ///
    #[prost(int32, tag = "8")]
    pub now_rank: i32,
    ///
    #[prost(int32, tag = "9")]
    pub his_rank: i32,
    ///
    #[prost(int32, tag = "10")]
    pub like: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Space {
    ///
    #[prost(string, tag = "1")]
    pub s_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub l_img: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Staff {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub vip: ::core::option::Option<VipInfo>,
    ///
    #[prost(message, optional, tag = "6")]
    pub official: ::core::option::Option<OfficialInfo>,
    ///
    #[prost(int64, tag = "7")]
    pub follower: i64,
    ///
    #[prost(int32, tag = "8")]
    pub label_style: i32,
    ///
    #[prost(message, optional, tag = "9")]
    pub relation: ::core::option::Option<Relation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaffInfo {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub view: i32,
    ///
    #[prost(int32, tag = "3")]
    pub danmaku: i32,
    ///
    #[prost(int32, tag = "4")]
    pub reply: i32,
    ///
    #[prost(int32, tag = "5")]
    pub fav: i32,
    ///
    #[prost(int32, tag = "6")]
    pub coin: i32,
    ///
    #[prost(int32, tag = "7")]
    pub share: i32,
    ///
    #[prost(int32, tag = "8")]
    pub now_rank: i32,
    ///
    #[prost(int32, tag = "9")]
    pub his_rank: i32,
    ///
    #[prost(int32, tag = "10")]
    pub like: i32,
    ///
    #[prost(int32, tag = "11")]
    pub dislike: i32,
    ///
    #[prost(string, tag = "12")]
    pub evaluation: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub argue_msg: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subtitle {
    ///
    #[prost(bool, tag = "1")]
    pub allow_submit: bool,
    ///
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<SubtitleItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubtitleItem {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub lan: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub lan_doc: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub is_lock: bool,
    ///
    #[prost(int64, tag = "5")]
    pub author_mid: i64,
    ///
    #[prost(string, tag = "6")]
    pub subtitle_url: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub author: ::core::option::Option<AccInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub head_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub short_content: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "8")]
    pub state: i32,
    ///
    #[prost(int64, tag = "9")]
    pub ctime: i64,
    ///
    #[prost(message, optional, tag = "10")]
    pub tag_count: ::core::option::Option<TagCount>,
    ///
    #[prost(int32, tag = "11")]
    pub is_atten: i32,
    ///
    #[prost(int64, tag = "12")]
    pub likes: i64,
    ///
    #[prost(int64, tag = "13")]
    pub hates: i64,
    ///
    #[prost(int32, tag = "14")]
    pub attribute: i32,
    ///
    #[prost(int32, tag = "15")]
    pub liked: i32,
    ///
    #[prost(int32, tag = "16")]
    pub hated: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagCount {
    ///
    #[prost(int64, tag = "1")]
    pub view: i64,
    ///
    #[prost(int64, tag = "2")]
    pub r#use: i64,
    ///
    #[prost(int64, tag = "3")]
    pub atten: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcPayAsset {
    ///
    #[prost(int64, tag = "1")]
    pub price: i64,
    ///
    #[prost(map = "string, int64", tag = "2")]
    pub platform_price: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcSeason {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub mid: i64,
    ///
    #[prost(string, tag = "5")]
    pub intro: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub sign_state: i32,
    ///
    #[prost(int64, tag = "7")]
    pub attribute: i64,
    ///
    #[prost(message, repeated, tag = "8")]
    pub sections: ::prost::alloc::vec::Vec<SeasonSection>,
    ///
    #[prost(message, optional, tag = "9")]
    pub stat: ::core::option::Option<SeasonStat>,
    ///
    #[prost(int64, tag = "10")]
    pub ep_count: i64,
    ///
    #[prost(int64, tag = "11")]
    pub season_type: i64,
    ///
    #[prost(bool, tag = "12")]
    pub is_pay_season: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct View {
    ///
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<Arc>,
    ///
    #[prost(bool, tag = "2")]
    pub no_cache: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub pages: ::prost::alloc::vec::Vec<Page>,
    ///
    #[prost(message, optional, tag = "4")]
    pub subtitle: ::core::option::Option<Subtitle>,
    ///
    #[prost(message, optional, tag = "5")]
    pub asset: ::core::option::Option<UgcPayAsset>,
    ///
    #[prost(message, optional, tag = "6")]
    pub label: ::core::option::Option<ViewLabel>,
    ///
    #[prost(message, repeated, tag = "7")]
    pub staff: ::prost::alloc::vec::Vec<Staff>,
    ///
    #[prost(message, optional, tag = "8")]
    pub ugc_season: ::core::option::Option<UgcSeason>,
    ///
    #[prost(int64, tag = "9")]
    pub stein_guide_cid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewDetailReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub view: ::core::option::Option<View>,
    ///
    #[prost(message, optional, tag = "2")]
    pub card: ::core::option::Option<Card>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    ///
    #[prost(message, optional, tag = "4")]
    pub reply: ::core::option::Option<HotReply>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub related: ::prost::alloc::vec::Vec<Arc>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewDetailReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewLabel {
    ///
    #[prost(int64, tag = "1")]
    pub r#type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipInfo {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub status: i32,
    ///
    #[prost(int32, tag = "3")]
    pub vip_pay_type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub theme_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    ///
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActivitySeasonStatus {
    ///
    StatusNone = 0,
    ///
    StatusLive = 1,
    ///
    StatusView = 2,
}
impl ActivitySeasonStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActivitySeasonStatus::StatusNone => "StatusNone",
            ActivitySeasonStatus::StatusLive => "StatusLive",
            ActivitySeasonStatus::StatusView => "StatusView",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "StatusNone" => Some(Self::StatusNone),
            "StatusLive" => Some(Self::StatusLive),
            "StatusView" => Some(Self::StatusView),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    ///
    TypeNone = 0,
    ///
    TypeOrderActivity = 1,
    ///
    TypeFavSeason = 2,
    ///
    TypeClick = 3,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::TypeNone => "TypeNone",
            OrderType::TypeOrderActivity => "TypeOrderActivity",
            OrderType::TypeFavSeason => "TypeFavSeason",
            OrderType::TypeClick => "TypeClick",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TypeNone" => Some(Self::TypeNone),
            "TypeOrderActivity" => Some(Self::TypeOrderActivity),
            "TypeFavSeason" => Some(Self::TypeFavSeason),
            "TypeClick" => Some(Self::TypeClick),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReplyEmoteMetaSize {
    ///
    EmoteMetaSizeUnspecified = 0,
    ///
    EmoteMetaSizeSmall = 1,
    ///
    EmoteMetaSizeBig = 2,
}
impl ReplyEmoteMetaSize {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReplyEmoteMetaSize::EmoteMetaSizeUnspecified => "EMOTE_META_SIZE_UNSPECIFIED",
            ReplyEmoteMetaSize::EmoteMetaSizeSmall => "EMOTE_META_SIZE_SMALL",
            ReplyEmoteMetaSize::EmoteMetaSizeBig => "EMOTE_META_SIZE_BIG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EMOTE_META_SIZE_UNSPECIFIED" => Some(Self::EmoteMetaSizeUnspecified),
            "EMOTE_META_SIZE_SMALL" => Some(Self::EmoteMetaSizeSmall),
            "EMOTE_META_SIZE_BIG" => Some(Self::EmoteMetaSizeBig),
            _ => None,
        }
    }
}
