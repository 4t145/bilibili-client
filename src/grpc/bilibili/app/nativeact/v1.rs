///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Area {
    ///
    #[prost(int64, tag = "1")]
    pub height: i64,
    ///
    #[prost(int64, tag = "2")]
    pub width: i64,
    ///
    #[prost(int64, tag = "3")]
    pub x: i64,
    ///
    #[prost(int64, tag = "4")]
    pub y: i64,
    ///
    #[prost(string, tag = "5")]
    pub ukey: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Badge {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_color_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarouselImgCard {
    ///
    #[prost(int64, tag = "1")]
    pub content_style: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub images: ::prost::alloc::vec::Vec<CarouselImgItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarouselImgItem {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub length: i64,
    ///
    #[prost(int64, tag = "4")]
    pub width: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub top_tab: ::core::option::Option<TopTab>,
    ///
    #[prost(int64, tag = "6")]
    pub height: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarouselWordCard {
    ///
    #[prost(int64, tag = "1")]
    pub content_style: i64,
    ///
    #[prost(int64, tag = "2")]
    pub scroll_type: i64,
    ///
    #[prost(message, repeated, tag = "3")]
    pub words: ::prost::alloc::vec::Vec<CarouselWordItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarouselWordItem {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    ///
    #[prost(string, tag = "1")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub top_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bottom_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub text_title_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub indicator_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub card_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub card_title_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub card_title_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub view_more_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub view_more_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub timeline_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub rcmd_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub subtitle_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub selected_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub selected_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub unselected_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "19")]
    pub unselected_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "20")]
    pub nt_selected_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "21")]
    pub nt_selected_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "22")]
    pub nt_unselected_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub nt_unselected_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "24")]
    pub progress_bar_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "25")]
    pub panel_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "26")]
    pub panel_select_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "27")]
    pub panel_select_font_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "28")]
    pub panel_nt_select_font_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicActMoreCard {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub subpage_data: ::core::option::Option<SubpageData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicCard {
    ///
    #[prost(message, optional, tag = "1")]
    pub dynamic: ::core::option::Option<super::super::dynamic::v2::DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicMoreCard {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub subpage_data: ::core::option::Option<SubpageData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicParams {
    ///
    #[prost(message, optional, tag = "1")]
    pub feed_offset: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    ///
    #[prost(int64, tag = "2")]
    pub offset: i64,
    ///
    #[prost(int64, tag = "3")]
    pub last_group: i64,
    ///
    #[prost(int64, tag = "4")]
    pub module_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub sort_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<DynamicParams>,
    ///
    #[prost(string, tag = "3")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "5")]
    pub local_time: i32,
    ///
    #[prost(bool, tag = "6")]
    pub is_cold_start: bool,
    ///
    #[prost(int64, tag = "7")]
    pub primary_page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorParams {
    ///
    #[prost(int64, tag = "1")]
    pub offset: i64,
    ///
    #[prost(int64, tag = "2")]
    pub module_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorRecommendCard {
    ///
    #[prost(string, tag = "1")]
    pub top_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub top_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bottom_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bottom_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_image_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub position1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub position2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub position3: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub position4: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub position5: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "13")]
    pub share: ::core::option::Option<Share>,
    ///
    #[prost(message, optional, tag = "14")]
    pub badge: ::core::option::Option<Badge>,
    ///
    #[prost(message, optional, tag = "15")]
    pub report_dic: ::core::option::Option<ReportDic>,
    ///
    #[prost(message, optional, tag = "16")]
    pub setting: ::core::option::Option<Setting>,
    ///
    #[prost(string, tag = "17")]
    pub middle_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub resource_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<EditorParams>,
    ///
    #[prost(int64, tag = "3")]
    pub primary_page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditorResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowOgvParams {
    ///
    #[prost(enumeration = "ActionType", tag = "1")]
    pub action: i32,
    ///
    #[prost(int32, tag = "2")]
    pub season_id: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowOgvReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<FollowOgvParams>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowOgvRly {
    ///
    #[prost(string, tag = "1")]
    pub follow_params: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameCard {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderCard {
    ///
    #[prost(string, tag = "1")]
    pub user_image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sponsor_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub high_light_image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub low_light_image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub view_num: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub discuss_num: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "9")]
    pub is_subscribed: bool,
    ///
    #[prost(int64, tag = "10")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IconCard {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<IconItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IconItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageTitleCard {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexReq {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub activity_from: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub dynamic_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub share_origin: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub tab_id: i64,
    ///
    #[prost(int64, tag = "6")]
    pub tab_module_id: i64,
    ///
    #[prost(int32, tag = "7")]
    pub https_url_req: i32,
    ///
    #[prost(string, tag = "8")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub current_tab: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "11")]
    pub local_time: i32,
    ///
    #[prost(bool, tag = "12")]
    pub is_cold_start: bool,
    ///
    #[prost(int64, tag = "13")]
    pub primary_page_id: i64,
    ///
    #[prost(string, tag = "14")]
    pub tab_from: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineIndexReq {
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub https_url_req: i32,
    ///
    #[prost(string, tag = "3")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "5")]
    pub local_time: i32,
    ///
    #[prost(bool, tag = "6")]
    pub is_cold_start: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayerDynamic {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub dynamic: ::core::option::Option<super::super::dynamic::v2::DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveCard {
    ///
    #[prost(int32, tag = "1")]
    pub has_live: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<LiveItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveItem {
    ///
    #[prost(int64, tag = "1")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub live_status: i64,
    ///
    #[prost(int64, tag = "4")]
    pub room_type: i64,
    ///
    #[prost(int64, tag = "5")]
    pub play_type: i64,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub online: i64,
    ///
    #[prost(int64, tag = "9")]
    pub area_id: i64,
    ///
    #[prost(string, tag = "10")]
    pub area_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub parent_area_id: i64,
    ///
    #[prost(string, tag = "12")]
    pub parent_area_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "13")]
    pub live_screen_type: i64,
    ///
    #[prost(int64, tag = "14")]
    pub last_end_time: i64,
    ///
    #[prost(string, tag = "15")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "16")]
    pub live_id: i64,
    ///
    #[prost(message, optional, tag = "17")]
    pub watched_show: ::core::option::Option<LiveWatchedShow>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveWatchedShow {
    ///
    #[prost(bool, tag = "1")]
    pub switch: bool,
    ///
    #[prost(int64, tag = "2")]
    pub num: i64,
    ///
    #[prost(string, tag = "3")]
    pub text_small: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text_large: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub icon_location: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageBox {
    ///
    #[prost(string, tag = "1")]
    pub alert_msg: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub confirm_button_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cancel_button_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub confirm_msg: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cancel_msg: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "MessageBoxType", tag = "6")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    ///
    #[prost(string, tag = "1")]
    pub module_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub module_id: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub module_color: ::core::option::Option<Color>,
    ///
    #[prost(message, optional, tag = "4")]
    pub module_setting: ::core::option::Option<Setting>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub module_items: ::prost::alloc::vec::Vec<ModuleItem>,
    ///
    #[prost(string, tag = "6")]
    pub subpage_params: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub module_ukey: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub has_more: bool,
    ///
    #[prost(bool, tag = "9")]
    pub is_feed: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleItem {
    ///
    #[prost(enumeration = "module_item::CardDetailCase", tag = "1")]
    pub card_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub card_id: ::prost::alloc::string::String,
    #[prost(
        oneof = "module_item::CardDetail",
        tags = "10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52"
    )]
    pub card_detail: ::core::option::Option<module_item::CardDetail>,
}
/// Nested message and enum types in `ModuleItem`.
pub mod module_item {
    ///
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CardDetailCase {
        ///
        CarddetailNotSet = 0,
        ///
        EditorRecommendCard = 10,
        ///
        ParticipationCard = 11,
        ///
        HeaderCard = 12,
        ///
        DynamicCard = 13,
        ///
        TextCard = 14,
        ///
        TextTitleCard = 15,
        ///
        ImageTitleCard = 16,
        ///
        DynamicMoreCard = 17,
        ///
        DynamicActMoreCard = 18,
        ///
        LiveCard = 19,
        ///
        CarouselImgCard = 20,
        ///
        CarouselWordCard = 21,
        ///
        ResourceCard = 22,
        ///
        ResourceMoreCard = 23,
        ///
        GameCard = 24,
        ///
        VideoCard = 25,
        ///
        VideoMoreCard = 26,
        ///
        RecommendCard = 27,
        ///
        RecommendVerticalCard = 28,
        ///
        RelativeactCard = 29,
        ///
        RelativeactCapsuleCard = 30,
        ///
        StatementCard = 31,
        ///
        IconCard = 32,
        ///
        VoteCard = 33,
        ///
        ReserveCard = 34,
        ///
        TimelineHeadCard = 35,
        ///
        TimelineEventTextCard = 36,
        ///
        TimelineEventImageCard = 37,
        ///
        TimelineEventImagetextCard = 38,
        ///
        TimelineEventResourceCard = 39,
        ///
        TimelineMoreCard = 40,
        ///
        TimelineUnfoldCard = 41,
        ///
        OgvOneCard = 42,
        ///
        OgvThreeCard = 43,
        ///
        OgvMoreCard = 44,
        ///
        NavigationCard = 45,
        ///
        ReplyCard = 46,
        ///
        TabCard = 47,
        ///
        NewactHeaderCard = 48,
        ///
        NewactAwardCard = 49,
        ///
        NewactStatementCard = 50,
        ///
        ProgressCard = 51,
        ///
        SelectCard = 52,
    }
    impl CardDetailCase {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CardDetailCase::CarddetailNotSet => "CARDDETAIL_NOT_SET",
                CardDetailCase::EditorRecommendCard => "EDITOR_RECOMMEND_CARD",
                CardDetailCase::ParticipationCard => "PARTICIPATION_CARD",
                CardDetailCase::HeaderCard => "HEADER_CARD",
                CardDetailCase::DynamicCard => "DYNAMIC_CARD",
                CardDetailCase::TextCard => "TEXT_CARD",
                CardDetailCase::TextTitleCard => "TEXT_TITLE_CARD",
                CardDetailCase::ImageTitleCard => "IMAGE_TITLE_CARD",
                CardDetailCase::DynamicMoreCard => "DYNAMIC_MORE_CARD",
                CardDetailCase::DynamicActMoreCard => "DYNAMIC_ACT_MORE_CARD",
                CardDetailCase::LiveCard => "LIVE_CARD",
                CardDetailCase::CarouselImgCard => "CAROUSEL_IMG_CARD",
                CardDetailCase::CarouselWordCard => "CAROUSEL_WORD_CARD",
                CardDetailCase::ResourceCard => "RESOURCE_CARD",
                CardDetailCase::ResourceMoreCard => "RESOURCE_MORE_CARD",
                CardDetailCase::GameCard => "GAME_CARD",
                CardDetailCase::VideoCard => "VIDEO_CARD",
                CardDetailCase::VideoMoreCard => "VIDEO_MORE_CARD",
                CardDetailCase::RecommendCard => "RECOMMEND_CARD",
                CardDetailCase::RecommendVerticalCard => "RECOMMEND_VERTICAL_CARD",
                CardDetailCase::RelativeactCard => "RELATIVEACT_CARD",
                CardDetailCase::RelativeactCapsuleCard => "RELATIVEACT_CAPSULE_CARD",
                CardDetailCase::StatementCard => "STATEMENT_CARD",
                CardDetailCase::IconCard => "ICON_CARD",
                CardDetailCase::VoteCard => "VOTE_CARD",
                CardDetailCase::ReserveCard => "RESERVE_CARD",
                CardDetailCase::TimelineHeadCard => "TIMELINE_HEAD_CARD",
                CardDetailCase::TimelineEventTextCard => "TIMELINE_EVENT_TEXT_CARD",
                CardDetailCase::TimelineEventImageCard => "TIMELINE_EVENT_IMAGE_CARD",
                CardDetailCase::TimelineEventImagetextCard => {
                    "TIMELINE_EVENT_IMAGETEXT_CARD"
                }
                CardDetailCase::TimelineEventResourceCard => {
                    "TIMELINE_EVENT_RESOURCE_CARD"
                }
                CardDetailCase::TimelineMoreCard => "TIMELINE_MORE_CARD",
                CardDetailCase::TimelineUnfoldCard => "TIMELINE_UNFOLD_CARD",
                CardDetailCase::OgvOneCard => "OGV_ONE_CARD",
                CardDetailCase::OgvThreeCard => "OGV_THREE_CARD",
                CardDetailCase::OgvMoreCard => "OGV_MORE_CARD",
                CardDetailCase::NavigationCard => "NAVIGATION_CARD",
                CardDetailCase::ReplyCard => "REPLY_CARD",
                CardDetailCase::TabCard => "TAB_CARD",
                CardDetailCase::NewactHeaderCard => "NEWACT_HEADER_CARD",
                CardDetailCase::NewactAwardCard => "NEWACT_AWARD_CARD",
                CardDetailCase::NewactStatementCard => "NEWACT_STATEMENT_CARD",
                CardDetailCase::ProgressCard => "PROGRESS_CARD",
                CardDetailCase::SelectCard => "SELECT_CARD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CARDDETAIL_NOT_SET" => Some(Self::CarddetailNotSet),
                "EDITOR_RECOMMEND_CARD" => Some(Self::EditorRecommendCard),
                "PARTICIPATION_CARD" => Some(Self::ParticipationCard),
                "HEADER_CARD" => Some(Self::HeaderCard),
                "DYNAMIC_CARD" => Some(Self::DynamicCard),
                "TEXT_CARD" => Some(Self::TextCard),
                "TEXT_TITLE_CARD" => Some(Self::TextTitleCard),
                "IMAGE_TITLE_CARD" => Some(Self::ImageTitleCard),
                "DYNAMIC_MORE_CARD" => Some(Self::DynamicMoreCard),
                "DYNAMIC_ACT_MORE_CARD" => Some(Self::DynamicActMoreCard),
                "LIVE_CARD" => Some(Self::LiveCard),
                "CAROUSEL_IMG_CARD" => Some(Self::CarouselImgCard),
                "CAROUSEL_WORD_CARD" => Some(Self::CarouselWordCard),
                "RESOURCE_CARD" => Some(Self::ResourceCard),
                "RESOURCE_MORE_CARD" => Some(Self::ResourceMoreCard),
                "GAME_CARD" => Some(Self::GameCard),
                "VIDEO_CARD" => Some(Self::VideoCard),
                "VIDEO_MORE_CARD" => Some(Self::VideoMoreCard),
                "RECOMMEND_CARD" => Some(Self::RecommendCard),
                "RECOMMEND_VERTICAL_CARD" => Some(Self::RecommendVerticalCard),
                "RELATIVEACT_CARD" => Some(Self::RelativeactCard),
                "RELATIVEACT_CAPSULE_CARD" => Some(Self::RelativeactCapsuleCard),
                "STATEMENT_CARD" => Some(Self::StatementCard),
                "ICON_CARD" => Some(Self::IconCard),
                "VOTE_CARD" => Some(Self::VoteCard),
                "RESERVE_CARD" => Some(Self::ReserveCard),
                "TIMELINE_HEAD_CARD" => Some(Self::TimelineHeadCard),
                "TIMELINE_EVENT_TEXT_CARD" => Some(Self::TimelineEventTextCard),
                "TIMELINE_EVENT_IMAGE_CARD" => Some(Self::TimelineEventImageCard),
                "TIMELINE_EVENT_IMAGETEXT_CARD" => Some(Self::TimelineEventImagetextCard),
                "TIMELINE_EVENT_RESOURCE_CARD" => Some(Self::TimelineEventResourceCard),
                "TIMELINE_MORE_CARD" => Some(Self::TimelineMoreCard),
                "TIMELINE_UNFOLD_CARD" => Some(Self::TimelineUnfoldCard),
                "OGV_ONE_CARD" => Some(Self::OgvOneCard),
                "OGV_THREE_CARD" => Some(Self::OgvThreeCard),
                "OGV_MORE_CARD" => Some(Self::OgvMoreCard),
                "NAVIGATION_CARD" => Some(Self::NavigationCard),
                "REPLY_CARD" => Some(Self::ReplyCard),
                "TAB_CARD" => Some(Self::TabCard),
                "NEWACT_HEADER_CARD" => Some(Self::NewactHeaderCard),
                "NEWACT_AWARD_CARD" => Some(Self::NewactAwardCard),
                "NEWACT_STATEMENT_CARD" => Some(Self::NewactStatementCard),
                "PROGRESS_CARD" => Some(Self::ProgressCard),
                "SELECT_CARD" => Some(Self::SelectCard),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CardDetail {
        ///
        #[prost(message, tag = "10")]
        EditorRecommendCard(super::EditorRecommendCard),
        ///
        #[prost(message, tag = "11")]
        ParticipationCard(super::ParticipationCard),
        ///
        #[prost(message, tag = "12")]
        HeaderCard(super::HeaderCard),
        ///
        #[prost(message, tag = "13")]
        DynamicCard(super::DynamicCard),
        ///
        #[prost(message, tag = "14")]
        TextCard(super::TextCard),
        ///
        #[prost(message, tag = "15")]
        TextTitleCard(super::TextTitleCard),
        ///
        #[prost(message, tag = "16")]
        ImageTitleCard(super::ImageTitleCard),
        ///
        #[prost(message, tag = "17")]
        DynamicMoreCard(super::DynamicMoreCard),
        ///
        #[prost(message, tag = "18")]
        DynamicActMoreCard(super::DynamicActMoreCard),
        ///
        #[prost(message, tag = "19")]
        LiveCard(super::LiveCard),
        ///
        #[prost(message, tag = "20")]
        CarouselImgCard(super::CarouselImgCard),
        ///
        #[prost(message, tag = "21")]
        CarouselWordCard(super::CarouselWordCard),
        ///
        #[prost(message, tag = "22")]
        ResourceCard(super::ResourceCard),
        ///
        #[prost(message, tag = "23")]
        ResourceMoreCard(super::ResourceMoreCard),
        ///
        #[prost(message, tag = "24")]
        GameCard(super::GameCard),
        ///
        #[prost(message, tag = "25")]
        VideoCard(super::VideoCard),
        ///
        #[prost(message, tag = "26")]
        VideoMoreCard(super::VideoMoreCard),
        ///
        #[prost(message, tag = "27")]
        RecommendCard(super::RcmdCard),
        ///
        #[prost(message, tag = "28")]
        RecommendVerticalCard(super::RcmdVerticalCard),
        ///
        #[prost(message, tag = "29")]
        RelativeactCard(super::RelativeactCard),
        ///
        #[prost(message, tag = "30")]
        RelativeactCapsuleCard(super::RelativeactCapsuleCard),
        ///
        #[prost(message, tag = "31")]
        StatementCard(super::StatementCard),
        ///
        #[prost(message, tag = "32")]
        IconCard(super::IconCard),
        ///
        #[prost(message, tag = "33")]
        VoteCard(super::VoteCard),
        ///
        #[prost(message, tag = "34")]
        ReserveCard(super::ReserveCard),
        ///
        #[prost(message, tag = "35")]
        TimelineHeadCard(super::TimelineHeadCard),
        ///
        #[prost(message, tag = "36")]
        TimelineEventTextCard(super::TimelineEventTextCard),
        ///
        #[prost(message, tag = "37")]
        TimelineEventImageCard(super::TimelineEventImageCard),
        ///
        #[prost(message, tag = "38")]
        TimelineEventImagetextCard(super::TimelineEventImagetextCard),
        ///
        #[prost(message, tag = "39")]
        TimelineEventResourceCard(super::TimelineEventResourceCard),
        ///
        #[prost(message, tag = "40")]
        TimelineMoreCard(super::TimelineMoreCard),
        ///
        #[prost(message, tag = "41")]
        TimelineUnfoldCard(super::TimelineUnfoldCard),
        ///
        #[prost(message, tag = "42")]
        OgvOneCard(super::OgvOneCard),
        ///
        #[prost(message, tag = "43")]
        OgvThreeCard(super::OgvThreeCard),
        ///
        #[prost(message, tag = "44")]
        OgvMoreCard(super::OgvMoreCard),
        ///
        #[prost(message, tag = "45")]
        NavigationCard(super::NavigationCard),
        ///
        #[prost(message, tag = "46")]
        ReplyCard(super::ReplyCard),
        ///
        #[prost(message, tag = "47")]
        TabCard(super::TabCard),
        ///
        #[prost(message, tag = "48")]
        NewactHeader(super::NewactHeader),
        ///
        #[prost(message, tag = "49")]
        NewactAward(super::NewactAward),
        ///
        #[prost(message, tag = "50")]
        NewactStatement(super::NewactStatement),
        ///
        #[prost(message, tag = "51")]
        ProgressCard(super::ProgressCard),
        ///
        #[prost(message, tag = "52")]
        SelectCard(super::SelectCard),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationCard {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<NavigationItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationItem {
    ///
    #[prost(int64, tag = "1")]
    pub module_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewactAward {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<NewactAwardItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewactAwardItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewactFeature {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub border_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewactHeader {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub sponsor_title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub mid: i64,
    ///
    #[prost(string, tag = "6")]
    pub user_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub user_face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub user_url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "9")]
    pub features: ::prost::alloc::vec::Vec<NewactFeature>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewactStatementItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewactStatement {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<NewactStatementItem>,
}
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
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvFollowButton {
    ///
    #[prost(bool, tag = "1")]
    pub is_followed: bool,
    ///
    #[prost(string, tag = "2")]
    pub follow_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub follow_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub unfollow_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub unfollow_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub follow_params: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvMoreCard {
    ///
    #[prost(string, tag = "1")]
    pub button_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub supernatant_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub params: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvOneCard {
    ///
    #[prost(string, tag = "1")]
    pub position1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub position2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub position3: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_right_text2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub rcmd_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub rcmd_icon: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub follow_button: ::core::option::Option<OgvFollowButton>,
    ///
    #[prost(string, tag = "13")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "14")]
    pub badge: ::core::option::Option<Badge>,
    ///
    #[prost(string, tag = "15")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "16")]
    pub report_dic: ::core::option::Option<ReportDic>,
    ///
    #[prost(string, tag = "17")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub resource_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvSupernatantParams {
    ///
    #[prost(int64, tag = "1")]
    pub last_index: i64,
    ///
    #[prost(int64, tag = "2")]
    pub offset: i64,
    ///
    #[prost(int64, tag = "3")]
    pub module_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvSupernatantReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<OgvSupernatantParams>,
    ///
    #[prost(int64, tag = "3")]
    pub primary_page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvSupernatantResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
    ///
    #[prost(int64, tag = "2")]
    pub last_index: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvThreeCard {
    ///
    #[prost(string, tag = "1")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub follow_button: ::core::option::Option<OgvFollowButton>,
    ///
    #[prost(string, tag = "4")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub badge: ::core::option::Option<Badge>,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub report_dic: ::core::option::Option<ReportDic>,
    ///
    #[prost(string, tag = "8")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub resource_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageResp {
    ///
    #[prost(bool, tag = "1")]
    pub is_online: bool,
    ///
    #[prost(bool, tag = "2")]
    pub ignore_app_dark_theme: bool,
    ///
    #[prost(message, optional, tag = "3")]
    pub page_color: ::core::option::Option<Color>,
    ///
    #[prost(message, optional, tag = "4")]
    pub page_share: ::core::option::Option<PageShare>,
    ///
    #[prost(message, optional, tag = "5")]
    pub page_header: ::core::option::Option<Module>,
    ///
    #[prost(message, optional, tag = "6")]
    pub participation: ::core::option::Option<Module>,
    ///
    #[prost(message, repeated, tag = "7")]
    pub module_list: ::prost::alloc::vec::Vec<Module>,
    ///
    #[prost(bool, tag = "8")]
    pub is_dynamic_feed: bool,
    ///
    #[prost(message, optional, tag = "9")]
    pub layer_dynamic: ::core::option::Option<LayerDynamic>,
    ///
    #[prost(bool, tag = "10")]
    pub is_editor_feed: bool,
    ///
    #[prost(int64, tag = "11")]
    pub sponsor_type: i64,
    ///
    #[prost(message, optional, tag = "12")]
    pub top_tab: ::core::option::Option<TopTab>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageShare {
    ///
    #[prost(int64, tag = "1")]
    pub r#type: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub inside_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub outside_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub origin: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub sid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub space_page_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub space_exclusive_page_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipationCard {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub selected_image: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<ParticipationCardItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipationCardItem {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDimension {
    ///
    #[prost(int64, tag = "1")]
    pub width: i64,
    ///
    #[prost(int64, tag = "2")]
    pub height: i64,
    ///
    #[prost(bool, tag = "3")]
    pub rotate: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressCard {
    ///
    #[prost(enumeration = "ProgressStyle", tag = "1")]
    pub style: i32,
    ///
    #[prost(enumeration = "ProgressSlot", tag = "2")]
    pub slot_type: i32,
    ///
    #[prost(enumeration = "ProgressBar", tag = "3")]
    pub bar_type: i32,
    ///
    #[prost(string, tag = "4")]
    pub texture_image: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub num: i64,
    ///
    #[prost(string, tag = "6")]
    pub display_num: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "7")]
    pub nodes: ::prost::alloc::vec::Vec<ProgressNode>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressEvent {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ProgressEventItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressEventItem {
    ///
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub num: i64,
    ///
    #[prost(string, tag = "4")]
    pub display_num: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub web_key: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub dimension: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressNode {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub num: i64,
    ///
    #[prost(string, tag = "3")]
    pub display_num: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressReq {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressRly {
    ///
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<ProgressEvent>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdCard {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub reason: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "6")]
    pub is_followed: bool,
    ///
    #[prost(message, optional, tag = "7")]
    pub official: ::core::option::Option<OfficialInfo>,
    ///
    #[prost(message, optional, tag = "8")]
    pub vip: ::core::option::Option<VipInfo>,
    ///
    #[prost(string, tag = "9")]
    pub rank_icon: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "RedirectType", tag = "10")]
    pub redirect_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdVerticalCard {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<RcmdCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelativeactCapsuleCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<RelativeactCapsuleItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelativeactCapsuleItem {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelativeactCard {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyCard {
    ///
    #[prost(int64, tag = "1")]
    pub reply_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub r#type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportDic {
    ///
    #[prost(string, tag = "1")]
    pub biz_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub season_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "5")]
    pub sub_type: i32,
    ///
    #[prost(int64, tag = "6")]
    pub ep_id: i64,
    ///
    #[prost(int32, tag = "7")]
    pub is_preview: i32,
    ///
    #[prost(int64, tag = "8")]
    pub season_id: i64,
    ///
    #[prost(string, tag = "9")]
    pub author_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveButton {
    ///
    #[prost(enumeration = "ReserveGoto", tag = "1")]
    pub goto: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub message_box: ::core::option::Option<MessageBox>,
    ///
    #[prost(string, tag = "3")]
    pub reserve_params: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub has_done: bool,
    ///
    #[prost(string, tag = "5")]
    pub done_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub undone_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub is_highlight: bool,
    ///
    #[prost(string, tag = "9")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveCard {
    ///
    #[prost(int64, tag = "1")]
    pub sid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub num: i64,
    ///
    #[prost(string, tag = "9")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub button: ::core::option::Option<ReserveButton>,
    ///
    #[prost(bool, tag = "11")]
    pub hide_reserve_num: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveParams {
    ///
    #[prost(int32, tag = "1")]
    pub action: i32,
    ///
    #[prost(int64, tag = "2")]
    pub sid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<ReserveParams>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveRly {
    ///
    #[prost(string, tag = "1")]
    pub reserve_params: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover_image_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub cover_left_icon1: i64,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub cover_left_icon2: i64,
    ///
    #[prost(message, optional, tag = "9")]
    pub badge: ::core::option::Option<Badge>,
    ///
    #[prost(message, optional, tag = "10")]
    pub report_dic: ::core::option::Option<ReportDic>,
    ///
    #[prost(string, tag = "11")]
    pub resource_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMoreCard {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub subpage_data: ::core::option::Option<SubpageData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceParams {
    ///
    #[prost(int64, tag = "1")]
    pub offset: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_offset: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub module_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub sort_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<ResourceParams>,
    ///
    #[prost(int64, tag = "3")]
    pub primary_page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectCard {
    ///
    #[prost(int64, tag = "1")]
    pub current_tab: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<SelectItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectItem {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub page_share: ::core::option::Option<PageShare>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Setting {
    ///
    #[prost(bool, tag = "1")]
    pub display_more_button: bool,
    ///
    #[prost(bool, tag = "2")]
    pub display_title: bool,
    ///
    #[prost(bool, tag = "3")]
    pub auto_carousel: bool,
    ///
    #[prost(bool, tag = "4")]
    pub top_tab_follow_img: bool,
    ///
    #[prost(bool, tag = "5")]
    pub top_tab_fade_away: bool,
    ///
    #[prost(bool, tag = "6")]
    pub auto_play: bool,
    ///
    #[prost(bool, tag = "7")]
    pub display_unfold_button: bool,
    ///
    #[prost(bool, tag = "8")]
    pub display_num: bool,
    ///
    #[prost(bool, tag = "9")]
    pub display_view_num: bool,
    ///
    #[prost(bool, tag = "10")]
    pub display_subscribe_btn: bool,
    ///
    #[prost(bool, tag = "11")]
    pub unfold_rest: bool,
    ///
    #[prost(bool, tag = "12")]
    pub display_progress_num: bool,
    ///
    #[prost(bool, tag = "13")]
    pub display_node_num: bool,
    ///
    #[prost(bool, tag = "14")]
    pub display_node_desc: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    ///
    #[prost(bool, tag = "1")]
    pub display_later: bool,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(string, tag = "3")]
    pub share_origin: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub share_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeImage {
    ///
    #[prost(string, tag = "1")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub height: i64,
    ///
    #[prost(int64, tag = "3")]
    pub width: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatementCard {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpageData {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub params: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub tabs: ::prost::alloc::vec::Vec<SubpageTab>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubpageTab {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub params: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabCard {
    ///
    #[prost(int64, tag = "1")]
    pub current_tab: i64,
    ///
    #[prost(enumeration = "TabStyle", tag = "2")]
    pub style: i32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<TabItem>,
    ///
    #[prost(message, optional, tag = "4")]
    pub bg_image: ::core::option::Option<SizeImage>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabIndexReq {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub https_url_req: i32,
    ///
    #[prost(string, tag = "3")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "5")]
    pub primary_page_id: i64,
    ///
    #[prost(int32, tag = "6")]
    pub local_time: i32,
    ///
    #[prost(bool, tag = "7")]
    pub is_cold_start: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabItem {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub disable_click: bool,
    ///
    #[prost(string, tag = "4")]
    pub disable_click_toast: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub selected_image: ::core::option::Option<SizeImage>,
    ///
    #[prost(message, optional, tag = "6")]
    pub unselected_image: ::core::option::Option<SizeImage>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextCard {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextTitleCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineEventImageCard {
    ///
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<SizeImage>,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineEventImagetextCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineEventResourceCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover_image_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub position1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub position2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub badge: ::core::option::Option<Badge>,
    ///
    #[prost(message, optional, tag = "7")]
    pub report_dic: ::core::option::Option<ReportDic>,
    ///
    #[prost(string, tag = "8")]
    pub resource_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineEventTextCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineHeadCard {
    ///
    #[prost(string, tag = "1")]
    pub stage: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineMoreCard {
    ///
    #[prost(string, tag = "1")]
    pub button_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub supernatant_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub params: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineSupernatantParams {
    ///
    #[prost(int64, tag = "1")]
    pub last_index: i64,
    ///
    #[prost(int64, tag = "2")]
    pub offset: i64,
    ///
    #[prost(int64, tag = "3")]
    pub module_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineSupernatantReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<TimelineSupernatantParams>,
    ///
    #[prost(int64, tag = "3")]
    pub primary_page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineSupernatantResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
    ///
    #[prost(int64, tag = "2")]
    pub last_index: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimelineUnfoldCard {
    ///
    #[prost(string, tag = "1")]
    pub unfold_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub fold_text: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub cards: ::prost::alloc::vec::Vec<timeline_unfold_card::Card>,
}
/// Nested message and enum types in `TimelineUnfoldCard`.
pub mod timeline_unfold_card {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Card {
        #[prost(oneof = "card::CardDetail", tags = "1, 2, 3, 4, 5")]
        pub card_detail: ::core::option::Option<card::CardDetail>,
    }
    /// Nested message and enum types in `Card`.
    pub mod card {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CardDetail {
            ///
            #[prost(message, tag = "1")]
            TimelineHeadCard(super::super::TimelineHeadCard),
            ///
            #[prost(message, tag = "2")]
            TimelineEventTextCard(super::super::TimelineEventTextCard),
            ///
            #[prost(message, tag = "3")]
            TimelineEventImageCard(super::super::TimelineEventImageCard),
            ///
            #[prost(message, tag = "4")]
            TimelineEventImagetextCard(super::super::TimelineEventImagetextCard),
            ///
            #[prost(message, tag = "5")]
            TimelineEventResourceCard(super::super::TimelineEventResourceCard),
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicIndexReq {
    ///
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub activity_from: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub dynamic_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub share_origin: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub tab_id: i64,
    ///
    #[prost(int64, tag = "6")]
    pub tab_module_id: i64,
    ///
    #[prost(int32, tag = "7")]
    pub https_url_req: i32,
    ///
    #[prost(string, tag = "8")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub current_tab: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "11")]
    pub local_time: i32,
    ///
    #[prost(bool, tag = "12")]
    pub is_cold_start: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopTab {
    ///
    #[prost(string, tag = "1")]
    pub bg_image1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bg_image2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub tab_top_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub tab_middle_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub tab_bottom_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub font_color: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub bar_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover_image_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text3: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub badge: ::core::option::Option<Badge>,
    ///
    #[prost(message, optional, tag = "8")]
    pub rights: ::core::option::Option<VideoRights>,
    ///
    #[prost(message, optional, tag = "9")]
    pub dimension: ::core::option::Option<PlayerDimension>,
    ///
    #[prost(message, optional, tag = "10")]
    pub report_dic: ::core::option::Option<ReportDic>,
    ///
    #[prost(string, tag = "11")]
    pub resource_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMoreCard {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub subpage_data: ::core::option::Option<SubpageData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoParams {
    ///
    #[prost(int64, tag = "1")]
    pub offset: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_offset: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub module_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub sort_type: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<VideoParams>,
    ///
    #[prost(int64, tag = "3")]
    pub primary_page_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoRights {
    ///
    #[prost(bool, tag = "1")]
    pub ugc_pay: bool,
    ///
    #[prost(bool, tag = "2")]
    pub is_cooperation: bool,
    ///
    #[prost(bool, tag = "3")]
    pub is_pgc: bool,
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
    #[prost(int64, tag = "3")]
    pub due_date: i64,
    ///
    #[prost(int32, tag = "4")]
    pub vip_pay_type: i32,
    ///
    #[prost(int32, tag = "5")]
    pub theme_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub label: ::core::option::Option<VipLabel>,
    ///
    #[prost(int32, tag = "7")]
    pub avatar_subscript: i32,
    ///
    #[prost(string, tag = "8")]
    pub nickname_color: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub role: i64,
    ///
    #[prost(string, tag = "10")]
    pub avatar_subscript_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    ///
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub label_theme: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub bg_style: i32,
    ///
    #[prost(string, tag = "7")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub border_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteButton {
    ///
    #[prost(message, optional, tag = "1")]
    pub area: ::core::option::Option<Area>,
    ///
    #[prost(string, tag = "2")]
    pub done_image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub undone_image: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub has_voted: bool,
    ///
    #[prost(message, optional, tag = "5")]
    pub message_box: ::core::option::Option<MessageBox>,
    ///
    #[prost(string, tag = "6")]
    pub vote_params: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub source_item_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteCard {
    ///
    #[prost(message, optional, tag = "1")]
    pub bg_image: ::core::option::Option<SizeImage>,
    ///
    #[prost(int64, tag = "2")]
    pub option_num: i64,
    ///
    #[prost(message, repeated, tag = "3")]
    pub buttons: ::prost::alloc::vec::Vec<VoteButton>,
    ///
    #[prost(message, optional, tag = "4")]
    pub left_num: ::core::option::Option<VoteNum>,
    ///
    #[prost(message, optional, tag = "5")]
    pub progress: ::core::option::Option<VoteProgress>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteNum {
    ///
    #[prost(message, optional, tag = "1")]
    pub area: ::core::option::Option<Area>,
    ///
    #[prost(int64, tag = "2")]
    pub num: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteParams {
    ///
    #[prost(int32, tag = "1")]
    pub action: i32,
    ///
    #[prost(int64, tag = "2")]
    pub sid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub gid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub source_item_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteProgress {
    ///
    #[prost(message, optional, tag = "1")]
    pub area: ::core::option::Option<Area>,
    ///
    #[prost(enumeration = "VoteProgressStyle", tag = "2")]
    pub style: i32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<vote_progress::VoteProgressItem>,
}
/// Nested message and enum types in `VoteProgress`.
pub mod vote_progress {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VoteProgressItem {
        ///
        #[prost(string, tag = "1")]
        pub color: ::prost::alloc::string::String,
        ///
        #[prost(int64, tag = "2")]
        pub num: i64,
        ///
        #[prost(int64, tag = "3")]
        pub source_item_id: i64,
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteReq {
    ///
    #[prost(string, tag = "1")]
    pub raw_params: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<VoteParams>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteResp {
    ///
    #[prost(string, tag = "1")]
    pub vote_params: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub left_num: i64,
    ///
    #[prost(int64, tag = "3")]
    pub can_vote_num: i64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    ///
    Default = 0,
    ///
    Do = 1,
    ///
    Undo = 2,
}
impl ActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionType::Default => "Default",
            ActionType::Do => "Do",
            ActionType::Undo => "Undo",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Default" => Some(Self::Default),
            "Do" => Some(Self::Do),
            "Undo" => Some(Self::Undo),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageBoxType {
    ///
    Dialog = 0,
    ///
    Toast = 1,
}
impl MessageBoxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageBoxType::Dialog => "Dialog",
            MessageBoxType::Toast => "Toast",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Dialog" => Some(Self::Dialog),
            "Toast" => Some(Self::Toast),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProgressBar {
    ///
    PBarDefault = 0,
    ///
    PBarColor = 1,
    ///
    PBarTexture = 2,
}
impl ProgressBar {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProgressBar::PBarDefault => "PBarDefault",
            ProgressBar::PBarColor => "PBarColor",
            ProgressBar::PBarTexture => "PBarTexture",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PBarDefault" => Some(Self::PBarDefault),
            "PBarColor" => Some(Self::PBarColor),
            "PBarTexture" => Some(Self::PBarTexture),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProgressSlot {
    ///
    PSlotDefault = 0,
    ///
    PSlotOutline = 1,
    ///
    PSlotFill = 2,
}
impl ProgressSlot {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProgressSlot::PSlotDefault => "PSlotDefault",
            ProgressSlot::PSlotOutline => "PSlotOutline",
            ProgressSlot::PSlotFill => "PSlotFill",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PSlotDefault" => Some(Self::PSlotDefault),
            "PSlotOutline" => Some(Self::PSlotOutline),
            "PSlotFill" => Some(Self::PSlotFill),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProgressStyle {
    ///
    PStyleDefault = 0,
    ///
    PStyleRound = 1,
    ///
    PStyleRectangle = 2,
    ///
    PStyleNode = 3,
}
impl ProgressStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProgressStyle::PStyleDefault => "PStyleDefault",
            ProgressStyle::PStyleRound => "PStyleRound",
            ProgressStyle::PStyleRectangle => "PStyleRectangle",
            ProgressStyle::PStyleNode => "PStyleNode",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PStyleDefault" => Some(Self::PStyleDefault),
            "PStyleRound" => Some(Self::PStyleRound),
            "PStyleRectangle" => Some(Self::PStyleRectangle),
            "PStyleNode" => Some(Self::PStyleNode),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RedirectType {
    ///
    RtTypeDefault = 0,
    ///
    RtTypeSpace = 1,
    ///
    RtTypeUri = 2,
}
impl RedirectType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RedirectType::RtTypeDefault => "RtTypeDefault",
            RedirectType::RtTypeSpace => "RtTypeSpace",
            RedirectType::RtTypeUri => "RtTypeUri",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RtTypeDefault" => Some(Self::RtTypeDefault),
            "RtTypeSpace" => Some(Self::RtTypeSpace),
            "RtTypeUri" => Some(Self::RtTypeUri),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveGoto {
    ///
    Reserve = 0,
    ///
    Redirect = 1,
    ///
    Unable = 2,
}
impl ReserveGoto {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReserveGoto::Reserve => "Reserve",
            ReserveGoto::Redirect => "Redirect",
            ReserveGoto::Unable => "Unable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Reserve" => Some(Self::Reserve),
            "Redirect" => Some(Self::Redirect),
            "Unable" => Some(Self::Unable),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TabStyle {
    ///
    Default = 0,
    ///
    Color = 1,
    ///
    Image = 2,
}
impl TabStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TabStyle::Default => "TabStyleDefault",
            TabStyle::Color => "TabStyleColor",
            TabStyle::Image => "TabStyleImage",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TabStyleDefault" => Some(Self::Default),
            "TabStyleColor" => Some(Self::Color),
            "TabStyleImage" => Some(Self::Image),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoteProgressStyle {
    ///
    VpStyleDefault = 0,
    ///
    VpStyleCircle = 1,
    ///
    VpStyleSquare = 2,
}
impl VoteProgressStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteProgressStyle::VpStyleDefault => "VPStyleDefault",
            VoteProgressStyle::VpStyleCircle => "VPStyleCircle",
            VoteProgressStyle::VpStyleSquare => "VPStyleSquare",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VPStyleDefault" => Some(Self::VpStyleDefault),
            "VPStyleCircle" => Some(Self::VpStyleCircle),
            "VPStyleSquare" => Some(Self::VpStyleSquare),
            _ => None,
        }
    }
}
