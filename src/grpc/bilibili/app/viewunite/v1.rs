///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttentionCard {
    ///
    #[prost(message, repeated, tag = "1")]
    pub show_time: ::prost::alloc::vec::Vec<ShowTime>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeType {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub border_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub bg_style: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizFollowVideoParam {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizJumpLinkParam {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizReserveActivityParam {
    ///
    #[prost(int64, tag = "1")]
    pub activity_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub reserve_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizReserveGameParam {
    ///
    #[prost(int64, tag = "1")]
    pub game_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chronos {
    ///
    #[prost(string, tag = "1")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub file: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sign: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChronosParam {
    ///
    #[prost(string, tag = "1")]
    pub engine_version: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub message_protocol: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub service_key: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDm {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(string, tag = "4")]
    pub command: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub progress: i32,
    ///
    #[prost(string, tag = "7")]
    pub ctime: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub mtime: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub extra: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub id_str: ::prost::alloc::string::String,
}
/// 视频播放时弹出的卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCard {
    /// 在第几秒弹出
    #[prost(float, tag = "1")]
    pub display_progress: f32,
    ///
    #[prost(int64, tag = "2")]
    pub display_accuracy: i64,
    /// 弹出后停留的时间
    #[prost(int64, tag = "3")]
    pub display_duration: i64,
    /// 展示方式, 暂未知对应关系
    #[prost(int32, tag = "4")]
    pub show_mode: i32,
    /// 页面类型, 暂未知对应关系
    #[prost(int32, tag = "5")]
    pub page_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub upper: ::core::option::Option<UpperInfos>,
    ///
    #[prost(int32, tag = "7")]
    pub is_follow_display: i32,
    /// 卡片的文字说明信息
    #[prost(message, optional, tag = "8")]
    pub text: ::core::option::Option<ContractText>,
    ///
    #[prost(int64, tag = "9")]
    pub follow_display_end_duration: i64,
    ///
    #[prost(int32, tag = "10")]
    pub is_play_display: i32,
    ///
    #[prost(int32, tag = "11")]
    pub is_interact_display: i32,
}
/// 视频播放时弹出的卡片的文字说明信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractText {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub inline_title: ::prost::alloc::string::String,
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
pub struct DmResource {
    ///
    #[prost(message, repeated, tag = "1")]
    pub command_dms: ::prost::alloc::vec::Vec<CommandDm>,
    ///
    #[prost(message, optional, tag = "2")]
    pub attention: ::core::option::Option<AttentionCard>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub cards: ::prost::alloc::vec::Vec<OperationCard>,
}
/// 素材详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Material {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "MaterialBizType", tag = "4")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "5")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub static_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub bg_pic: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub jump_type: i32,
    ///
    #[prost(enumeration = "PageType", tag = "10")]
    pub page_type: i32,
    ///
    #[prost(bool, tag = "11")]
    pub need_login: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCard {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub from: i32,
    ///
    #[prost(int32, tag = "3")]
    pub to: i32,
    ///
    #[prost(bool, tag = "4")]
    pub status: bool,
    ///
    #[prost(enumeration = "BizType", tag = "5")]
    pub biz_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub content: ::core::option::Option<OperationCardContent>,
    ///
    #[prost(message, optional, tag = "7")]
    pub follow: ::core::option::Option<BizFollowVideoParam>,
    ///
    #[prost(message, optional, tag = "8")]
    pub reserve: ::core::option::Option<BizReserveActivityParam>,
    ///
    #[prost(message, optional, tag = "9")]
    pub jump: ::core::option::Option<BizJumpLinkParam>,
    ///
    #[prost(message, optional, tag = "10")]
    pub game: ::core::option::Option<BizReserveGameParam>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCardContent {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub button_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub button_selected_title: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "6")]
    pub show_selected: bool,
}
/// 播放策略
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayStrategy {
    ///
    #[prost(string, repeated, tag = "1")]
    pub strategies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "2")]
    pub recommend_show_strategy: i32,
    /// 自动播放时的提示语
    #[prost(string, tag = "3")]
    pub auto_play_toast: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointMaterial {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "MaterialSource", tag = "2")]
    pub material_source: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Selection {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub item: ::prost::alloc::vec::Vec<SelectionItem>,
    ///
    #[prost(enumeration = "ArcType", tag = "3")]
    pub arc_type: i32,
    ///
    #[prost(enumeration = "SelectionType", tag = "4")]
    pub selection_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectionItem {
    ///
    #[prost(uint64, tag = "1")]
    pub aid: u64,
    ///
    #[prost(uint64, tag = "2")]
    pub cid: u64,
    ///
    #[prost(message, optional, tag = "4")]
    pub badge_type: ::core::option::Option<BadgeType>,
    ///
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub long_title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub dimension: ::core::option::Option<Dimension>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectionModule {
    ///
    #[prost(message, repeated, tag = "1")]
    pub selection: ::prost::alloc::vec::Vec<Selection>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub serial_season: ::prost::alloc::vec::Vec<SerialSeason>,
    ///
    #[prost(message, optional, tag = "3")]
    pub play_strategy: ::core::option::Option<PlayStrategy>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerialSeason {
    ///
    #[prost(uint32, tag = "1")]
    pub season_id: u32,
    ///
    #[prost(string, tag = "2")]
    pub season_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowTime {
    ///
    #[prost(int32, tag = "1")]
    pub start_time: i32,
    ///
    #[prost(int32, tag = "2")]
    pub end_time: i32,
    ///
    #[prost(double, tag = "3")]
    pub pos_x: f64,
    ///
    #[prost(double, tag = "4")]
    pub pos_y: f64,
}
/// UP主信息(可是Upper这个... 程序员英文不过关吧? )
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpperInfos {
    /// 粉丝数
    #[prost(uint64, tag = "1")]
    pub fans_count: u64,
    /// 过去半年内的稿件数
    #[prost(uint64, tag = "2")]
    pub arc_count_last_half_year: u64,
    ///
    #[prost(int64, tag = "3")]
    pub first_up_dates: i64,
    /// UP稿件总播放数
    #[prost(uint64, tag = "4")]
    pub total_play_count: u64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoGuide {
    ///
    #[prost(message, repeated, tag = "1")]
    pub material: ::prost::alloc::vec::Vec<Material>,
    ///
    #[prost(message, optional, tag = "2")]
    pub video_point: ::core::option::Option<VideoViewPoint>,
    ///
    #[prost(message, optional, tag = "3")]
    pub contract_card: ::core::option::Option<ContractCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoPoint {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub from: i64,
    ///
    #[prost(int64, tag = "3")]
    pub to: i64,
    ///
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub logo_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoShot {
    ///
    #[prost(string, tag = "1")]
    pub pv_data: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub img_x_len: i32,
    ///
    #[prost(int32, tag = "3")]
    pub imd_x_size: i32,
    ///
    #[prost(int32, tag = "4")]
    pub img_y_len: i32,
    ///
    #[prost(int32, tag = "5")]
    pub img_y_size: i32,
    ///
    #[prost(string, repeated, tag = "6")]
    pub image: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoViewPoint {
    ///
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<VideoPoint>,
    ///
    #[prost(message, optional, tag = "2")]
    pub point_material: ::core::option::Option<PointMaterial>,
    ///
    #[prost(bool, tag = "3")]
    pub point_permanent: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewBase {
    ///
    #[prost(enumeration = "UnionType", tag = "1")]
    pub union_type: i32,
    ///
    #[prost(enumeration = "PageType", tag = "2")]
    pub page_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewProgressReq {
    ///
    #[prost(uint64, tag = "1")]
    pub aid: u64,
    ///
    #[prost(uint64, tag = "2")]
    pub cid: u64,
    ///
    #[prost(uint64, tag = "3")]
    pub up_mid: u64,
    ///
    #[prost(message, optional, tag = "4")]
    pub chronos_param: ::core::option::Option<ChronosParam>,
    ///
    #[prost(enumeration = "UnionType", tag = "5")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewProgressReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub video_guide: ::core::option::Option<VideoGuide>,
    ///
    #[prost(message, optional, tag = "2")]
    pub chronos: ::core::option::Option<Chronos>,
    ///
    #[prost(message, optional, tag = "3")]
    pub arc_shot: ::core::option::Option<VideoShot>,
    ///
    #[prost(message, optional, tag = "4")]
    pub dm: ::core::option::Option<DmResource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewReq {
    ///
    #[prost(uint64, tag = "1")]
    pub aid: u64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(string, tag = "8")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "9")]
    pub extra_content: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub view_base: ::core::option::Option<ViewBase>,
    ///
    #[prost(message, optional, tag = "3")]
    pub selection_module: ::core::option::Option<SelectionModule>,
    /// 使用 pgcanymodel / ugcanymodel 进行proto any转换成对应业务码结构体
    #[prost(message, optional, tag = "4")]
    pub supplement: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
///   业务类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArcType {
    ///
    Unknown = 0,
    ///
    Pages = 1,
    ///
    Series = 2,
    ///
    Positive = 3,
    ///
    Section = 4,
    ///
    Relate = 5,
    ///
    Pugv = 6,
}
impl ArcType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArcType::Unknown => "UNKNOWN",
            ArcType::Pages => "PAGES",
            ArcType::Series => "SERIES",
            ArcType::Positive => "POSITIVE",
            ArcType::Section => "SECTION",
            ArcType::Relate => "RELATE",
            ArcType::Pugv => "PUGV",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PAGES" => Some(Self::Pages),
            "SERIES" => Some(Self::Series),
            "POSITIVE" => Some(Self::Positive),
            "SECTION" => Some(Self::Section),
            "RELATE" => Some(Self::Relate),
            "PUGV" => Some(Self::Pugv),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BizType {
    ///
    None = 0,
    ///
    FollowVideo = 1,
    ///
    ReserveActivity = 2,
    ///
    JumpLink = 3,
    ///
    FavSeason = 4,
    ///
    ReserveGame = 5,
}
impl BizType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BizType::None => "BizTypeNone",
            BizType::FollowVideo => "BizTypeFollowVideo",
            BizType::ReserveActivity => "BizTypeReserveActivity",
            BizType::JumpLink => "BizTypeJumpLink",
            BizType::FavSeason => "BizTypeFavSeason",
            BizType::ReserveGame => "BizTypeReserveGame",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BizTypeNone" => Some(Self::None),
            "BizTypeFollowVideo" => Some(Self::FollowVideo),
            "BizTypeReserveActivity" => Some(Self::ReserveActivity),
            "BizTypeJumpLink" => Some(Self::JumpLink),
            "BizTypeFavSeason" => Some(Self::FavSeason),
            "BizTypeReserveGame" => Some(Self::ReserveGame),
            _ => None,
        }
    }
}
/// 素材类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaterialBizType {
    ///
    None = 0,
    ///
    Activity = 1,
    ///
    Bgm = 2,
    ///
    Effect = 3,
    ///
    ShootSame = 4,
    ///
    ShootTogether = 5,
    ///
    ActivityIcon = 6,
    ///
    NewBgm = 7,
}
impl MaterialBizType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MaterialBizType::None => "NONE",
            MaterialBizType::Activity => "ACTIVITY",
            MaterialBizType::Bgm => "BGM",
            MaterialBizType::Effect => "EFFECT",
            MaterialBizType::ShootSame => "SHOOT_SAME",
            MaterialBizType::ShootTogether => "SHOOT_TOGETHER",
            MaterialBizType::ActivityIcon => "ACTIVITY_ICON",
            MaterialBizType::NewBgm => "NEW_BGM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "ACTIVITY" => Some(Self::Activity),
            "BGM" => Some(Self::Bgm),
            "EFFECT" => Some(Self::Effect),
            "SHOOT_SAME" => Some(Self::ShootSame),
            "SHOOT_TOGETHER" => Some(Self::ShootTogether),
            "ACTIVITY_ICON" => Some(Self::ActivityIcon),
            "NEW_BGM" => Some(Self::NewBgm),
            _ => None,
        }
    }
}
/// 素材来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaterialSource {
    ///
    Default = 0,
    /// 必剪素材
    Bijian = 1,
}
impl MaterialSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MaterialSource::Default => "DEFAULT",
            MaterialSource::Bijian => "BIJIAN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "BIJIAN" => Some(Self::Bijian),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PageCategory {
    ///
    CommonPage = 0,
    ///
    ActivityPage = 1,
}
impl PageCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PageCategory::CommonPage => "COMMON_PAGE",
            PageCategory::ActivityPage => "ACTIVITY_PAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMMON_PAGE" => Some(Self::CommonPage),
            "ACTIVITY_PAGE" => Some(Self::ActivityPage),
            _ => None,
        }
    }
}
/// 页面类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PageType {
    /// H5页面(Webview)
    H5 = 0,
    /// 原生页面(native)
    Na = 1,
}
impl PageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PageType::H5 => "H5",
            PageType::Na => "NA",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "H5" => Some(Self::H5),
            "NA" => Some(Self::Na),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SelectionType {
    ///
    Longtitle = 0,
    ///
    Shorttitle = 1,
}
impl SelectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SelectionType::Longtitle => "LONGTITLE",
            SelectionType::Shorttitle => "SHORTTITLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LONGTITLE" => Some(Self::Longtitle),
            "SHORTTITLE" => Some(Self::Shorttitle),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UnionType {
    ///
    Ugc = 0,
    ///
    Ogv = 1,
}
impl UnionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UnionType::Ugc => "UGC",
            UnionType::Ogv => "OGV",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UGC" => Some(Self::Ugc),
            "OGV" => Some(Self::Ogv),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod view_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 统一视频信息接口(7.23启用)
    #[derive(Debug, Clone)]
    pub struct ViewClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ViewClient<tonic::transport::Channel> {
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
    impl<T> ViewClient<T>
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
        ) -> ViewClient<InterceptedService<T, F>>
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
            ViewClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn view(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewReq>,
        ) -> std::result::Result<tonic::Response<super::ViewReply>, tonic::Status> {
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
                "/bilibili.app.viewunite.v1.View/View",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.viewunite.v1.View", "View"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn view_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewProgressReq>,
        ) -> std::result::Result<
            tonic::Response<super::ViewProgressReply>,
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
                "/bilibili.app.viewunite.v1.View/ViewProgress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.viewunite.v1.View", "ViewProgress"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
