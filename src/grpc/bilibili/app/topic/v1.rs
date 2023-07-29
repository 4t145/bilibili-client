///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonMeta {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailsTopInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic_info: ::core::option::Option<TopicInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<User>,
    ///
    #[prost(string, tag = "3")]
    pub stats_desc: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub has_create_jurisdiction: bool,
    ///
    #[prost(message, optional, tag = "5")]
    pub operation_content: ::core::option::Option<OperationContent>,
    ///
    #[prost(string, tag = "6")]
    pub head_img_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub head_img_backcolor: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub word_color: i32,
    ///
    #[prost(int32, tag = "9")]
    pub mission_page_show_type: i32,
    ///
    #[prost(string, tag = "10")]
    pub mission_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub mission_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "12")]
    pub topic_set: ::core::option::Option<TopicSet>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FoldCardItem {
    ///
    #[prost(int32, tag = "1")]
    pub is_show_fold: i32,
    ///
    #[prost(int64, tag = "2")]
    pub fold_count: i64,
    ///
    #[prost(string, tag = "3")]
    pub card_show_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub fold_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionalCard {
    ///
    #[prost(message, repeated, tag = "1")]
    pub capsules: ::prost::alloc::vec::Vec<TopicCapsule>,
    ///
    #[prost(message, optional, tag = "2")]
    pub traffic_card: ::core::option::Option<TrafficCard>,
    ///
    #[prost(message, optional, tag = "3")]
    pub game_card: ::core::option::Option<GameCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameCard {
    ///
    #[prost(int64, tag = "1")]
    pub game_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub game_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub game_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub score: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub game_tags: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub notice: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub game_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineProgressBar {
    ///
    #[prost(string, tag = "1")]
    pub icon_drag: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_drag_hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon_stop: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon_stop_hash: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargeCoverInline {
    ///
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<super::super::card::v1::Base>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_left_icon1: i32,
    ///
    #[prost(string, tag = "4")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub cover_left_icon2: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub right_top_live_badge: ::core::option::Option<RightTopLiveBadge>,
    ///
    #[prost(string, tag = "7")]
    pub extra_uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub inline_progress_bar: ::core::option::Option<InlineProgressBar>,
    ///
    #[prost(message, optional, tag = "9")]
    pub topic_three_point: ::core::option::Option<TopicThreePoint>,
    ///
    #[prost(string, tag = "10")]
    pub cover_left_desc: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "11")]
    pub hide_danmu_switch: bool,
    ///
    #[prost(bool, tag = "12")]
    pub disable_danmu: bool,
    ///
    #[prost(int32, tag = "13")]
    pub can_play: i32,
    ///
    #[prost(string, tag = "14")]
    pub duration_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "15")]
    pub relation_data: ::core::option::Option<RelationData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveBadgeResource {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub animation_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub animation_url_hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub background_color_light: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub background_color_night: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub alpha_light: i64,
    ///
    #[prost(int64, tag = "7")]
    pub alpha_night: i64,
    ///
    #[prost(string, tag = "8")]
    pub font_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCard {
    #[prost(oneof = "operation_card::Card", tags = "1")]
    pub card: ::core::option::Option<operation_card::Card>,
}
/// Nested message and enum types in `OperationCard`.
pub mod operation_card {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Card {
        ///
        #[prost(message, tag = "1")]
        LargeCoverInline(super::LargeCoverInline),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationContent {
    ///
    #[prost(message, optional, tag = "1")]
    pub operation_card: ::core::option::Option<OperationCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubLayer {
    ///
    #[prost(int32, tag = "1")]
    pub show_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub jump_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub button_meta: ::core::option::Option<ButtonMeta>,
    ///
    #[prost(bool, tag = "4")]
    pub close_pub_layer_entry: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationData {
    ///
    #[prost(bool, tag = "1")]
    pub is_fav: bool,
    ///
    #[prost(bool, tag = "2")]
    pub is_coin: bool,
    ///
    #[prost(bool, tag = "3")]
    pub is_follow: bool,
    ///
    #[prost(bool, tag = "4")]
    pub is_like: bool,
    ///
    #[prost(int64, tag = "5")]
    pub like_count: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RightTopLiveBadge {
    ///
    #[prost(int64, tag = "1")]
    pub live_status: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub in_live: ::core::option::Option<LiveBadgeResource>,
    ///
    #[prost(string, tag = "3")]
    pub live_stats_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortContent {
    ///
    #[prost(int64, tag = "1")]
    pub sort_by: i64,
    ///
    #[prost(string, tag = "2")]
    pub sort_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeLineEvents {
    ///
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub time_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub jump_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeLineResource {
    ///
    #[prost(int64, tag = "1")]
    pub time_line_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub time_line_title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub time_line_events: ::prost::alloc::vec::Vec<TimeLineEvents>,
    ///
    #[prost(bool, tag = "4")]
    pub has_more: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicActivities {
    ///
    #[prost(message, repeated, tag = "1")]
    pub activity: ::prost::alloc::vec::Vec<TopicActivity>,
    ///
    #[prost(string, tag = "2")]
    pub act_list_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicActivity {
    ///
    #[prost(int64, tag = "1")]
    pub activity_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub activity_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicCapsule {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicCardItem {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub dynamic_item: ::core::option::Option<super::super::dynamic::v2::DynamicItem>,
    ///
    #[prost(message, optional, tag = "3")]
    pub ford_card_item: ::core::option::Option<FoldCardItem>,
    ///
    #[prost(message, optional, tag = "4")]
    pub video_small_card_item: ::core::option::Option<VideoSmallCardItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicCardList {
    ///
    #[prost(message, repeated, tag = "1")]
    pub topic_card_items: ::prost::alloc::vec::Vec<TopicCardItem>,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    ///
    #[prost(message, optional, tag = "4")]
    pub topic_sort_by_conf: ::core::option::Option<TopicSortByConf>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicDetailsAllReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub details_top_info: ::core::option::Option<DetailsTopInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub topic_activities: ::core::option::Option<TopicActivities>,
    ///
    #[prost(message, optional, tag = "3")]
    pub topic_card_list: ::core::option::Option<TopicCardList>,
    ///
    #[prost(message, optional, tag = "4")]
    pub functional_card: ::core::option::Option<FunctionalCard>,
    ///
    #[prost(message, optional, tag = "5")]
    pub pub_layer: ::core::option::Option<PubLayer>,
    ///
    #[prost(message, optional, tag = "6")]
    pub time_line_resource: ::core::option::Option<TimeLineResource>,
    ///
    #[prost(message, optional, tag = "7")]
    pub topic_server_config: ::core::option::Option<TopicServerConfig>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicDetailsAllReq {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub sort_by: i64,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    ///
    #[prost(int32, tag = "5")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int32, tag = "7")]
    pub need_refresh: i32,
    ///
    #[prost(string, tag = "8")]
    pub source: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub topic_details_ext_mode: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicDetailsFoldReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic_card_list: ::core::option::Option<TopicCardList>,
    ///
    #[prost(int64, tag = "2")]
    pub fold_count: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicDetailsFoldReq {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    ///
    #[prost(int32, tag = "4")]
    pub local_time: i32,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "6")]
    pub from_sort_by: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicInfo {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub view: i64,
    ///
    #[prost(int64, tag = "5")]
    pub discuss: i64,
    ///
    #[prost(int64, tag = "6")]
    pub fav: i64,
    ///
    #[prost(int64, tag = "7")]
    pub dynamics: i64,
    ///
    #[prost(int32, tag = "8")]
    pub state: i32,
    ///
    #[prost(string, tag = "9")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub backcolor: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "11")]
    pub is_fav: bool,
    ///
    #[prost(string, tag = "12")]
    pub description: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "13")]
    pub create_source: i32,
    ///
    #[prost(string, tag = "14")]
    pub share_pic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "15")]
    pub share: i64,
    ///
    #[prost(int64, tag = "16")]
    pub like: i64,
    ///
    #[prost(string, tag = "17")]
    pub share_url: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "18")]
    pub is_like: bool,
    ///
    #[prost(int32, tag = "19")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "20")]
    pub stats_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "21")]
    pub fixed_topic_icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicServerConfig {
    ///
    #[prost(int64, tag = "1")]
    pub pub_events_increase_threshold: i64,
    ///
    #[prost(int64, tag = "2")]
    pub pub_events_hidden_timeout_threshold: i64,
    ///
    #[prost(int64, tag = "3")]
    pub vert_online_refresh_time: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSet {
    ///
    #[prost(int64, tag = "1")]
    pub set_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub set_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSetDetailsReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic_set_head_info: ::core::option::Option<TopicSetHeadInfo>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub topic_info: ::prost::alloc::vec::Vec<TopicInfo>,
    ///
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "4")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub sort_cfg: ::core::option::Option<TopicSetSortCfg>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSetDetailsReq {
    ///
    #[prost(int64, tag = "1")]
    pub set_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub sort_by: i64,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub page_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSetHeadInfo {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic_set: ::core::option::Option<TopicSet>,
    ///
    #[prost(string, tag = "2")]
    pub topic_cnt_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub head_img_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub mission_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub mission_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub icon_url: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "7")]
    pub is_fav: bool,
    ///
    #[prost(bool, tag = "8")]
    pub is_first_time: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSetSortCfg {
    ///
    #[prost(int64, tag = "1")]
    pub default_sort_by: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub all_sort_by: ::prost::alloc::vec::Vec<SortContent>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicSortByConf {
    ///
    #[prost(int64, tag = "1")]
    pub default_sort_by: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub all_sort_by: ::prost::alloc::vec::Vec<SortContent>,
    ///
    #[prost(int64, tag = "3")]
    pub show_sort_by: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicThreePoint {
    ///
    #[prost(message, repeated, tag = "1")]
    pub dyn_three_point_items: ::prost::alloc::vec::Vec<ThreePointItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficCard {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub base_pic: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub benefit_point: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub card_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub jump_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(string, tag = "2")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub name_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoCardBase {
    ///
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub up_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub play: i64,
    ///
    #[prost(string, tag = "5")]
    pub jump_link: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub aid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSmallCardItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub video_card_base: ::core::option::Option<VideoCardBase>,
    ///
    #[prost(string, tag = "2")]
    pub cover_left_badge_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub card_stat_icon1: i64,
    ///
    #[prost(string, tag = "4")]
    pub card_stat_text1: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub card_stat_icon2: i64,
    ///
    #[prost(string, tag = "6")]
    pub card_stat_text2: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopicCardType {
    ///
    IllegalType = 0,
    ///
    Dynamic = 1,
    ///
    Fold = 2,
    ///
    VideoSmallCard = 3,
}
impl TopicCardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TopicCardType::IllegalType => "ILLEGAL_TYPE",
            TopicCardType::Dynamic => "DYNAMIC",
            TopicCardType::Fold => "FOLD",
            TopicCardType::VideoSmallCard => "VIDEO_SMALL_CARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ILLEGAL_TYPE" => Some(Self::IllegalType),
            "DYNAMIC" => Some(Self::Dynamic),
            "FOLD" => Some(Self::Fold),
            "VIDEO_SMALL_CARD" => Some(Self::VideoSmallCard),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopicDetailsExtMode {
    ///
    ModeIllegalType = 0,
    ///
    Story = 1,
}
impl TopicDetailsExtMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TopicDetailsExtMode::ModeIllegalType => "MODE_ILLEGAL_TYPE",
            TopicDetailsExtMode::Story => "STORY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MODE_ILLEGAL_TYPE" => Some(Self::ModeIllegalType),
            "STORY" => Some(Self::Story),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod topic_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct TopicClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TopicClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TopicClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TopicClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TopicClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn topic_details_all(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicDetailsAllReq>,
        ) -> std::result::Result<
            tonic::Response<super::TopicDetailsAllReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.topic.v1.Topic/TopicDetailsAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.topic.v1.Topic", "TopicDetailsAll"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn topic_details_fold(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicDetailsFoldReq>,
        ) -> std::result::Result<
            tonic::Response<super::TopicDetailsFoldReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.topic.v1.Topic/TopicDetailsFold",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.topic.v1.Topic", "TopicDetailsFold"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn topic_set_details(
            &mut self,
            request: impl tonic::IntoRequest<super::TopicSetDetailsReq>,
        ) -> std::result::Result<
            tonic::Response<super::TopicSetDetailsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.topic.v1.Topic/TopicSetDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.topic.v1.Topic", "TopicSetDetails"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
