///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdInfo {
    ///
    #[prost(int64, tag = "1")]
    pub creative_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub creative_type: i32,
    ///
    #[prost(int32, tag = "3")]
    pub card_type: i32,
    ///
    #[prost(message, optional, tag = "4")]
    pub creative_content: ::core::option::Option<CreativeContent>,
    ///
    #[prost(string, tag = "5")]
    pub ad_cb: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub resource: i64,
    ///
    #[prost(int32, tag = "7")]
    pub source: i32,
    ///
    #[prost(string, tag = "8")]
    pub request_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "9")]
    pub is_ad: bool,
    ///
    #[prost(int64, tag = "10")]
    pub cm_mark: i64,
    ///
    #[prost(int32, tag = "11")]
    pub index: i32,
    ///
    #[prost(bool, tag = "12")]
    pub is_ad_loc: bool,
    ///
    #[prost(int32, tag = "13")]
    pub card_index: i32,
    ///
    #[prost(string, tag = "14")]
    pub client_ip: ::prost::alloc::string::String,
    ///
    #[prost(bytes = "vec", tag = "15")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
    ///
    #[prost(int32, tag = "16")]
    pub creative_style: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreativeContent {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub video_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub username: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub image_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub image_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub log_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub log_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub click_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub show_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Args {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub up_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub up_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub rid: i32,
    ///
    #[prost(string, tag = "5")]
    pub rname: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub tid: i64,
    ///
    #[prost(string, tag = "7")]
    pub tname: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub state: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub converge_type: i32,
    ///
    #[prost(int64, tag = "11")]
    pub aid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Avatar {
    ///
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "5")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub event_v2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub defalut_cover: i32,
}
/// 条目基本信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base {
    /// 卡片类型
    #[prost(string, tag = "1")]
    pub card_type: ::prost::alloc::string::String,
    /// 卡片跳转类型?
    #[prost(string, tag = "2")]
    pub card_goto: ::prost::alloc::string::String,
    /// 跳转类型
    /// av:视频稿件 mid:用户空间
    #[prost(string, tag = "3")]
    pub goto: ::prost::alloc::string::String,
    /// 目标参数
    #[prost(string, tag = "4")]
    pub param: ::prost::alloc::string::String,
    /// 封面url
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub three_point: ::core::option::Option<ThreePoint>,
    ///
    #[prost(message, optional, tag = "9")]
    pub args: ::core::option::Option<Args>,
    ///
    #[prost(message, optional, tag = "10")]
    pub player_args: ::core::option::Option<PlayerArgs>,
    /// 条目排位序号
    #[prost(int64, tag = "11")]
    pub idx: i64,
    ///
    #[prost(message, optional, tag = "12")]
    pub ad_info: ::core::option::Option<AdInfo>,
    ///
    #[prost(message, optional, tag = "13")]
    pub mask: ::core::option::Option<Mask>,
    /// 来源标识
    /// recommend:推荐 operation:管理?
    #[prost(string, tag = "14")]
    pub from_type: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "15")]
    pub three_point_v2: ::prost::alloc::vec::Vec<ThreePointV2>,
    ///
    #[prost(message, repeated, tag = "16")]
    pub three_point_v3: ::prost::alloc::vec::Vec<ThreePointV3>,
    ///
    #[prost(message, optional, tag = "17")]
    pub desc_button: ::core::option::Option<Button>,
    /// 三点v4
    #[prost(message, optional, tag = "18")]
    pub three_point_v4: ::core::option::Option<ThreePointV4>,
    ///
    #[prost(message, optional, tag = "19")]
    pub up_args: ::core::option::Option<UpArgs>,
}
/// 按钮信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    /// 文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 参数
    #[prost(string, tag = "2")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 事件
    #[prost(string, tag = "4")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub selected: i32,
    /// 类型
    #[prost(int32, tag = "6")]
    pub r#type: i32,
    /// 事件v2
    #[prost(string, tag = "7")]
    pub event_v2: ::prost::alloc::string::String,
    /// 关系信息
    #[prost(message, optional, tag = "8")]
    pub relation: ::core::option::Option<Relation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DislikeReason {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeButton {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub count: i32,
    ///
    #[prost(bool, tag = "3")]
    pub show_count: bool,
    ///
    #[prost(string, tag = "4")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub selected: i32,
    ///
    #[prost(string, tag = "6")]
    pub event_v2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mask {
    ///
    #[prost(message, optional, tag = "1")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(message, optional, tag = "2")]
    pub button: ::core::option::Option<Button>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerArgs {
    ///
    #[prost(int32, tag = "1")]
    pub is_live: i32,
    ///
    #[prost(int64, tag = "2")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub sub_type: i32,
    ///
    #[prost(int64, tag = "5")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "7")]
    pub ep_id: i64,
    ///
    #[prost(int32, tag = "8")]
    pub is_preview: i32,
    ///
    #[prost(string, tag = "9")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "11")]
    pub season_id: i64,
}
/// 标签框信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonStyle {
    /// 文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 文字颜色
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    /// 背景色
    #[prost(string, tag = "3")]
    pub bg_color: ::prost::alloc::string::String,
    /// 边框色
    #[prost(string, tag = "4")]
    pub border_color: ::prost::alloc::string::String,
    /// 图标url
    #[prost(string, tag = "5")]
    pub icon_url: ::prost::alloc::string::String,
    /// 文字颜色-夜间
    #[prost(string, tag = "6")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 背景色-夜间
    #[prost(string, tag = "7")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 边框色-夜间
    #[prost(string, tag = "8")]
    pub border_color_night: ::prost::alloc::string::String,
    /// 图标url-夜间
    #[prost(string, tag = "9")]
    pub icon_night_url: ::prost::alloc::string::String,
    /// 背景风格id
    /// 1:无背景 2:有背景
    #[prost(int32, tag = "10")]
    pub bg_style: i32,
    ///
    #[prost(string, tag = "11")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub icon_bg_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub event_v2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "15")]
    pub right_icon_type: i32,
    ///
    #[prost(string, tag = "16")]
    pub left_icon_type: ::prost::alloc::string::String,
}
/// 关系信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    /// 关系状态
    #[prost(int32, tag = "1")]
    pub status: i32,
    /// 是否关注
    #[prost(int32, tag = "2")]
    pub is_follow: i32,
    /// 是否粉丝
    #[prost(int32, tag = "3")]
    pub is_followed: i32,
}
/// 分享面板信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharePlane {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 副标贴文案
    #[prost(string, tag = "2")]
    pub share_subtitle: ::prost::alloc::string::String,
    /// 备注
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    /// 封面url
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    /// 稿件avid
    #[prost(int64, tag = "5")]
    pub aid: i64,
    /// 稿件bvid
    #[prost(string, tag = "6")]
    pub bvid: ::prost::alloc::string::String,
    /// 允许分享方式
    #[prost(map = "string, bool", tag = "7")]
    pub share_to: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
    /// UP主昵称
    #[prost(string, tag = "8")]
    pub author: ::prost::alloc::string::String,
    /// UP主mid
    #[prost(int64, tag = "9")]
    pub author_id: i64,
    /// 短连接
    #[prost(string, tag = "10")]
    pub short_link: ::prost::alloc::string::String,
    /// 播放次数文案
    #[prost(string, tag = "11")]
    pub play_number: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "12")]
    pub first_cid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePoint {
    ///
    #[prost(message, repeated, tag = "1")]
    pub dislike_reasons: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub feedbacks: ::prost::alloc::vec::Vec<DislikeReason>,
    /// 稍后再看
    #[prost(int32, tag = "3")]
    pub watch_later: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointV2 {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub reasons: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointV3 {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub selected_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub reasons: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub id: i64,
    ///
    #[prost(int32, tag = "7")]
    pub selected: i32,
    ///
    #[prost(string, tag = "8")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub selected_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub default_id: i32,
}
/// 三点v4
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointV4 {
    /// 分享面板信息
    #[prost(message, optional, tag = "1")]
    pub share_plane: ::core::option::Option<SharePlane>,
    /// 稍后再看
    #[prost(message, optional, tag = "2")]
    pub watch_later: ::core::option::Option<WatchLater>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Up {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(int32, tag = "5")]
    pub official_icon: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub desc_button: ::core::option::Option<Button>,
    ///
    #[prost(string, tag = "7")]
    pub cooperation: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpArgs {
    ///
    #[prost(int64, tag = "1")]
    pub up_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub up_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub up_face: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub selected: i64,
}
/// 稍后再看信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchLater {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 稿件bvid
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV5 {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_gif: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub up: ::core::option::Option<Up>,
    /// 封面右下角标文案
    #[prost(string, tag = "4")]
    pub cover_right_text_1: ::prost::alloc::string::String,
    /// 右侧文案1
    #[prost(string, tag = "5")]
    pub right_desc_1: ::prost::alloc::string::String,
    /// 右侧文案2
    #[prost(string, tag = "6")]
    pub right_desc_2: ::prost::alloc::string::String,
    /// 右侧推荐原因标签框
    #[prost(message, optional, tag = "7")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "8")]
    pub hotword_entrance: ::core::option::Option<HotwordEntrance>,
    /// 直播小卡的角标
    #[prost(message, optional, tag = "9")]
    pub corner_mark_style: ::core::option::Option<ReasonStyle>,
    /// 右侧文案1图标id
    #[prost(int32, tag = "10")]
    pub right_icon_1: i32,
    /// 右侧文案2图标id
    #[prost(int32, tag = "11")]
    pub right_icon_2: i32,
    /// 左上角角标
    #[prost(message, optional, tag = "12")]
    pub left_corner_mark_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(string, tag = "13")]
    pub cover_right_text_content_description: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub right_desc1_content_description: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV5Ad {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_gif: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub up: ::core::option::Option<Up>,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub right_desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub right_desc2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "8")]
    pub hotword_entrance: ::core::option::Option<HotwordEntrance>,
    ///
    #[prost(message, optional, tag = "9")]
    pub corner_mark_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(int32, tag = "10")]
    pub right_icon1: i32,
    ///
    #[prost(int32, tag = "11")]
    pub right_icon2: i32,
    ///
    #[prost(message, optional, tag = "12")]
    pub left_corner_mark_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(string, tag = "13")]
    pub cover_right_text_content_description: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub right_desc1_content_description: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotwordEntrance {
    ///
    #[prost(int64, tag = "1")]
    pub hotword_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub hot_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub h5_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCoverV1 {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_gif: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cover_badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub top_rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub bottom_rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub official_icon: i32,
    ///
    #[prost(int32, tag = "12")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "13")]
    pub top_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "14")]
    pub bottom_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "15")]
    pub rcmd_reason_style_v2: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "16")]
    pub left_cover_badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "17")]
    pub right_cover_badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(string, tag = "18")]
    pub cover_badge_2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "19")]
    pub like_button: ::core::option::Option<LikeButton>,
    ///
    #[prost(int32, tag = "20")]
    pub title_single_line: i32,
    ///
    #[prost(string, tag = "21")]
    pub cover_right_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreeItemAllV2 {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(message, optional, tag = "2")]
    pub top_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub item: ::prost::alloc::vec::Vec<TwoItemHv1Item>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwoItemHv1Item {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub args: ::core::option::Option<Args>,
    ///
    #[prost(string, tag = "6")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "9")]
    pub cover_right_text: ::prost::alloc::string::String,
}
/// 推荐
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdOneItem {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    /// 标签框信息
    #[prost(message, optional, tag = "2")]
    pub top_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    /// 小封面推荐内容信息
    #[prost(message, optional, tag = "3")]
    pub item: ::core::option::Option<SmallCoverRcmdItem>,
}
/// 小封面推荐内容信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverRcmdItem {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面url
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 参数
    #[prost(string, tag = "4")]
    pub param: ::prost::alloc::string::String,
    /// 跳转类型
    /// av:视频稿件
    #[prost(string, tag = "5")]
    pub goto: ::prost::alloc::string::String,
    /// 封面右下角标文案
    #[prost(string, tag = "6")]
    pub cover_right_text1: ::prost::alloc::string::String,
    /// 右侧文案1
    #[prost(string, tag = "7")]
    pub right_desc1: ::prost::alloc::string::String,
    /// 右侧文案2
    #[prost(string, tag = "8")]
    pub right_desc2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub cover_gif: ::prost::alloc::string::String,
    /// 右侧文案1图标id
    #[prost(int32, tag = "10")]
    pub right_icon1: i32,
    /// 右侧文案2图标id
    #[prost(int32, tag = "11")]
    pub right_icon2: i32,
    ///
    #[prost(string, tag = "12")]
    pub cover_right_text_content_description: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub right_desc1_content_description: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreeItemV1 {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(int32, tag = "2")]
    pub title_icon: i32,
    ///
    #[prost(string, tag = "3")]
    pub more_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub more_text: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "5")]
    pub items: ::prost::alloc::vec::Vec<ThreeItemV1Item>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreeItemV1Item {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_left_icon: i32,
    ///
    #[prost(string, tag = "4")]
    pub desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub desc2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub badge: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotTopicItem {
    ///
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotTopic {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<HotTopicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicHot {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub top_left_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub more_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub more_text: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "7")]
    pub covers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "8")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub top_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiddleCoverV3 {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub cover_badge_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCoverV4 {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text_3: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_badge: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "7")]
    pub up: ::core::option::Option<Up>,
    ///
    #[prost(string, tag = "8")]
    pub short_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub share_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub play_number: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub sub_param: ::prost::alloc::string::String,
}
/// 热门列表顶部按钮
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopularTopEntrance {
    /// 条目基本信息
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    /// 按钮项
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<EntranceItem>,
}
/// 热门列表按钮信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntranceItem {
    /// 跳转类型
    #[prost(string, tag = "1")]
    pub goto: ::prost::alloc::string::String,
    /// 图标url
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 入口模块id
    #[prost(string, tag = "4")]
    pub module_id: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// 入口id
    #[prost(int64, tag = "6")]
    pub entrance_id: i64,
    /// 气泡信息
    #[prost(message, optional, tag = "7")]
    pub bubble: ::core::option::Option<Bubble>,
    /// 入口类型
    /// 1:代表分品类热门
    #[prost(int32, tag = "8")]
    pub entrance_type: i32,
}
/// 气泡信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bubble {
    /// 文案
    #[prost(string, tag = "1")]
    pub bubble_content: ::prost::alloc::string::String,
    /// 版本
    #[prost(int32, tag = "2")]
    pub version: i32,
    /// 起始时间
    #[prost(int64, tag = "3")]
    pub stime: i64,
}
/// 卡片信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Card {
    #[prost(oneof = "card::Item", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub item: ::core::option::Option<card::Item>,
}
/// Nested message and enum types in `Card`.
pub mod card {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        /// 小封面条目
        #[prost(message, tag = "1")]
        SmallCoverV5(super::SmallCoverV5),
        ///
        #[prost(message, tag = "2")]
        LargeCoverV1(super::LargeCoverV1),
        ///
        #[prost(message, tag = "3")]
        ThreeItemAllV2(super::ThreeItemAllV2),
        ///
        #[prost(message, tag = "4")]
        ThreeItemV1(super::ThreeItemV1),
        ///
        #[prost(message, tag = "5")]
        HotTopic(super::HotTopic),
        ///
        #[prost(message, tag = "6")]
        ThreeItemHV5(super::DynamicHot),
        ///
        #[prost(message, tag = "7")]
        MiddleCoverV3(super::MiddleCoverV3),
        ///
        #[prost(message, tag = "8")]
        LargeCoverV4(super::LargeCoverV4),
        /// 热门列表顶部按钮
        #[prost(message, tag = "9")]
        PopularTopEntrance(super::PopularTopEntrance),
        ///
        #[prost(message, tag = "10")]
        RcmdOneItem(super::RcmdOneItem),
        ///
        #[prost(message, tag = "11")]
        SmallCoverV5Ad(super::SmallCoverV5Ad),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleCards {
    #[prost(oneof = "double_cards::Card", tags = "1, 2, 3")]
    pub card: ::core::option::Option<double_cards::Card>,
}
/// Nested message and enum types in `DoubleCards`.
pub mod double_cards {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Card {
        ///
        #[prost(message, tag = "1")]
        SmallCoverV2(super::SmallCoverV2),
        ///
        #[prost(message, tag = "2")]
        OnePicV2(super::OnePicV2),
        ///
        #[prost(message, tag = "3")]
        ThreePicV2(super::ThreePicV2),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_gif: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_blur: i32,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "6")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "8")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub cover_right_icon: i32,
    ///
    #[prost(string, tag = "10")]
    pub cover_right_background_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "15")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(int32, tag = "16")]
    pub official_icon: i32,
    ///
    #[prost(int32, tag = "17")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "18")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "19")]
    pub rcmd_reason_style_v2: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "20")]
    pub like_button: ::core::option::Option<LikeButton>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV3 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(message, optional, tag = "2")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub cover_right_button: ::core::option::Option<Button>,
    ///
    #[prost(string, tag = "5")]
    pub rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub official_icon: i32,
    ///
    #[prost(int32, tag = "8")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "9")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiddleCoverV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(int32, tag = "2")]
    pub ratio: i32,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub badge: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCoverV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(message, optional, tag = "2")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(string, tag = "3")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub cover_right_button: ::core::option::Option<Button>,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "9")]
    pub rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub official_icon: i32,
    ///
    #[prost(int32, tag = "11")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "12")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(int32, tag = "13")]
    pub show_top: i32,
    ///
    #[prost(int32, tag = "14")]
    pub show_bottom: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreeItemV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(int32, tag = "2")]
    pub title_icon: i32,
    ///
    #[prost(string, tag = "3")]
    pub more_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub more_text: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "5")]
    pub items: ::prost::alloc::vec::Vec<ThreeItemV2Item>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreeItemV2Item {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(int32, tag = "2")]
    pub cover_left_icon: i32,
    ///
    #[prost(string, tag = "3")]
    pub desc_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub desc_icon_1: i32,
    ///
    #[prost(string, tag = "5")]
    pub desc_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub desc_icon_2: i32,
    ///
    #[prost(string, tag = "7")]
    pub badge: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV4 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub title_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub title_right_pic: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwoItemV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<TwoItemV2Item>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TwoItemV2Item {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub cover_left_icon_1: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub more_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub more_text: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub items: ::prost::alloc::vec::Vec<DoubleCards>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePicV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub left_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub right_cover_1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub right_cover_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "9")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub cover_right_icon: i32,
    ///
    #[prost(string, tag = "11")]
    pub cover_right_background_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "15")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(message, optional, tag = "16")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnePicV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(int32, tag = "2")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub cover_right_icon: i32,
    ///
    #[prost(string, tag = "6")]
    pub cover_right_background_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub rcmd_reason: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(message, optional, tag = "10")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCoverV3 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_gif: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(message, optional, tag = "4")]
    pub top_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "5")]
    pub bottom_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(string, tag = "6")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "8")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "10")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "12")]
    pub official_icon: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePicV3 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub left_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub right_cover_1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub right_cover_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "9")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub cover_right_icon: i32,
    ///
    #[prost(string, tag = "11")]
    pub cover_right_background_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "13")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnePicV3 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub cover_right_icon: i32,
    ///
    #[prost(string, tag = "6")]
    pub cover_right_background_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV7 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverV9 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "6")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub cover_right_icon: i32,
    ///
    #[prost(int32, tag = "8")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "9")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "10")]
    pub up: ::core::option::Option<Up>,
    ///
    #[prost(message, optional, tag = "11")]
    pub left_cover_badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "12")]
    pub left_bottom_rcmd_reason_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallCoverConvergeV2 {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_text_1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_left_icon_1: i32,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text_2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub cover_left_icon_2: i32,
    ///
    #[prost(string, tag = "6")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cover_right_top_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "9")]
    pub rcmd_reason_style_v2: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallChannelSpecial {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<Base>,
    ///
    #[prost(string, tag = "2")]
    pub bg_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc_1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc_2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub rcmd_reason_style_2: ::core::option::Option<ReasonStyle>,
}
