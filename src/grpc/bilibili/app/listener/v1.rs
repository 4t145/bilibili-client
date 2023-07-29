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
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub relation: ::core::option::Option<FollowRelation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkArcDetailsReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<PlayItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkArcDetailsResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DetailItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkArchive {
    ///
    #[prost(int64, tag = "1")]
    pub oid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub duration: i64,
    ///
    #[prost(int32, tag = "6")]
    pub rid: i32,
    ///
    #[prost(string, tag = "7")]
    pub rname: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub publish: i64,
    ///
    #[prost(string, tag = "9")]
    pub displayed_oid: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub copyright: i32,
    ///
    #[prost(message, optional, tag = "11")]
    pub rights: ::core::option::Option<BkArcRights>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkArcPart {
    ///
    #[prost(int64, tag = "1")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub sub_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub duration: i64,
    ///
    #[prost(int32, tag = "5")]
    pub page: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkArcRights {
    ///
    #[prost(int32, tag = "1")]
    pub no_reprint: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BkStat {
    ///
    #[prost(int32, tag = "1")]
    pub like: i32,
    ///
    #[prost(int32, tag = "2")]
    pub coin: i32,
    ///
    #[prost(int32, tag = "3")]
    pub favourite: i32,
    ///
    #[prost(int32, tag = "4")]
    pub reply: i32,
    ///
    #[prost(int32, tag = "5")]
    pub share: i32,
    ///
    #[prost(int32, tag = "6")]
    pub view: i32,
    ///
    #[prost(bool, tag = "7")]
    pub has_like: bool,
    ///
    #[prost(bool, tag = "8")]
    pub has_coin: bool,
    ///
    #[prost(bool, tag = "9")]
    pub has_fav: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardModule {
    ///
    #[prost(int32, tag = "1")]
    pub module_type: i32,
    #[prost(oneof = "card_module::Module", tags = "2, 3, 4")]
    pub module: ::core::option::Option<card_module::Module>,
}
/// Nested message and enum types in `CardModule`.
pub mod card_module {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Module {
        ///
        #[prost(message, tag = "2")]
        ModuleHeader(super::PkcmHeader),
        ///
        #[prost(message, tag = "3")]
        ModuleArchive(super::PkcmArchive),
        ///
        #[prost(message, tag = "4")]
        ModuleCbtn(super::PkcmCenterButton),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickReq {
    ///
    #[prost(int64, tag = "1")]
    pub sid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub action: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickResp {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinAddReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(int32, tag = "2")]
    pub num: i32,
    ///
    #[prost(bool, tag = "3")]
    pub thumb_up: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinAddResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashItem {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(string, tag = "2")]
    pub base_url: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "3")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "4")]
    pub bandwidth: i32,
    ///
    #[prost(string, tag = "5")]
    pub mime_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub codecs: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "12")]
    pub segment_base: ::core::option::Option<DashSegmentBase>,
    ///
    #[prost(int32, tag = "13")]
    pub codecid: i32,
    ///
    #[prost(string, tag = "14")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "15")]
    pub size: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashSegmentBase {
    ///
    #[prost(string, tag = "1")]
    pub initialization: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub index_range: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub arc: ::core::option::Option<BkArchive>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub parts: ::prost::alloc::vec::Vec<BkArcPart>,
    ///
    #[prost(message, optional, tag = "4")]
    pub owner: ::core::option::Option<Author>,
    ///
    #[prost(message, optional, tag = "5")]
    pub stat: ::core::option::Option<BkStat>,
    ///
    #[prost(int64, tag = "6")]
    pub last_part: i64,
    ///
    #[prost(int64, tag = "7")]
    pub progress: i64,
    ///
    #[prost(int32, tag = "8")]
    pub playable: i32,
    ///
    #[prost(string, tag = "9")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(map = "int64, message", tag = "10")]
    pub player_info: ::std::collections::HashMap<i64, PlayInfo>,
    ///
    #[prost(message, optional, tag = "11")]
    pub associated_item: ::core::option::Option<PlayItem>,
    ///
    #[prost(int64, tag = "12")]
    pub last_play_time: i64,
    ///
    #[prost(string, tag = "13")]
    pub history_tag: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "14")]
    pub device_type: ::core::option::Option<super::super::interface::v1::DeviceType>,
    ///
    #[prost(message, optional, tag = "15")]
    pub ugc_season_info: ::core::option::Option<FavFolder>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventReq {
    ///
    #[prost(int32, tag = "1")]
    pub event_type: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<PlayItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventResp {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTracking {
    ///
    #[prost(string, tag = "1")]
    pub operator: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub batch: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub entity_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub entity_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub track_json: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolder {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub owner: ::core::option::Option<FavFolderAuthor>,
    ///
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub count: i32,
    ///
    #[prost(int32, tag = "8")]
    pub attr: i32,
    ///
    #[prost(int32, tag = "9")]
    pub state: i32,
    ///
    #[prost(int32, tag = "10")]
    pub favored: i32,
    ///
    #[prost(int64, tag = "11")]
    pub ctime: i64,
    ///
    #[prost(int64, tag = "12")]
    pub mtime: i64,
    ///
    #[prost(int32, tag = "13")]
    pub stat_fav_cnt: i32,
    ///
    #[prost(int32, tag = "14")]
    pub stat_share_cnt: i32,
    ///
    #[prost(int32, tag = "15")]
    pub stat_like_cnt: i32,
    ///
    #[prost(int32, tag = "16")]
    pub stat_play_cnt: i32,
    ///
    #[prost(int32, tag = "17")]
    pub stat_reply_cnt: i32,
    ///
    #[prost(int32, tag = "18")]
    pub fav_state: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderAction {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
    ///
    #[prost(int32, tag = "3")]
    pub action: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderAuthor {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderCreateReq {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub public: i32,
    ///
    #[prost(int32, tag = "4")]
    pub folder_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderCreateResp {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
    ///
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderDeleteReq {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderDeleteResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderDetailReq {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub fav_mid: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub last_item: ::core::option::Option<FavItem>,
    ///
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    ///
    #[prost(bool, tag = "6")]
    pub need_folder_info: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderDetailResp {
    ///
    #[prost(int32, tag = "1")]
    pub total: i32,
    ///
    #[prost(bool, tag = "2")]
    pub reach_end: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<FavItemDetail>,
    ///
    #[prost(message, optional, tag = "4")]
    pub folder_info: ::core::option::Option<FavFolder>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderListReq {
    ///
    #[prost(int32, repeated, tag = "1")]
    pub folder_types: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<PlayItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderListResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<FavFolder>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavFolderMeta {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItem {
    ///
    #[prost(int32, tag = "1")]
    pub item_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub fid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub mtime: i64,
    ///
    #[prost(int64, tag = "6")]
    pub ctime: i64,
    ///
    #[prost(message, optional, tag = "7")]
    pub et: ::core::option::Option<EventTracking>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemAddReq {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
    #[prost(oneof = "fav_item_add_req::Item", tags = "3, 4")]
    pub item: ::core::option::Option<fav_item_add_req::Item>,
}
/// Nested message and enum types in `FavItemAddReq`.
pub mod fav_item_add_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        ///
        #[prost(message, tag = "3")]
        Play(super::PlayItem),
        ///
        #[prost(message, tag = "4")]
        Fav(super::FavItem),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemAddResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemAuthor {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemBatchReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<FavFolderAction>,
    #[prost(oneof = "fav_item_batch_req::Item", tags = "2, 3")]
    pub item: ::core::option::Option<fav_item_batch_req::Item>,
}
/// Nested message and enum types in `FavItemBatchReq`.
pub mod fav_item_batch_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        ///
        #[prost(message, tag = "2")]
        Play(super::PlayItem),
        ///
        #[prost(message, tag = "3")]
        Fav(super::FavItem),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemBatchResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemDelReq {
    ///
    #[prost(int64, tag = "1")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub folder_type: i32,
    #[prost(oneof = "fav_item_del_req::Item", tags = "3, 4")]
    pub item: ::core::option::Option<fav_item_del_req::Item>,
}
/// Nested message and enum types in `FavItemDelReq`.
pub mod fav_item_del_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        ///
        #[prost(message, tag = "3")]
        Play(super::PlayItem),
        ///
        #[prost(message, tag = "4")]
        Fav(super::FavItem),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemDelResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemDetail {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<FavItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub owner: ::core::option::Option<FavItemAuthor>,
    ///
    #[prost(message, optional, tag = "3")]
    pub stat: ::core::option::Option<FavItemStat>,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub duration: i64,
    ///
    #[prost(int32, tag = "7")]
    pub state: i32,
    ///
    #[prost(string, tag = "8")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub parts: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavItemStat {
    ///
    #[prost(int32, tag = "1")]
    pub view: i32,
    ///
    #[prost(int32, tag = "2")]
    pub reply: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavoredInAnyFoldersReq {
    ///
    #[prost(int32, repeated, tag = "1")]
    pub folder_types: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<PlayItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavoredInAnyFoldersResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<FavFolderMeta>,
    ///
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<PlayItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavTabShowReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavTabShowResp {
    ///
    #[prost(bool, tag = "1")]
    pub show_menu: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowRelation {
    ///
    #[prost(int32, tag = "1")]
    pub status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatDescription {
    ///
    #[prost(int32, tag = "1")]
    pub quality: i32,
    ///
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub display_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub superscript: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainFavMusicMenuListReq {
    ///
    #[prost(int32, tag = "1")]
    pub tab_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub offset: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainFavMusicMenuListResp {
    ///
    #[prost(int32, tag = "1")]
    pub tab_type: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub menu_list: ::prost::alloc::vec::Vec<MusicMenu>,
    ///
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "4")]
    pub offset: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainFavMusicSubTabListReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainFavMusicSubTabListResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub tabs: ::prost::alloc::vec::Vec<MusicSubTab>,
    ///
    #[prost(message, optional, tag = "2")]
    pub default_tab_res: ::core::option::Option<MainFavMusicMenuListResp>,
    ///
    #[prost(map = "int32, message", tag = "3")]
    pub first_page_res: ::std::collections::HashMap<i32, MainFavMusicMenuListResp>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MedialistItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub duration: i64,
    ///
    #[prost(int32, tag = "5")]
    pub parts: i32,
    ///
    #[prost(int64, tag = "6")]
    pub up_mid: i64,
    ///
    #[prost(string, tag = "7")]
    pub up_name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub state: i32,
    ///
    #[prost(string, tag = "9")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub stat_view: i64,
    ///
    #[prost(int64, tag = "11")]
    pub stat_reply: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MedialistReq {
    ///
    #[prost(int64, tag = "1")]
    pub list_type: i64,
    ///
    #[prost(int64, tag = "2")]
    pub biz_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MedialistResp {
    ///
    #[prost(int64, tag = "1")]
    pub total: i64,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub items: ::prost::alloc::vec::Vec<MedialistItem>,
    ///
    #[prost(message, optional, tag = "5")]
    pub up_info: ::core::option::Option<MedialistUpInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MedialistUpInfo {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub fans: i64,
    ///
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuDeleteReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuDeleteResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuEditReq {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub is_public: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuEditResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuSubscribeReq {
    ///
    #[prost(int32, tag = "1")]
    pub action: i32,
    ///
    #[prost(int64, tag = "2")]
    pub target_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuSubscribeResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MusicMenu {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub menu_type: i32,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub owner: ::core::option::Option<MusicMenuAuthor>,
    ///
    #[prost(int32, tag = "7")]
    pub state: i32,
    ///
    #[prost(int64, tag = "8")]
    pub attr: i64,
    ///
    #[prost(message, optional, tag = "9")]
    pub stat: ::core::option::Option<MusicMenuStat>,
    ///
    #[prost(int64, tag = "10")]
    pub total: i64,
    ///
    #[prost(int64, tag = "11")]
    pub ctime: i64,
    ///
    #[prost(string, tag = "12")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MusicMenuAuthor {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub avatar: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MusicMenuStat {
    ///
    #[prost(int64, tag = "1")]
    pub play: i64,
    ///
    #[prost(int64, tag = "2")]
    pub reply: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MusicSubTab {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub tab_type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub total: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageOption {
    ///
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    ///
    #[prost(int32, tag = "2")]
    pub direction: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub last_item: ::core::option::Option<PlayItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickArchive {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub owner: ::core::option::Option<PickArchiveAuthor>,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub duration: i64,
    ///
    #[prost(int32, tag = "6")]
    pub parts: i32,
    ///
    #[prost(int32, tag = "7")]
    pub stat_view: i32,
    ///
    #[prost(int32, tag = "8")]
    pub stat_reply: i32,
    ///
    #[prost(int32, tag = "9")]
    pub state: i32,
    ///
    #[prost(string, tag = "10")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickArchiveAuthor {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickCard {
    ///
    #[prost(int64, tag = "1")]
    pub pick_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub card_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub card_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub modules: ::prost::alloc::vec::Vec<CardModule>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickCardDetailReq {
    ///
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub pick_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickCardDetailResp {
    ///
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub pick_id: i64,
    ///
    #[prost(message, repeated, tag = "3")]
    pub modules: ::prost::alloc::vec::Vec<CardModule>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFeedReq {
    ///
    #[prost(int64, tag = "1")]
    pub offset: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFeedResp {
    ///
    #[prost(int64, tag = "1")]
    pub offset: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub cards: ::prost::alloc::vec::Vec<PickCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkcmArchive {
    ///
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<PickArchive>,
    ///
    #[prost(string, tag = "2")]
    pub pick_reason: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkcmCenterButton {
    ///
    #[prost(string, tag = "1")]
    pub icon_head: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_tail: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkcmHeader {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub btn_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub btn_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub btn_uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayActionReportReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(string, tag = "2")]
    pub from_spmid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayDash {
    ///
    #[prost(int32, tag = "1")]
    pub duration: i32,
    ///
    #[prost(float, tag = "2")]
    pub min_buffer_time: f32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub audio: ::prost::alloc::vec::Vec<DashItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayHistoryAddReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(int64, tag = "2")]
    pub progress: i64,
    ///
    #[prost(int64, tag = "3")]
    pub duration: i64,
    ///
    #[prost(int32, tag = "4")]
    pub play_style: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayHistoryDelReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<PlayItem>,
    ///
    #[prost(bool, tag = "2")]
    pub truncate: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayHistoryReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub page_opt: ::core::option::Option<PageOption>,
    ///
    #[prost(int64, tag = "2")]
    pub local_today_zero: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayHistoryResp {
    ///
    #[prost(int32, tag = "1")]
    pub total: i32,
    ///
    #[prost(bool, tag = "2")]
    pub reach_end: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<DetailItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayInfo {
    ///
    #[prost(int32, tag = "1")]
    pub qn: i32,
    ///
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub qn_type: i32,
    #[prost(int32, tag = "6")]
    pub fnver: i32,
    ///
    #[prost(int32, tag = "7")]
    pub fnval: i32,
    ///
    #[prost(int32, repeated, tag = "8")]
    pub formats: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(int32, tag = "9")]
    pub video_codecid: i32,
    ///
    #[prost(int64, tag = "10")]
    pub length: i64,
    ///
    #[prost(int32, tag = "11")]
    pub code: i32,
    ///
    #[prost(string, tag = "12")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "13")]
    pub expire_time: i64,
    ///
    #[prost(message, optional, tag = "14")]
    pub volume: ::core::option::Option<super::super::playurl::v1::VolumeInfo>,
    #[prost(oneof = "play_info::Info", tags = "4, 5")]
    pub info: ::core::option::Option<play_info::Info>,
}
/// Nested message and enum types in `PlayInfo`.
pub mod play_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Info {
        ///
        #[prost(message, tag = "4")]
        PlayUrl(super::PlayUrl),
        ///
        #[prost(message, tag = "5")]
        PlayDash(super::PlayDash),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayItem {
    ///
    #[prost(int32, tag = "1")]
    pub item_type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub oid: i64,
    ///
    #[prost(int64, repeated, tag = "4")]
    pub sub_id: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(message, optional, tag = "5")]
    pub et: ::core::option::Option<EventTracking>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistAddReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<PlayItem>,
    #[prost(oneof = "playlist_add_req::Pos", tags = "2, 3, 4")]
    pub pos: ::core::option::Option<playlist_add_req::Pos>,
}
/// Nested message and enum types in `PlaylistAddReq`.
pub mod playlist_add_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pos {
        ///
        #[prost(message, tag = "2")]
        After(super::PlayItem),
        ///
        #[prost(bool, tag = "3")]
        Head(bool),
        ///
        #[prost(bool, tag = "4")]
        Tail(bool),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistDelReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<PlayItem>,
    ///
    #[prost(bool, tag = "2")]
    pub truncate: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistReq {
    ///
    #[prost(int32, tag = "1")]
    pub from: i32,
    ///
    #[prost(int64, tag = "2")]
    pub id: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub anchor: ::core::option::Option<PlayItem>,
    ///
    #[prost(message, optional, tag = "4")]
    pub page_opt: ::core::option::Option<PageOption>,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "6")]
    pub extra_id: i64,
    ///
    #[prost(message, optional, tag = "7")]
    pub sort_opt: ::core::option::Option<SortOption>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaylistResp {
    ///
    #[prost(int32, tag = "1")]
    pub total: i32,
    ///
    #[prost(bool, tag = "2")]
    pub reach_start: bool,
    ///
    #[prost(bool, tag = "3")]
    pub reach_end: bool,
    ///
    #[prost(message, repeated, tag = "4")]
    pub list: ::prost::alloc::vec::Vec<DetailItem>,
    ///
    #[prost(message, optional, tag = "5")]
    pub last_play: ::core::option::Option<PlayItem>,
    ///
    #[prost(int64, tag = "6")]
    pub last_progress: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayUrl {
    ///
    #[prost(message, repeated, tag = "1")]
    pub durl: ::prost::alloc::vec::Vec<ResponseUrl>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayUrlReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(message, optional, tag = "2")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayUrlResp {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(int32, tag = "2")]
    pub playable: i32,
    ///
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(map = "int64, message", tag = "4")]
    pub player_info: ::std::collections::HashMap<i64, PlayInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdPlaylistReq {
    ///
    #[prost(int32, tag = "1")]
    pub from: i32,
    ///
    #[prost(int64, tag = "2")]
    pub id: i64,
    ///
    #[prost(bool, tag = "3")]
    pub need_history: bool,
    ///
    #[prost(bool, tag = "4")]
    pub need_top_cards: bool,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdPlaylistResp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<DetailItem>,
    ///
    #[prost(int64, tag = "2")]
    pub history_len: i64,
    ///
    #[prost(message, repeated, tag = "3")]
    pub top_cards: ::prost::alloc::vec::Vec<TopCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseUrl {
    ///
    #[prost(int32, tag = "1")]
    pub order: i32,
    ///
    #[prost(int64, tag = "2")]
    pub length: i64,
    ///
    #[prost(int64, tag = "3")]
    pub size: i64,
    ///
    #[prost(string, tag = "4")]
    pub ahead: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub vhead: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "7")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "8")]
    pub md5: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortOption {
    ///
    #[prost(int32, tag = "1")]
    pub order: i32,
    ///
    #[prost(int32, tag = "2")]
    pub sort_field: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThumbUpReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
    ///
    #[prost(int32, tag = "2")]
    pub action: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThumbUpResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub play_style: i32,
    ///
    #[prost(int32, tag = "3")]
    pub card_type: i32,
    ///
    #[prost(int64, tag = "8")]
    pub pos: i64,
    ///
    #[prost(string, tag = "9")]
    pub title_icon: ::prost::alloc::string::String,
    ///
    #[prost(oneof = "top_card::Card", tags = "4, 5, 6, 7")]
    pub card: ::core::option::Option<top_card::Card>,
}
/// Nested message and enum types in `TopCard`.
pub mod top_card {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Card {
        ///
        #[prost(message, tag = "4")]
        ListenHistory(super::TpcdHistory),
        ///
        #[prost(message, tag = "5")]
        FavFolder(super::TpcdFavFolder),
        ///
        #[prost(message, tag = "6")]
        UpRecall(super::TpcdUpRecall),
        ///
        #[prost(message, tag = "7")]
        PickToday(super::TpcdPickToday),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpcdFavFolder {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<DetailItem>,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub pic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub fid: i64,
    ///
    #[prost(int32, tag = "5")]
    pub folder_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpcdHistory {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<DetailItem>,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub pic: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpcdPickToday {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<DetailItem>,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub pic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub pick_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub pick_card_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpcdUpRecall {
    ///
    #[prost(int64, tag = "1")]
    pub up_mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub avatar: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub medialist_type: i64,
    ///
    #[prost(int64, tag = "5")]
    pub medialist_biz_id: i64,
    ///
    #[prost(message, optional, tag = "6")]
    pub item: ::core::option::Option<DetailItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleLikeReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<PlayItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleLikeResp {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub thumb_ok: bool,
    ///
    #[prost(bool, tag = "3")]
    pub coin_ok: bool,
    ///
    #[prost(bool, tag = "4")]
    pub fav_ok: bool,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CardModuleType {
    ModuleInvalid = 0,
    ModuleHeader = 1,
    ModuleArchive = 2,
    ModuleCbtn = 3,
}
impl CardModuleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CardModuleType::ModuleInvalid => "Module_invalid",
            CardModuleType::ModuleHeader => "Module_header",
            CardModuleType::ModuleArchive => "Module_archive",
            CardModuleType::ModuleCbtn => "Module_cbtn",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Module_invalid" => Some(Self::ModuleInvalid),
            "Module_header" => Some(Self::ModuleHeader),
            "Module_archive" => Some(Self::ModuleArchive),
            "Module_cbtn" => Some(Self::ModuleCbtn),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ListOrder {
    ///
    NoOrder = 0,
    ///
    OrderNormal = 1,
    ///
    OrderReverse = 2,
    ///
    OrderRandom = 3,
}
impl ListOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ListOrder::NoOrder => "NO_ORDER",
            ListOrder::OrderNormal => "ORDER_NORMAL",
            ListOrder::OrderReverse => "ORDER_REVERSE",
            ListOrder::OrderRandom => "ORDER_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_ORDER" => Some(Self::NoOrder),
            "ORDER_NORMAL" => Some(Self::OrderNormal),
            "ORDER_REVERSE" => Some(Self::OrderReverse),
            "ORDER_RANDOM" => Some(Self::OrderRandom),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ListSortField {
    ///
    NoSort = 0,
    ///
    SortCtime = 1,
    ///
    SortViewcnt = 2,
    ///
    SortFavcnt = 3,
}
impl ListSortField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ListSortField::NoSort => "NO_SORT",
            ListSortField::SortCtime => "SORT_CTIME",
            ListSortField::SortViewcnt => "SORT_VIEWCNT",
            ListSortField::SortFavcnt => "SORT_FAVCNT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_SORT" => Some(Self::NoSort),
            "SORT_CTIME" => Some(Self::SortCtime),
            "SORT_VIEWCNT" => Some(Self::SortViewcnt),
            "SORT_FAVCNT" => Some(Self::SortFavcnt),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlaylistSource {
    ///
    Default = 0,
    ///
    MemSpace = 1,
    ///
    AudioCollection = 2,
    ///
    AudioCard = 3,
    ///
    UserFavourite = 4,
    ///
    UpArchive = 5,
    ///
    AudioCache = 6,
    ///
    PickCard = 7,
    ///
    MediaList = 8,
}
impl PlaylistSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlaylistSource::Default => "DEFAULT",
            PlaylistSource::MemSpace => "MEM_SPACE",
            PlaylistSource::AudioCollection => "AUDIO_COLLECTION",
            PlaylistSource::AudioCard => "AUDIO_CARD",
            PlaylistSource::UserFavourite => "USER_FAVOURITE",
            PlaylistSource::UpArchive => "UP_ARCHIVE",
            PlaylistSource::AudioCache => "AUDIO_CACHE",
            PlaylistSource::PickCard => "PICK_CARD",
            PlaylistSource::MediaList => "MEDIA_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "MEM_SPACE" => Some(Self::MemSpace),
            "AUDIO_COLLECTION" => Some(Self::AudioCollection),
            "AUDIO_CARD" => Some(Self::AudioCard),
            "USER_FAVOURITE" => Some(Self::UserFavourite),
            "UP_ARCHIVE" => Some(Self::UpArchive),
            "AUDIO_CACHE" => Some(Self::AudioCache),
            "PICK_CARD" => Some(Self::PickCard),
            "MEDIA_LIST" => Some(Self::MediaList),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TopCardType {
    ///
    Unspecified = 0,
    ///
    ListenHistory = 1,
    ///
    FavoriteFolder = 2,
    ///
    UpRecall = 3,
    ///
    PickToday = 4,
}
impl TopCardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TopCardType::Unspecified => "UNSPECIFIED",
            TopCardType::ListenHistory => "LISTEN_HISTORY",
            TopCardType::FavoriteFolder => "FAVORITE_FOLDER",
            TopCardType::UpRecall => "UP_RECALL",
            TopCardType::PickToday => "PICK_TODAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "LISTEN_HISTORY" => Some(Self::ListenHistory),
            "FAVORITE_FOLDER" => Some(Self::FavoriteFolder),
            "UP_RECALL" => Some(Self::UpRecall),
            "PICK_TODAY" => Some(Self::PickToday),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod listener_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 听视频
    #[derive(Debug, Clone)]
    pub struct ListenerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ListenerClient<tonic::transport::Channel> {
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
    impl<T> ListenerClient<T>
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
        ) -> ListenerClient<InterceptedService<T, F>>
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
            ListenerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.app.listener.v1.Listener/Ping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Listener", "Ping"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取音频URL
        pub async fn play_url(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayUrlReq>,
        ) -> std::result::Result<tonic::Response<super::PlayUrlResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/PlayUrl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Listener", "PlayUrl"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn bkarc_details(
            &mut self,
            request: impl tonic::IntoRequest<super::BkArcDetailsReq>,
        ) -> std::result::Result<
            tonic::Response<super::BkArcDetailsResp>,
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
                "/bilibili.app.listener.v1.Listener/BkarcDetails",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "BkarcDetails"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn playlist(
            &mut self,
            request: impl tonic::IntoRequest<super::PlaylistReq>,
        ) -> std::result::Result<tonic::Response<super::PlaylistResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/Playlist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "Playlist"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn playlist_add(
            &mut self,
            request: impl tonic::IntoRequest<super::PlaylistAddReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.app.listener.v1.Listener/PlaylistAdd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "PlaylistAdd"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn playlist_del(
            &mut self,
            request: impl tonic::IntoRequest<super::PlaylistDelReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.app.listener.v1.Listener/PlaylistDel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "PlaylistDel"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 推荐列表
        pub async fn rcmd_playlist(
            &mut self,
            request: impl tonic::IntoRequest<super::RcmdPlaylistReq>,
        ) -> std::result::Result<
            tonic::Response<super::RcmdPlaylistResp>,
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
                "/bilibili.app.listener.v1.Listener/RcmdPlaylist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "RcmdPlaylist"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn play_history(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayHistoryReq>,
        ) -> std::result::Result<
            tonic::Response<super::PlayHistoryResp>,
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
                "/bilibili.app.listener.v1.Listener/PlayHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "PlayHistory"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 添加历史记录
        pub async fn play_history_add(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayHistoryAddReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.app.listener.v1.Listener/PlayHistoryAdd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "PlayHistoryAdd",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn play_history_del(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayHistoryDelReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.app.listener.v1.Listener/PlayHistoryDel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "PlayHistoryDel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 播放上报
        pub async fn play_action_report(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayActionReportReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.app.listener.v1.Listener/PlayActionReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "PlayActionReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 三联
        pub async fn triple_like(
            &mut self,
            request: impl tonic::IntoRequest<super::TripleLikeReq>,
        ) -> std::result::Result<tonic::Response<super::TripleLikeResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/TripleLike",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "TripleLike"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 点赞
        pub async fn thumb_up(
            &mut self,
            request: impl tonic::IntoRequest<super::ThumbUpReq>,
        ) -> std::result::Result<tonic::Response<super::ThumbUpResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/ThumbUp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Listener", "ThumbUp"));
            self.inner.unary(req, path, codec).await
        }
        /// 投币
        pub async fn coin_add(
            &mut self,
            request: impl tonic::IntoRequest<super::CoinAddReq>,
        ) -> std::result::Result<tonic::Response<super::CoinAddResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/CoinAdd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Listener", "CoinAdd"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn fav_item_add(
            &mut self,
            request: impl tonic::IntoRequest<super::FavItemAddReq>,
        ) -> std::result::Result<tonic::Response<super::FavItemAddResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/FavItemAdd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "FavItemAdd"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn fav_item_del(
            &mut self,
            request: impl tonic::IntoRequest<super::FavItemDelReq>,
        ) -> std::result::Result<tonic::Response<super::FavItemDelResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/FavItemDel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "FavItemDel"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 批量处理收藏
        pub async fn fav_item_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::FavItemBatchReq>,
        ) -> std::result::Result<
            tonic::Response<super::FavItemBatchResp>,
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
                "/bilibili.app.listener.v1.Listener/FavItemBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "FavItemBatch"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn favored_in_any_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::FavoredInAnyFoldersReq>,
        ) -> std::result::Result<
            tonic::Response<super::FavoredInAnyFoldersResp>,
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
                "/bilibili.app.listener.v1.Listener/FavoredInAnyFolders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "FavoredInAnyFolders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 用户收藏夹列表
        pub async fn fav_folder_list(
            &mut self,
            request: impl tonic::IntoRequest<super::FavFolderListReq>,
        ) -> std::result::Result<
            tonic::Response<super::FavFolderListResp>,
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
                "/bilibili.app.listener.v1.Listener/FavFolderList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "FavFolderList"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 收藏夹详细信息
        pub async fn fav_folder_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::FavFolderDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::FavFolderDetailResp>,
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
                "/bilibili.app.listener.v1.Listener/FavFolderDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "FavFolderDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 创建收藏夹
        pub async fn fav_folder_create(
            &mut self,
            request: impl tonic::IntoRequest<super::FavFolderCreateReq>,
        ) -> std::result::Result<
            tonic::Response<super::FavFolderCreateResp>,
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
                "/bilibili.app.listener.v1.Listener/FavFolderCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "FavFolderCreate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn fav_folder_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::FavFolderDeleteReq>,
        ) -> std::result::Result<
            tonic::Response<super::FavFolderDeleteResp>,
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
                "/bilibili.app.listener.v1.Listener/FavFolderDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "FavFolderDelete",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 每日播单列表
        pub async fn pick_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::PickFeedReq>,
        ) -> std::result::Result<tonic::Response<super::PickFeedResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/PickFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "PickFeed"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 每日播单详情
        pub async fn pick_card_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::PickCardDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::PickCardDetailResp>,
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
                "/bilibili.app.listener.v1.Listener/PickCardDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Listener",
                        "PickCardDetail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn medialist(
            &mut self,
            request: impl tonic::IntoRequest<super::MedialistReq>,
        ) -> std::result::Result<tonic::Response<super::MedialistResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/Medialist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Listener", "Medialist"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn event(
            &mut self,
            request: impl tonic::IntoRequest<super::EventReq>,
        ) -> std::result::Result<tonic::Response<super::EventResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Listener/Event",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Listener", "Event"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod music_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct MusicClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MusicClient<tonic::transport::Channel> {
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
    impl<T> MusicClient<T>
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
        ) -> MusicClient<InterceptedService<T, F>>
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
            MusicClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn fav_tab_show(
            &mut self,
            request: impl tonic::IntoRequest<super::FavTabShowReq>,
        ) -> std::result::Result<tonic::Response<super::FavTabShowResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Music/FavTabShow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Music", "FavTabShow"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn main_fav_music_sub_tab_list(
            &mut self,
            request: impl tonic::IntoRequest<super::MainFavMusicSubTabListReq>,
        ) -> std::result::Result<
            tonic::Response<super::MainFavMusicSubTabListResp>,
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
                "/bilibili.app.listener.v1.Music/MainFavMusicSubTabList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Music",
                        "MainFavMusicSubTabList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn main_fav_music_menu_list(
            &mut self,
            request: impl tonic::IntoRequest<super::MainFavMusicMenuListReq>,
        ) -> std::result::Result<
            tonic::Response<super::MainFavMusicMenuListResp>,
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
                "/bilibili.app.listener.v1.Music/MainFavMusicMenuList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.listener.v1.Music",
                        "MainFavMusicMenuList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn menu_edit(
            &mut self,
            request: impl tonic::IntoRequest<super::MenuEditReq>,
        ) -> std::result::Result<tonic::Response<super::MenuEditResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Music/MenuEdit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Music", "MenuEdit"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn menu_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::MenuDeleteReq>,
        ) -> std::result::Result<tonic::Response<super::MenuDeleteResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Music/MenuDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Music", "MenuDelete"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn menu_subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::MenuSubscribeReq>,
        ) -> std::result::Result<
            tonic::Response<super::MenuSubscribeResp>,
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
                "/bilibili.app.listener.v1.Music/MenuSubscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.listener.v1.Music", "MenuSubscribe"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn click(
            &mut self,
            request: impl tonic::IntoRequest<super::ClickReq>,
        ) -> std::result::Result<tonic::Response<super::ClickResp>, tonic::Status> {
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
                "/bilibili.app.listener.v1.Music/Click",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.listener.v1.Music", "Click"));
            self.inner.unary(req, path, codec).await
        }
    }
}
