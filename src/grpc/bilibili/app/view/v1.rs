/// 活动页资源包
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityResource {
    /// mod资源池名称
    #[prost(string, tag = "1")]
    pub mod_pool_name: ::prost::alloc::string::String,
    /// mod资源名称
    #[prost(string, tag = "2")]
    pub mod_resource_name: ::prost::alloc::string::String,
    /// 背景色
    #[prost(string, tag = "3")]
    pub bg_color: ::prost::alloc::string::String,
    /// 选中背景色
    #[prost(string, tag = "4")]
    pub selected_bg_color: ::prost::alloc::string::String,
    /// 文字颜色
    #[prost(string, tag = "5")]
    pub text_color: ::prost::alloc::string::String,
    /// 浅字色
    #[prost(string, tag = "6")]
    pub light_text_color: ::prost::alloc::string::String,
    /// 深字色
    #[prost(string, tag = "7")]
    pub dark_text_color: ::prost::alloc::string::String,
    /// 分割线色
    #[prost(string, tag = "8")]
    pub divider_color: ::prost::alloc::string::String,
}
/// 大型活动合集
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivitySeason {
    /// 稿件信息
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<super::super::archive::v1::Arc>,
    /// 分P信息
    #[prost(message, repeated, tag = "2")]
    pub pages: ::prost::alloc::vec::Vec<ViewPage>,
    /// ("OnwerExt"为源码中拼写错误)
    #[prost(message, optional, tag = "3")]
    pub owner_ext: ::core::option::Option<OnwerExt>,
    /// 稿件用户操作状态
    #[prost(message, optional, tag = "4")]
    pub req_user: ::core::option::Option<ReqUser>,
    /// 充电排行
    #[prost(message, optional, tag = "5")]
    pub elec_rank: ::core::option::Option<ElecRank>,
    /// 历史观看进度
    #[prost(message, optional, tag = "6")]
    pub history: ::core::option::Option<History>,
    /// 稿件bvid
    #[prost(string, tag = "7")]
    pub bvid: ::prost::alloc::string::String,
    /// 获得荣誉信息
    #[prost(message, optional, tag = "8")]
    pub honor: ::core::option::Option<Honor>,
    /// 联合投稿成员列表
    #[prost(message, repeated, tag = "9")]
    pub staff: ::prost::alloc::vec::Vec<Staff>,
    /// UGC视频合集信息
    #[prost(message, optional, tag = "10")]
    pub ugc_season: ::core::option::Option<UgcSeason>,
    /// 播放页定制tab
    #[prost(message, optional, tag = "11")]
    pub tab: ::core::option::Option<Tab>,
    /// 排行榜
    #[prost(message, optional, tag = "12")]
    pub rank: ::core::option::Option<Rank>,
    /// 预约模块
    #[prost(message, optional, tag = "13")]
    pub order: ::core::option::Option<Order>,
    /// 是否支持点踩
    #[prost(bool, tag = "14")]
    pub support_dislike: bool,
    /// 相关推荐(运营配置+AI推荐)
    #[prost(message, optional, tag = "15")]
    pub operation_relate: ::core::option::Option<OperationRelate>,
    /// 活动页资源包
    #[prost(message, optional, tag = "16")]
    pub activity_resource: ::core::option::Option<ActivityResource>,
    /// 短链接
    #[prost(string, tag = "17")]
    pub short_link: ::prost::alloc::string::String,
    /// 标签
    #[prost(message, optional, tag = "18")]
    pub label: ::core::option::Option<Label>,
    /// 不感兴趣原因
    #[prost(message, optional, tag = "19")]
    pub dislike: ::core::option::Option<Dislike>,
    /// 播放图标动画配置档
    #[prost(message, optional, tag = "20")]
    pub player_icon: ::core::option::Option<PlayerIcon>,
    /// 分享副标题(已观看xxx次)
    #[prost(string, tag = "21")]
    pub share_subtitle: ::prost::alloc::string::String,
    /// 广告配置
    #[prost(message, optional, tag = "22")]
    pub cm_config: ::core::option::Option<CmConfig>,
    /// 免流面板定制
    #[prost(message, optional, tag = "23")]
    pub tf_panel_customized: ::core::option::Option<TfPanelCustomized>,
    /// 争议信息
    #[prost(string, tag = "24")]
    pub argue_msg: ::prost::alloc::string::String,
    /// 错误码
    /// DEFAULT:正常 CODE404:视频被UP主删除
    #[prost(enumeration = "ECode", tag = "25")]
    pub ecode: i32,
    /// 404页信息
    #[prost(message, optional, tag = "26")]
    pub custom_config: ::core::option::Option<CustomConfig>,
    /// 评论样式
    #[prost(string, tag = "27")]
    pub badge_url: ::prost::alloc::string::String,
    /// 稿件简介v2
    #[prost(message, repeated, tag = "28")]
    pub desc_v2: ::prost::alloc::vec::Vec<DescV2>,
    ///
    #[prost(message, optional, tag = "29")]
    pub config: ::core::option::Option<Config>,
    ///
    #[prost(message, optional, tag = "30")]
    pub online: ::core::option::Option<Online>,
    ///
    #[prost(message, optional, tag = "31")]
    pub arc_extra: ::core::option::Option<ArcExtra>,
    ///
    #[prost(message, optional, tag = "32")]
    pub reply_preface: ::core::option::Option<ReplyStyle>,
}
/// 点击签订契约-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddContractReq {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// UP主mid
    #[prost(int64, tag = "2")]
    pub up_mid: i64,
    /// 当前页面spm
    #[prost(string, tag = "3")]
    pub spmid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdInfo {
    ///
    #[prost(int64, tag = "1")]
    pub creative_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub creative_type: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub creative_content: ::core::option::Option<CreativeContent>,
    ///
    #[prost(string, tag = "4")]
    pub ad_cb: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub card_type: i32,
    ///
    #[prost(bytes = "vec", tag = "6")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArcExtra {
    ///
    #[prost(string, tag = "1")]
    pub arc_pub_location: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArcsPlayer {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(map = "int64, string", tag = "2")]
    pub player_info: ::std::collections::HashMap<i64, ::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    ///
    #[prost(int32, tag = "1")]
    pub paid: i32,
    ///
    #[prost(int64, tag = "2")]
    pub price: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub msg: ::core::option::Option<AssetMsg>,
    ///
    #[prost(message, optional, tag = "4")]
    pub preview_msg: ::core::option::Option<AssetMsg>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMsg {
    ///
    #[prost(string, tag = "1")]
    pub desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc2: ::prost::alloc::string::String,
}
/// 关注按钮卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attention {
    /// 开始时间
    #[prost(int32, tag = "1")]
    pub start_time: i32,
    /// 结束时间
    #[prost(int32, tag = "2")]
    pub end_time: i32,
    /// 位置x坐标
    #[prost(double, tag = "3")]
    pub pos_x: f64,
    /// 位置y坐标
    #[prost(double, tag = "4")]
    pub pos_y: f64,
}
/// 音频稿件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audio {
    /// 音频标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 音频封面url
    #[prost(string, tag = "2")]
    pub cover_url: ::prost::alloc::string::String,
    /// 音频auid
    #[prost(int64, tag = "3")]
    pub song_id: i64,
    /// 音频播放量
    #[prost(int64, tag = "4")]
    pub play_count: i64,
    /// 音频评论数
    #[prost(int64, tag = "5")]
    pub reply_count: i64,
    /// 音频作者UID
    #[prost(int64, tag = "6")]
    pub upper_id: i64,
    /// 进入按钮文案
    #[prost(string, tag = "7")]
    pub entrance: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub song_attr: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeStyle {
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
/// 视频引用的bgm音频
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bgm {
    /// 音频auid
    #[prost(int64, tag = "1")]
    pub sid: i64,
    /// 音频作者mid
    #[prost(int64, tag = "2")]
    pub mid: i64,
    /// 音频标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 音频作者昵称
    #[prost(string, tag = "4")]
    pub author: ::prost::alloc::string::String,
    /// bgm页面url
    #[prost(string, tag = "5")]
    pub jump_url: ::prost::alloc::string::String,
    /// 音频封面url
    #[prost(string, tag = "6")]
    pub cover: ::prost::alloc::string::String,
}
/// 收藏合集参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizFavSeasonParam {
    /// 合集id
    #[prost(int64, tag = "1")]
    pub season_id: i64,
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
    /// 链接
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// 预约活动参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizReserveActivityParam {
    /// 活动id
    #[prost(int64, tag = "1")]
    pub activity_id: i64,
    /// 场景
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    /// 类型
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// 资源id
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
    /// 游戏id
    #[prost(int64, tag = "1")]
    pub game_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    /// 按钮文案
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonStyle {
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
    pub jump_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuzzwordConfig {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub schema: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub source: i32,
    ///
    #[prost(int64, tag = "4")]
    pub start: i64,
    ///
    #[prost(int64, tag = "5")]
    pub end: i64,
    ///
    #[prost(bool, tag = "6")]
    pub follow_control: bool,
    ///
    #[prost(int64, tag = "7")]
    pub id: i64,
    ///
    #[prost(int64, tag = "8")]
    pub buzzword_id: i64,
    ///
    #[prost(int32, tag = "9")]
    pub schema_type: i32,
    ///
    #[prost(string, tag = "10")]
    pub picture: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheViewReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<super::super::archive::v1::Arc>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub pages: ::prost::alloc::vec::Vec<ViewPage>,
    ///
    #[prost(message, optional, tag = "3")]
    pub owner_ext: ::core::option::Option<OnwerExt>,
    ///
    #[prost(message, optional, tag = "4")]
    pub req_user: ::core::option::Option<ReqUser>,
    ///
    #[prost(message, optional, tag = "5")]
    pub season: ::core::option::Option<Season>,
    ///
    #[prost(message, optional, tag = "6")]
    pub elec_rank: ::core::option::Option<ElecRank>,
    ///
    #[prost(message, optional, tag = "7")]
    pub history: ::core::option::Option<History>,
    ///
    #[prost(message, optional, tag = "8")]
    pub dislike: ::core::option::Option<Dislike>,
    ///
    #[prost(message, optional, tag = "9")]
    pub player_icon: ::core::option::Option<PlayerIcon>,
    ///
    #[prost(string, tag = "10")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub short_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub share_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "13")]
    pub tf_panel_customized: ::core::option::Option<TfPanelCustomized>,
    ///
    #[prost(message, optional, tag = "14")]
    pub online: ::core::option::Option<Online>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheViewReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub trackid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub ad_extra: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub from_spmid: ::prost::alloc::string::String,
}
/// Chronos灰度管理
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chronos {
    /// 资源包md5
    #[prost(string, tag = "1")]
    pub md5: ::prost::alloc::string::String,
    /// 资源包
    #[prost(string, tag = "2")]
    pub file: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sign: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChronosPkgReq {
    ///
    #[prost(string, tag = "1")]
    pub service_key: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub engine_version: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub message_protocol: ::prost::alloc::string::String,
}
/// 点击大型活动页预约-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickActivitySeasonReq {
    /// 预约类型
    #[prost(enumeration = "BizType", tag = "1")]
    pub order_type: i32,
    /// 当前页面spm
    #[prost(string, tag = "2")]
    pub spmid: ::prost::alloc::string::String,
    /// 操作
    /// 0:操作 1:取消操作
    #[prost(int64, tag = "5")]
    pub action: i64,
    /// 业务参数
    #[prost(oneof = "click_activity_season_req::OrderParam", tags = "3, 4")]
    pub order_param: ::core::option::Option<click_activity_season_req::OrderParam>,
}
/// Nested message and enum types in `ClickActivitySeasonReq`.
pub mod click_activity_season_req {
    /// 业务参数
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OrderParam {
        /// 预约活动参数
        #[prost(message, tag = "3")]
        Reserve(super::BizReserveActivityParam),
        /// 收藏合集参数
        #[prost(message, tag = "4")]
        FavSeason(super::BizFavSeasonParam),
    }
}
/// 点击播放器卡片-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickPlayerCardReply {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// 点击播放器卡片-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickPlayerCardReq {
    /// 卡片id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 稿件avid
    #[prost(int64, tag = "2")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "3")]
    pub cid: i64,
    /// 操作
    /// 0:操作 1:取消操作
    #[prost(int64, tag = "4")]
    pub action: i64,
    /// 当前页面spm
    #[prost(string, tag = "5")]
    pub spmid: ::prost::alloc::string::String,
}
/// 广告
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cm {
    /// 广告数据(需解包)
    #[prost(message, optional, tag = "1")]
    pub source_content: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
/// 广告配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmConfig {
    /// 广告配置数据(需要二次解包)
    #[prost(message, optional, tag = "1")]
    pub ads_control: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CmIpad {
    ///
    #[prost(message, optional, tag = "1")]
    pub cm: ::core::option::Option<Cm>,
    ///
    #[prost(message, optional, tag = "2")]
    pub author: ::core::option::Option<super::super::archive::v1::Author>,
    ///
    #[prost(message, optional, tag = "3")]
    pub stat: ::core::option::Option<super::super::archive::v1::Stat>,
    ///
    #[prost(int64, tag = "4")]
    pub duration: i64,
    ///
    #[prost(int64, tag = "5")]
    pub aid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinCustom {
    ///
    #[prost(string, tag = "1")]
    pub toast: ::prost::alloc::string::String,
}
/// 互动弹幕条目信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDm {
    /// 弹幕id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 对象视频cid
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 发送者mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    /// 互动弹幕指令
    #[prost(string, tag = "4")]
    pub command: ::prost::alloc::string::String,
    /// 互动弹幕正文
    #[prost(string, tag = "5")]
    pub content: ::prost::alloc::string::String,
    /// 出现时间
    #[prost(int32, tag = "6")]
    pub progress: i32,
    /// 创建时间
    #[prost(string, tag = "7")]
    pub ctime: ::prost::alloc::string::String,
    /// 发布时间
    #[prost(string, tag = "8")]
    pub mtime: ::prost::alloc::string::String,
    /// 扩展json数据
    #[prost(string, tag = "9")]
    pub extra: ::prost::alloc::string::String,
    /// 弹幕id str类型
    #[prost(string, tag = "10")]
    pub id_str: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// 下方推荐项标题
    #[prost(string, tag = "1")]
    pub relates_title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub relates_style: i32,
    ///
    #[prost(int32, tag = "3")]
    pub relate_gif_exp: i32,
    ///
    #[prost(int32, tag = "4")]
    pub end_page_half: i32,
    ///
    #[prost(int32, tag = "5")]
    pub end_page_full: i32,
    /// 退出是否自动小窗
    #[prost(bool, tag = "6")]
    pub auto_swindow: bool,
    ///
    #[prost(bool, tag = "7")]
    pub popup_info: bool,
    ///
    #[prost(string, tag = "8")]
    pub abtest_small_window: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub rec_three_point_style: i32,
    ///
    #[prost(bool, tag = "10")]
    pub is_absolute_time: bool,
    ///
    #[prost(bool, tag = "11")]
    pub new_swindow: bool,
    ///
    #[prost(bool, tag = "12")]
    pub relates_biserial: bool,
    ///
    #[prost(message, optional, tag = "13")]
    pub listener_conf: ::core::option::Option<ListenerConfig>,
    ///
    #[prost(string, tag = "14")]
    pub relates_feed_style: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "15")]
    pub relates_feed_popup: bool,
    ///
    #[prost(bool, tag = "16")]
    pub relates_has_next: bool,
    ///
    #[prost(int32, tag = "17")]
    pub local_play: i32,
    ///
    #[prost(bool, tag = "18")]
    pub play_story: bool,
    ///
    #[prost(bool, tag = "19")]
    pub arc_play_story: bool,
    ///
    #[prost(string, tag = "20")]
    pub story_icon: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "21")]
    pub landscape_story: bool,
    ///
    #[prost(bool, tag = "22")]
    pub arc_landscape_story: bool,
    ///
    #[prost(string, tag = "23")]
    pub landscape_icon: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "24")]
    pub show_listen_button: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousPlayReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub relates: ::prost::alloc::vec::Vec<Relate>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousPlayReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub trackid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub autoplay: i32,
    ///
    #[prost(message, optional, tag = "7")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "8")]
    pub device_type: i64,
    ///
    #[prost(string, tag = "9")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "10")]
    pub display_id: i64,
}
/// 契约卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCard {
    /// 需要触发的播放进度百分比
    #[prost(float, tag = "1")]
    pub display_progress: f32,
    /// 触发位置的前后误差(单位ms)
    #[prost(int64, tag = "2")]
    pub display_accuracy: i64,
    /// 展示持续时间(单位ms)
    #[prost(int64, tag = "3")]
    pub display_duration: i64,
    /// 弹出模式
    /// 0: 原有模式 1: 半屏弹出 2: 全屏、半屏均弹出
    #[prost(int32, tag = "4")]
    pub show_mode: i32,
    /// 提示页面
    /// 0: 原有页面 1: 6.23版本新页面
    #[prost(int32, tag = "5")]
    pub page_type: i32,
    /// UP主信息
    #[prost(message, optional, tag = "6")]
    pub upper: ::core::option::Option<UpperInfos>,
    ///
    #[prost(int32, tag = "7")]
    pub is_follow_display: i32,
    ///
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
    ///
    #[prost(bool, tag = "12")]
    pub play_display_switch: bool,
}
///
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
pub struct CreativeContent {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub button_title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub video_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub username: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub image_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub image_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub log_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub log_md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub click_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub show_url: ::prost::alloc::string::String,
}
/// 404页信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConfig {
    /// 重定向页面url
    #[prost(string, tag = "1")]
    pub redirect_url: ::prost::alloc::string::String,
}
/// 特殊稿件简介
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescV2 {
    /// 文本内容
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 文本类型
    #[prost(enumeration = "DescType", tag = "2")]
    pub r#type: i32,
    /// 点击跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 资源ID
    #[prost(int64, tag = "4")]
    pub rid: i64,
}
/// 不喜欢原因
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dislike {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    /// 原因项列表
    #[prost(message, repeated, tag = "3")]
    pub reasons: ::prost::alloc::vec::Vec<DislikeReasons>,
}
/// 不喜欢原因项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DislikeReasons {
    /// 类型
    /// 1:全部类型 3:TAG 4:UP主
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 相关UP主mid
    #[prost(int64, tag = "2")]
    pub mid: i64,
    /// 相关分区tid
    #[prost(int32, tag = "3")]
    pub rid: i32,
    /// 相关TAG id
    #[prost(int64, tag = "4")]
    pub tag_id: i64,
    /// 相关名称
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// 分P弹幕信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dm {
    /// 分P是否关闭弹幕
    /// 0:正常 1:关闭
    #[prost(bool, tag = "1")]
    pub closed: bool,
    ///
    #[prost(bool, tag = "2")]
    pub real_name: bool,
    /// 分P弹幕总数
    #[prost(int64, tag = "3")]
    pub count: i64,
}
/// 充电排行信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElecRank {
    /// 充电排行列表
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<ElecRankItem>,
    /// 充电用户数
    #[prost(int64, tag = "2")]
    pub count: i64,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
/// 充电用户信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElecRankItem {
    /// 用户头像url
    #[prost(string, tag = "1")]
    pub avatar: ::prost::alloc::string::String,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub nickname: ::prost::alloc::string::String,
    /// 充电留言
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// 用户mid
    #[prost(int64, tag = "4")]
    pub mid: i64,
}
/// 视频合集单话信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Episode {
    /// 合集单话id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 稿件avid
    #[prost(int64, tag = "2")]
    pub aid: i64,
    /// 视频1P cid
    #[prost(int64, tag = "3")]
    pub cid: i64,
    /// 稿件标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 稿件封面url
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 投稿时间显示文案
    #[prost(string, tag = "6")]
    pub cover_right_text: ::prost::alloc::string::String,
    /// 视频分P信息
    #[prost(message, optional, tag = "7")]
    pub page: ::core::option::Option<super::super::archive::v1::Page>,
    /// 视频状态数
    #[prost(message, optional, tag = "8")]
    pub stat: ::core::option::Option<super::super::archive::v1::Stat>,
    /// 稿件bvid
    #[prost(string, tag = "9")]
    pub bvid: ::prost::alloc::string::String,
    /// 稿件UP主信息
    #[prost(message, optional, tag = "10")]
    pub author: ::core::option::Option<super::super::archive::v1::Author>,
    ///
    #[prost(string, tag = "11")]
    pub author_desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "12")]
    pub badge_style: ::core::option::Option<BadgeStyle>,
    ///
    #[prost(bool, tag = "13")]
    pub need_pay: bool,
    ///
    #[prost(bool, tag = "14")]
    pub episode_pay: bool,
    ///
    #[prost(bool, tag = "15")]
    pub free_watch: bool,
    ///
    #[prost(string, tag = "16")]
    pub first_frame: ::prost::alloc::string::String,
}
/// 播放器卡片曝光-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExposePlayerCardReq {
    /// 卡片类型
    #[prost(enumeration = "PlayerCardType", tag = "1")]
    pub card_type: i32,
    /// 稿件avid
    #[prost(int64, tag = "2")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "3")]
    pub cid: i64,
    /// 当前页面spm
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedViewItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub view: ::core::option::Option<ViewReply>,
    ///
    #[prost(string, tag = "2")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub track_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedViewReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<FeedViewItem>,
    ///
    #[prost(bool, tag = "2")]
    pub has_next: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedViewReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
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
    #[prost(message, optional, tag = "6")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "7")]
    pub display_id: i64,
    ///
    #[prost(string, tag = "8")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub page_version: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub from_track_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArcsPlayerReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub arcs_player: ::prost::alloc::vec::Vec<ArcsPlayer>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetArcsPlayerReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub play_avs: ::prost::alloc::vec::Vec<PlayAv>,
    ///
    #[prost(message, optional, tag = "2")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsInfo {
    ///
    #[prost(string, tag = "1")]
    pub goods_id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub category: i32,
    ///
    #[prost(int64, tag = "3")]
    pub goods_price: i64,
    ///
    #[prost(enumeration = "PayState", tag = "4")]
    pub pay_state: i32,
    ///
    #[prost(string, tag = "5")]
    pub goods_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub price_fmt: ::prost::alloc::string::String,
}
/// 稿件观看进度
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct History {
    /// 播放进度分P cid
    #[prost(int64, tag = "1")]
    pub cid: i64,
    /// 播放进度时间
    /// 0:未观看 -1:已看完 正整数:播放时间进度
    #[prost(int64, tag = "2")]
    pub progress: i64,
}
/// 稿件获得荣誉信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Honor {
    /// 荣誉栏图标url
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 荣誉栏图标url 夜间模式
    #[prost(string, tag = "2")]
    pub icon_night: ::prost::alloc::string::String,
    /// 荣誉文案
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// 荣誉副文案
    #[prost(string, tag = "4")]
    pub text_extra: ::prost::alloc::string::String,
    /// 标题颜色
    #[prost(string, tag = "5")]
    pub text_color: ::prost::alloc::string::String,
    /// 标题颜色 夜间模式
    #[prost(string, tag = "6")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 背景颜色
    #[prost(string, tag = "7")]
    pub bg_color: ::prost::alloc::string::String,
    /// 背景颜色 夜间模式
    #[prost(string, tag = "8")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "9")]
    pub url: ::prost::alloc::string::String,
    /// 跳转角标文案
    #[prost(string, tag = "10")]
    pub url_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IconData {
    ///
    #[prost(string, tag = "1")]
    pub meta_json: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub sprits_img: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interaction {
    ///
    #[prost(message, optional, tag = "1")]
    pub history_node: ::core::option::Option<Node>,
    ///
    #[prost(int64, tag = "2")]
    pub graph_version: i64,
    ///
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub evaluation: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub mark: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon_night: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub icon_width: i64,
    ///
    #[prost(int64, tag = "6")]
    pub icon_height: i64,
    ///
    #[prost(string, tag = "7")]
    pub lottie: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub lottie_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeAnimation {
    ///
    #[prost(string, tag = "1")]
    pub like_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub liked_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub like_animation: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeCustom {
    ///
    #[prost(bool, tag = "1")]
    pub like_switch: bool,
    ///
    #[prost(int64, tag = "2")]
    pub full_to_half_progress: i64,
    ///
    #[prost(int64, tag = "3")]
    pub non_full_progress: i64,
    ///
    #[prost(int64, tag = "4")]
    pub update_count: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerConfig {
    ///
    #[prost(int64, tag = "1")]
    pub jump_style: i64,
    ///
    #[prost(message, optional, tag = "2")]
    pub guide_bar: ::core::option::Option<ListenerGuideBar>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListenerGuideBar {
    ///
    #[prost(int64, tag = "1")]
    pub show_strategy: i64,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub btn_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub show_time: i64,
    ///
    #[prost(int64, tag = "6")]
    pub background_time: i64,
}
/// 直播信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Live {
    /// 主播UID
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 直播间id
    #[prost(int64, tag = "2")]
    pub roomid: i64,
    /// 直播间url
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub endpage_uri: ::prost::alloc::string::String,
}
/// 直播预约信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveOrderInfo {
    /// 预约id
    #[prost(int64, tag = "1")]
    pub sid: i64,
    /// 预约条文案
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    /// 直播开始时间
    #[prost(int64, tag = "3")]
    pub live_plan_start_time: i64,
    /// 是否预约
    #[prost(bool, tag = "4")]
    pub is_follow: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterialLeft {
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
    #[prost(string, tag = "4")]
    pub left_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub operational_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub static_icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterialRes {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "MaterialSource", tag = "4")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub bg_pic: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub jump_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    ///
    #[prost(int64, tag = "1")]
    pub node_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
}
/// 空回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notice {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
/// 认证信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    /// 认证类型
    /// 0:个人认证 1:官方认证
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 认证名称
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Online {
    ///
    #[prost(bool, tag = "1")]
    pub online_show: bool,
    ///
    #[prost(string, tag = "2")]
    pub player_online_logo: ::prost::alloc::string::String,
}
/// UP主扩展信息 ("OnwerExt"为源码中拼写错误)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnwerExt {
    /// 认证信息
    #[prost(message, optional, tag = "1")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    /// 直播信息
    #[prost(message, optional, tag = "2")]
    pub live: ::core::option::Option<Live>,
    /// 会员信息
    #[prost(message, optional, tag = "3")]
    pub vip: ::core::option::Option<Vip>,
    ///
    #[prost(int64, repeated, tag = "4")]
    pub assists: ::prost::alloc::vec::Vec<i64>,
    /// 粉丝数
    #[prost(int64, tag = "5")]
    pub fans: i64,
    /// 总投稿数
    #[prost(string, tag = "6")]
    pub arc_count: ::prost::alloc::string::String,
}
/// 老运营卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCard {
    /// 开始时间(单位为秒)
    #[prost(int32, tag = "1")]
    pub start_time: i32,
    /// 结束时间(单位为秒)
    #[prost(int32, tag = "2")]
    pub end_time: i32,
    /// 图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 按钮文案
    #[prost(string, tag = "5")]
    pub button_text: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "6")]
    pub url: ::prost::alloc::string::String,
    /// 内容描述
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
}
/// 内嵌操作按钮卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCardNew {
    /// 卡片id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 开始时间
    #[prost(int32, tag = "2")]
    pub from: i32,
    /// 结束时间
    #[prost(int32, tag = "3")]
    pub to: i32,
    /// 用户操作态
    /// true已操作 false未操作
    #[prost(bool, tag = "4")]
    pub status: bool,
    /// 卡片类型
    #[prost(enumeration = "OperationCardType", tag = "5")]
    pub card_type: i32,
    ///
    #[prost(enumeration = "BizType", tag = "8")]
    pub biz_type: i32,
    /// 卡片渲染
    #[prost(oneof = "operation_card_new::Render", tags = "6, 7")]
    pub render: ::core::option::Option<operation_card_new::Render>,
    ///
    #[prost(oneof = "operation_card_new::Param", tags = "9, 10, 11, 12")]
    pub param: ::core::option::Option<operation_card_new::Param>,
}
/// Nested message and enum types in `OperationCardNew`.
pub mod operation_card_new {
    /// 卡片渲染
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Render {
        /// 标准卡
        #[prost(message, tag = "6")]
        Standard(super::StandardCard),
        /// 老运营卡片(原B剪跳转卡)
        #[prost(message, tag = "7")]
        Skip(super::OperationCard),
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Param {
        /// 追番追剧参数
        #[prost(message, tag = "9")]
        Follow(super::BizFollowVideoParam),
        /// 预约活动参数
        #[prost(message, tag = "10")]
        Reserve(super::BizReserveActivityParam),
        /// 跳转参数
        #[prost(message, tag = "11")]
        Jump(super::BizJumpLinkParam),
        /// 预约游戏参数
        #[prost(message, tag = "12")]
        Game(super::BizReserveGameParam),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCardV2 {
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
    #[prost(int32, tag = "5")]
    pub biz_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub content: ::core::option::Option<OperationCardV2Content>,
    ///
    #[prost(oneof = "operation_card_v2::Param", tags = "7, 8, 9, 10")]
    pub param: ::core::option::Option<operation_card_v2::Param>,
}
/// Nested message and enum types in `OperationCardV2`.
pub mod operation_card_v2 {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Param {
        ///
        #[prost(message, tag = "7")]
        BizFollowVideoParam(super::BizFollowVideoParam),
        ///
        #[prost(message, tag = "8")]
        BizReserveActivityParam(super::BizReserveActivityParam),
        ///
        #[prost(message, tag = "9")]
        BizJumpLinkParam(super::BizJumpLinkParam),
        ///
        #[prost(message, tag = "10")]
        BizReserveGameParam(super::BizReserveGameParam),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationCardV2Content {
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
/// 相关推荐(运营配置+AI推荐)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationRelate {
    /// 模块标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 相关推荐模块内容
    #[prost(message, repeated, tag = "2")]
    pub relate_item: ::prost::alloc::vec::Vec<RelateItem>,
    /// AI相关推荐
    #[prost(message, repeated, tag = "3")]
    pub ai_relate_item: ::prost::alloc::vec::Vec<Relate>,
}
/// 预约模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// 用户操作态
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// 模块标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 按钮文字 未操作
    #[prost(string, tag = "3")]
    pub button_title: ::prost::alloc::string::String,
    /// 按钮文字 已操作
    #[prost(string, tag = "4")]
    pub button_selected_title: ::prost::alloc::string::String,
    /// 合集播放数
    #[prost(int64, tag = "5")]
    pub season_stat_view: i64,
    /// 合集弹幕数
    #[prost(int64, tag = "6")]
    pub season_stat_danmaku: i64,
    /// 预约类型(点击时透传，直播开始前预约活动，直播开始后收藏合集)
    #[prost(enumeration = "BizType", tag = "7")]
    pub order_type: i32,
    /// 合集简介
    #[prost(string, tag = "10")]
    pub intro: ::prost::alloc::string::String,
    /// 预约业务参数
    #[prost(oneof = "order::OrderParam", tags = "8, 9")]
    pub order_param: ::core::option::Option<order::OrderParam>,
}
/// Nested message and enum types in `Order`.
pub mod order {
    /// 预约业务参数
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OrderParam {
        /// 预约活动参数
        #[prost(message, tag = "8")]
        Reserve(super::BizReserveActivityParam),
        /// 收藏合集参数
        #[prost(message, tag = "9")]
        FavSeason(super::BizFavSeasonParam),
    }
}
/// 游戏礼包信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackInfo {
    /// 礼包标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 礼包页uri
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAv {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
}
/// 进度条动画配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerIcon {
    /// 拖动动画配置档url
    #[prost(string, tag = "1")]
    pub url1: ::prost::alloc::string::String,
    /// 拖动动画配置档hash
    #[prost(string, tag = "2")]
    pub hash1: ::prost::alloc::string::String,
    /// 松手动画配置档url
    #[prost(string, tag = "3")]
    pub url2: ::prost::alloc::string::String,
    /// 松手动画配置档hash
    #[prost(string, tag = "4")]
    pub hash2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub drag_left_png: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub middle_png: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub drag_right_png: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub drag_data: ::core::option::Option<IconData>,
    ///
    #[prost(message, optional, tag = "9")]
    pub nodrag_data: ::core::option::Option<IconData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRelatesReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Relate>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRelatesReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
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
    #[prost(message, optional, tag = "6")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(string, tag = "7")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub from_track_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointMaterial {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub material_source: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerIconStyle {
    ///
    #[prost(string, tag = "1")]
    pub icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_night_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub icon_width: i64,
    ///
    #[prost(int64, tag = "4")]
    pub icon_height: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Premiere {
    ///
    #[prost(enumeration = "PremiereState", tag = "1")]
    pub premiere_state: i32,
    ///
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    ///
    #[prost(int64, tag = "3")]
    pub service_time: i64,
    ///
    #[prost(int64, tag = "4")]
    pub room_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereArchiveReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub premiere: ::core::option::Option<Premiere>,
    ///
    #[prost(bool, tag = "2")]
    pub risk_status: bool,
    ///
    #[prost(string, tag = "3")]
    pub risk_reason: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereArchiveReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereReserve {
    ///
    #[prost(int64, tag = "1")]
    pub reserve_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub count: i64,
    ///
    #[prost(bool, tag = "3")]
    pub is_follow: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereResource {
    ///
    #[prost(message, optional, tag = "1")]
    pub premiere: ::core::option::Option<Premiere>,
    ///
    #[prost(message, optional, tag = "2")]
    pub reserve: ::core::option::Option<PremiereReserve>,
    ///
    #[prost(message, optional, tag = "3")]
    pub text: ::core::option::Option<PremiereText>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereText {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub online_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub online_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub online_icon_dark: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub intro_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub intro_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub guidance_pulldown: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub guidance_entry: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub intro_icon_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullClientAction {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub pull_action: bool,
    ///
    #[prost(string, tag = "3")]
    pub params: ::prost::alloc::string::String,
}
/// 排行榜
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rank {
    /// 排行榜icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 排行榜icon 夜间模式
    #[prost(string, tag = "2")]
    pub icon_night: ::prost::alloc::string::String,
    /// 排行榜文案
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankInfo {
    ///
    #[prost(string, tag = "1")]
    pub icon_url_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_url_day: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bkg_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bkg_day_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub font_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub font_day_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub rank_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub rank_link: ::prost::alloc::string::String,
}
/// 推荐理由样式
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonStyle {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 日间模式文字
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub border_color: ::prost::alloc::string::String,
    /// 夜间模式文字
    #[prost(string, tag = "5")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub bg_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    /// 1:填充 2:描边 3:填充 + 描边 4:背景不填充 + 背景不描边
    #[prost(int32, tag = "8")]
    pub bg_style: i32,
    ///
    #[prost(int32, tag = "9")]
    pub selected: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecDislike {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub sub_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub closed_sub_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub paste_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub closed_paste_text: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub dislike_reason: ::prost::alloc::vec::Vec<DislikeReasons>,
    ///
    #[prost(string, tag = "7")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub closed_toast: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecThreePoint {
    ///
    #[prost(message, optional, tag = "1")]
    pub dislike: ::core::option::Option<RecDislike>,
    ///
    #[prost(message, optional, tag = "2")]
    pub feedback: ::core::option::Option<RecDislike>,
    ///
    #[prost(bool, tag = "3")]
    pub watch_later: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshPage {
    ///
    #[prost(int32, tag = "1")]
    pub refreshable: i32,
    ///
    #[prost(int32, tag = "2")]
    pub refresh_icon: i32,
    ///
    #[prost(string, tag = "3")]
    pub refresh_text: ::prost::alloc::string::String,
    ///
    #[prost(float, tag = "4")]
    pub refresh_show: f32,
}
/// 相关推荐项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relate {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 封面url
    #[prost(string, tag = "2")]
    pub pic: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// UP主信息
    #[prost(message, optional, tag = "4")]
    pub author: ::core::option::Option<super::super::archive::v1::Author>,
    /// 稿件状态数
    #[prost(message, optional, tag = "5")]
    pub stat: ::core::option::Option<super::super::archive::v1::Stat>,
    /// 时长
    #[prost(int64, tag = "6")]
    pub duration: i64,
    /// 跳转类型
    /// special:pgc视频 av:稿件视频 cm:广告 game:游戏
    #[prost(string, tag = "7")]
    pub goto: ::prost::alloc::string::String,
    /// 参数（如av号等）
    #[prost(string, tag = "8")]
    pub param: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub jump_url: ::prost::alloc::string::String,
    /// 评分
    #[prost(double, tag = "11")]
    pub rating: f64,
    ///
    #[prost(string, tag = "12")]
    pub reserve: ::prost::alloc::string::String,
    /// 来源标识
    /// operation:管理员添加
    #[prost(string, tag = "13")]
    pub from: ::prost::alloc::string::String,
    /// 备注
    #[prost(string, tag = "14")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub rcmd_reason: ::prost::alloc::string::String,
    /// 标志文字
    #[prost(string, tag = "16")]
    pub badge: ::prost::alloc::string::String,
    /// 1P cid
    #[prost(int64, tag = "17")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "18")]
    pub season_type: i32,
    ///
    #[prost(int32, tag = "19")]
    pub rating_count: i32,
    /// 标签文案
    #[prost(string, tag = "20")]
    pub tag_name: ::prost::alloc::string::String,
    /// 游戏礼包信息
    #[prost(message, optional, tag = "21")]
    pub pack_info: ::core::option::Option<PackInfo>,
    ///
    #[prost(message, optional, tag = "22")]
    pub notice: ::core::option::Option<Notice>,
    /// 按钮信息
    #[prost(message, optional, tag = "23")]
    pub button: ::core::option::Option<Button>,
    /// spm追踪id
    #[prost(string, tag = "24")]
    pub trackid: ::prost::alloc::string::String,
    /// 游戏卡片新样式
    #[prost(int32, tag = "25")]
    pub new_card: i32,
    /// 推荐理由样式
    #[prost(message, optional, tag = "26")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(string, tag = "27")]
    pub cover_gif: ::prost::alloc::string::String,
    /// 广告
    #[prost(message, optional, tag = "28")]
    pub cm: ::core::option::Option<Cm>,
    /// 游戏卡字段
    /// 0:下载 1:预约(跳过详情) 2:预约 3:测试 4:测试+预约 5:跳过详情页
    #[prost(int64, tag = "29")]
    pub reserve_status: i64,
    ///
    #[prost(string, tag = "30")]
    pub rcmd_reason_extra: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "31")]
    pub rec_three_point: ::core::option::Option<RecThreePoint>,
    ///
    #[prost(string, tag = "32")]
    pub unique_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "33")]
    pub material_id: i64,
    ///
    #[prost(int64, tag = "34")]
    pub from_source_type: i64,
    ///
    #[prost(string, tag = "35")]
    pub from_source_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "36")]
    pub dimension: ::core::option::Option<super::super::archive::v1::Dimension>,
    ///
    #[prost(string, tag = "37")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "38")]
    pub badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "39")]
    pub power_icon_style: ::core::option::Option<PowerIconStyle>,
    ///
    #[prost(string, tag = "40")]
    pub reserve_status_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "41")]
    pub dislike_report_data: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "42")]
    pub rank_info_game: ::core::option::Option<RankInfo>,
    ///
    #[prost(string, tag = "43")]
    pub first_frame: ::prost::alloc::string::String,
}
/// 相关推荐内容
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelateItem {
    /// 跳链
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// 封面
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
/// 播放页推荐IFS-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatesFeedReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Relate>,
    ///
    #[prost(bool, tag = "2")]
    pub has_next: bool,
    ///
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<
        super::super::super::pagination::PaginationReply,
    >,
}
/// 播放页推荐IFS-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatesFeedReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
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
    #[prost(message, optional, tag = "6")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(int64, tag = "7")]
    pub relates_page: i64,
    ///
    #[prost(string, tag = "8")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub autoplay: i32,
    ///
    #[prost(string, tag = "10")]
    pub from_track_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub biz_extra: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "12")]
    pub device_type: i64,
    ///
    #[prost(string, tag = "13")]
    pub ad_extra: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "14")]
    pub pagination: ::core::option::Option<super::super::super::pagination::Pagination>,
    ///
    #[prost(int32, tag = "15")]
    pub refresh_num: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelateTab {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyStyle {
    ///
    #[prost(string, tag = "1")]
    pub badge_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub badge_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub badge_type: i64,
}
/// 用户操作状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqUser {
    /// 用户是否关注UP
    #[prost(int32, tag = "1")]
    pub attention: i32,
    /// UP是否关注用户
    #[prost(int32, tag = "2")]
    pub guest_attention: i32,
    /// 是否收藏
    #[prost(int32, tag = "3")]
    pub favorite: i32,
    /// 是否点赞
    #[prost(int32, tag = "4")]
    pub like: i32,
    /// 是否点踩
    #[prost(int32, tag = "5")]
    pub dislike: i32,
    /// 是否投币
    #[prost(int32, tag = "6")]
    pub coin: i32,
    /// 关注等级
    #[prost(int32, tag = "7")]
    pub attention_level: i32,
    /// 是否收藏合集
    #[prost(int32, tag = "8")]
    pub fav_season: i32,
    ///
    #[prost(message, optional, tag = "9")]
    pub elec_plus_btn: ::core::option::Option<Button>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveReply {
    ///
    #[prost(int64, tag = "1")]
    pub reserve_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveReq {
    ///
    #[prost(int64, tag = "1")]
    pub reserve_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub reserve_action: i64,
    ///
    #[prost(int64, tag = "3")]
    pub up_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Restriction {
    ///
    #[prost(bool, tag = "1")]
    pub is_teenagers: bool,
    ///
    #[prost(bool, tag = "2")]
    pub is_lessons: bool,
    ///
    #[prost(bool, tag = "3")]
    pub is_review: bool,
    ///
    #[prost(bool, tag = "4")]
    pub disable_rcmd: bool,
}
/// 剧集信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Season {
    ///
    #[prost(string, tag = "1")]
    pub allow_download: ::prost::alloc::string::String,
    /// 剧集ssid
    #[prost(int64, tag = "2")]
    pub season_id: i64,
    /// 是否重定向跳转
    #[prost(int32, tag = "3")]
    pub is_jump: i32,
    /// 剧集标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 剧集封面url
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 剧集是否完结
    #[prost(int32, tag = "6")]
    pub is_finish: i32,
    /// 最新一话epid
    #[prost(int64, tag = "7")]
    pub newest_ep_id: i64,
    /// 最新一话标题
    #[prost(string, tag = "8")]
    pub newest_ep_index: ::prost::alloc::string::String,
    /// 总集数
    #[prost(int64, tag = "9")]
    pub total_count: i64,
    /// 更新星期日
    #[prost(int32, tag = "10")]
    pub weekday: i32,
    /// 用户追番标志
    #[prost(message, optional, tag = "11")]
    pub user_season: ::core::option::Option<UserSeason>,
    ///
    #[prost(message, optional, tag = "12")]
    pub player: ::core::option::Option<SeasonPlayer>,
    /// 单集页面url
    #[prost(string, tag = "13")]
    pub ogv_playurl: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonActivityRecordReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub activity: ::core::option::Option<UgcSeasonActivity>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonActivityRecordReq {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub activity_id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub action: i32,
    ///
    #[prost(int64, tag = "4")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "5")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "6")]
    pub scene: i64,
    ///
    #[prost(string, tag = "7")]
    pub spmid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonPlayer {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub vid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
    ///
    #[prost(string, tag = "4")]
    pub from: ::prost::alloc::string::String,
}
/// 合集详情页-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonReply {
    /// 合集信息
    #[prost(message, optional, tag = "1")]
    pub season: ::core::option::Option<UgcSeason>,
}
/// 合集详情页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonReq {
    /// 合集id
    #[prost(int64, tag = "1")]
    pub season_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonShow {
    ///
    #[prost(string, tag = "1")]
    pub button_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub join_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub rule_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub checkin_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub checkin_prompt: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonWidgetExposeReply {
    ///
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub activity_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonWidgetExposeReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "3")]
    pub season_id: i64,
    ///
    #[prost(int64, tag = "4")]
    pub activity_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "6")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "7")]
    pub scene: i64,
}
/// 视频合集小节信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Section {
    /// 小节id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 小节标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 小节类型
    /// 0:其他 1:正片
    #[prost(int64, tag = "3")]
    pub r#type: i64,
    /// 单话列表
    #[prost(message, repeated, tag = "4")]
    pub episodes: ::prost::alloc::vec::Vec<Episode>,
}
/// 短视频下载-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortFormVideoDownloadReply {
    /// 是否有下载分享按钮
    #[prost(bool, tag = "1")]
    pub has_download_url: bool,
    /// 下载url
    #[prost(string, tag = "2")]
    pub download_url: ::prost::alloc::string::String,
    /// 文件md5
    #[prost(string, tag = "3")]
    pub md5: ::prost::alloc::string::String,
    /// 文件大小(单位为Byte)
    #[prost(int64, tag = "4")]
    pub size: i64,
    ///
    #[prost(string, tag = "5")]
    pub backup_download_url: ::prost::alloc::string::String,
}
/// 短视频下载-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortFormVideoDownloadReq {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 用户mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    /// 设备buvid
    #[prost(string, tag = "4")]
    pub buvid: ::prost::alloc::string::String,
    /// 移动端包类型
    #[prost(string, tag = "5")]
    pub mobi_app: ::prost::alloc::string::String,
    /// 移动端版本号
    #[prost(int64, tag = "6")]
    pub build: i64,
    /// 运行设备
    #[prost(string, tag = "7")]
    pub device: ::prost::alloc::string::String,
    /// 平台
    #[prost(string, tag = "8")]
    pub platform: ::prost::alloc::string::String,
    /// 当前页面spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub restriction: ::core::option::Option<Restriction>,
    ///
    #[prost(string, tag = "11")]
    pub tf_isp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialCell {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cell_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub cell_bgcolor: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub cell_bgcolor_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub page_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub jump_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub end_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub end_icon_night: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "15")]
    pub notes_count: i64,
}
/// 合作成员信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Staff {
    /// 成员mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 成员角色
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 成员头像url
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// 成员昵称
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// 成员官方信息
    #[prost(message, optional, tag = "5")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    /// 成员会员信息
    #[prost(message, optional, tag = "6")]
    pub vip: ::core::option::Option<Vip>,
    /// 是否关注该成员
    #[prost(int32, tag = "7")]
    pub attention: i32,
    ///
    #[prost(int32, tag = "8")]
    pub label_style: i32,
}
/// 标准卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardCard {
    /// 卡片文案
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 按钮文字 未操作
    #[prost(string, tag = "2")]
    pub button_title: ::prost::alloc::string::String,
    /// 按钮文字 已操作
    #[prost(string, tag = "3")]
    pub button_selected_title: ::prost::alloc::string::String,
    /// 已操作态是否显示
    #[prost(bool, tag = "4")]
    pub show_selected: bool,
}
/// 免流子面板定制化配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubTfPanel {
    /// 右侧按钮素材
    #[prost(string, tag = "1")]
    pub right_btn_img: ::prost::alloc::string::String,
    /// 右侧按钮文案
    #[prost(string, tag = "2")]
    pub right_btn_text: ::prost::alloc::string::String,
    /// 右侧按钮字体颜色
    #[prost(string, tag = "3")]
    pub right_btn_text_color: ::prost::alloc::string::String,
    /// 右侧按钮跳转链接
    #[prost(string, tag = "4")]
    pub right_btn_link: ::prost::alloc::string::String,
    /// 中心主文案内容
    #[prost(string, tag = "5")]
    pub main_label: ::prost::alloc::string::String,
    /// 运营商
    #[prost(string, tag = "6")]
    pub operator: ::prost::alloc::string::String,
}
/// TAB
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tab {
    /// 背景图片
    #[prost(string, tag = "1")]
    pub background: ::prost::alloc::string::String,
    /// 跳转类型
    #[prost(enumeration = "TabOtype", tag = "2")]
    pub otype: i32,
    /// 类型id
    #[prost(int64, tag = "3")]
    pub oid: i64,
    /// 跳转url
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    /// 样式
    #[prost(enumeration = "TabStyle", tag = "5")]
    pub style: i32,
    /// 文字
    #[prost(string, tag = "6")]
    pub text: ::prost::alloc::string::String,
    /// 未选中态字色
    #[prost(string, tag = "7")]
    pub text_color: ::prost::alloc::string::String,
    /// 选中态字色
    #[prost(string, tag = "8")]
    pub text_color_selected: ::prost::alloc::string::String,
    /// 图片
    #[prost(string, tag = "9")]
    pub pic: ::prost::alloc::string::String,
    /// 后台配置自增
    #[prost(int64, tag = "10")]
    pub id: i64,
    ///
    #[prost(message, optional, tag = "11")]
    pub ad_tab_info: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
/// TAG信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// TAD id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// TAG名
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub likes: i64,
    ///
    #[prost(int64, tag = "4")]
    pub hates: i64,
    ///
    #[prost(int32, tag = "5")]
    pub liked: i32,
    ///
    #[prost(int32, tag = "6")]
    pub hated: i32,
    /// TAG页面uri
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    /// TAG类型
    /// common:普通 new:话题 act:活动
    #[prost(string, tag = "8")]
    pub tag_type: ::prost::alloc::string::String,
}
/// 免流面板定制
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TfPanelCustomized {
    /// 右侧按钮素材
    #[prost(string, tag = "1")]
    pub right_btn_img: ::prost::alloc::string::String,
    /// 右侧按钮文案
    #[prost(string, tag = "2")]
    pub right_btn_text: ::prost::alloc::string::String,
    /// 右侧按钮字体颜色
    #[prost(string, tag = "3")]
    pub right_btn_text_color: ::prost::alloc::string::String,
    /// 右侧按钮跳转链接
    #[prost(string, tag = "4")]
    pub right_btn_link: ::prost::alloc::string::String,
    /// 中心主文案内容
    #[prost(string, tag = "5")]
    pub main_label: ::prost::alloc::string::String,
    /// 运营商(cm ct cu)
    #[prost(string, tag = "6")]
    pub operator: ::prost::alloc::string::String,
    /// 子面板定制化配置
    #[prost(map = "string, message", tag = "7")]
    pub sub_panel: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SubTfPanel,
    >,
}
/// TAG图标信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TIcon {
    /// TAG图标url
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
}
/// UGC视频合集信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcSeason {
    /// 合集id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 合集标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 合集封面url
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    /// 合集简介
    #[prost(string, tag = "4")]
    pub intro: ::prost::alloc::string::String,
    /// 小节列表
    #[prost(message, repeated, tag = "5")]
    pub sections: ::prost::alloc::vec::Vec<Section>,
    /// 合集状态数
    #[prost(message, optional, tag = "6")]
    pub stat: ::core::option::Option<UgcSeasonStat>,
    /// 标签字色
    #[prost(string, tag = "7")]
    pub label_text: ::prost::alloc::string::String,
    /// 标签背景色
    #[prost(string, tag = "8")]
    pub label_text_color: ::prost::alloc::string::String,
    /// 标签夜间字色
    #[prost(string, tag = "9")]
    pub label_bg_color: ::prost::alloc::string::String,
    /// 标签夜间背景色
    #[prost(string, tag = "10")]
    pub label_text_night_color: ::prost::alloc::string::String,
    /// 右侧描述文案
    #[prost(string, tag = "11")]
    pub label_bg_night_color: ::prost::alloc::string::String,
    /// 按钮文案
    #[prost(string, tag = "12")]
    pub desc_right: ::prost::alloc::string::String,
    /// 分集总数
    #[prost(int64, tag = "13")]
    pub ep_count: i64,
    /// 合集类型
    #[prost(enumeration = "SeasonType", tag = "14")]
    pub season_type: i32,
    ///
    #[prost(bool, tag = "15")]
    pub show_continual_button: bool,
    ///
    #[prost(int64, tag = "16")]
    pub ep_num: i64,
    ///
    #[prost(bool, tag = "17")]
    pub season_pay: bool,
    ///
    #[prost(message, optional, tag = "18")]
    pub goods_info: ::core::option::Option<GoodsInfo>,
    ///
    #[prost(message, optional, tag = "19")]
    pub pay_button: ::core::option::Option<ButtonStyle>,
    ///
    #[prost(string, tag = "20")]
    pub label_text_new: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "21")]
    pub activity: ::core::option::Option<UgcSeasonActivity>,
    ///
    #[prost(string, repeated, tag = "22")]
    pub season_ability: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcSeasonActivity {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub activity_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub intro: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub day_count: i32,
    ///
    #[prost(int32, tag = "7")]
    pub user_count: i32,
    ///
    #[prost(int64, tag = "8")]
    pub join_deadline: i64,
    ///
    #[prost(int64, tag = "9")]
    pub activity_deadline: i64,
    ///
    #[prost(int32, tag = "10")]
    pub checkin_view_time: i32,
    ///
    #[prost(bool, tag = "11")]
    pub new_activity: bool,
    ///
    #[prost(message, optional, tag = "12")]
    pub user_activity: ::core::option::Option<UserActivity>,
    ///
    #[prost(message, optional, tag = "13")]
    pub season_show: ::core::option::Option<SeasonShow>,
}
/// ugc视频合集状态数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UgcSeasonStat {
    /// 合集id
    #[prost(int64, tag = "1")]
    pub season_id: i64,
    /// 观看数
    #[prost(int32, tag = "2")]
    pub view: i32,
    /// 弹幕数
    #[prost(int32, tag = "3")]
    pub danmaku: i32,
    /// 评论数
    #[prost(int32, tag = "4")]
    pub reply: i32,
    /// 收藏数
    #[prost(int32, tag = "5")]
    pub fav: i32,
    /// 投币数
    #[prost(int32, tag = "6")]
    pub coin: i32,
    /// 分享数
    #[prost(int32, tag = "7")]
    pub share: i32,
    /// 当前排名
    #[prost(int32, tag = "8")]
    pub now_rank: i32,
    /// 历史最高排名
    #[prost(int32, tag = "9")]
    pub his_rank: i32,
    /// 总计点赞
    #[prost(int32, tag = "10")]
    pub like: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpAct {
    ///
    #[prost(int64, tag = "1")]
    pub sid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub statement: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub button: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpLikeImg {
    ///
    #[prost(string, tag = "1")]
    pub pre_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub suc_img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub r#type: i64,
}
/// UP主信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpperInfos {
    /// 粉丝数
    #[prost(int64, tag = "1")]
    pub fans_count: i64,
    /// 近半年投稿数
    #[prost(int64, tag = "2")]
    pub arc_count_last_half_year: i64,
    /// 成为UP主时间
    #[prost(int64, tag = "3")]
    pub first_up_dates: i64,
    /// 总播放量
    #[prost(int64, tag = "4")]
    pub total_play_count: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserActivity {
    ///
    #[prost(int32, tag = "1")]
    pub user_state: i32,
    ///
    #[prost(int64, tag = "2")]
    pub last_checkin_date: i64,
    ///
    #[prost(int32, tag = "3")]
    pub checkin_today: i32,
    ///
    #[prost(int32, tag = "4")]
    pub user_day_count: i32,
    ///
    #[prost(int32, tag = "5")]
    pub user_view_time: i32,
    ///
    #[prost(string, tag = "6")]
    pub portrait: ::prost::alloc::string::String,
}
/// 用户装扮信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserGarb {
    /// 点赞动画url
    #[prost(string, tag = "1")]
    pub url_image_ani_cut: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub like_toast: ::prost::alloc::string::String,
}
/// 用户追番标志
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSeason {
    /// 关注状态
    /// 0:未关注 1:已关注
    #[prost(string, tag = "1")]
    pub attention: ::prost::alloc::string::String,
}
/// 视频引导信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoGuide {
    /// 关注按钮卡片
    #[prost(message, repeated, tag = "1")]
    pub attention: ::prost::alloc::vec::Vec<Attention>,
    /// 互动弹幕
    #[prost(message, repeated, tag = "2")]
    pub command_dms: ::prost::alloc::vec::Vec<CommandDm>,
    /// 运营卡片
    #[prost(message, repeated, tag = "3")]
    pub operation_card: ::prost::alloc::vec::Vec<OperationCard>,
    /// 运营卡片新版
    #[prost(message, repeated, tag = "4")]
    pub operation_card_new: ::prost::alloc::vec::Vec<OperationCardNew>,
    /// 契约卡
    #[prost(message, optional, tag = "5")]
    pub contract_card: ::core::option::Option<ContractCard>,
    ///
    #[prost(message, repeated, tag = "6")]
    pub cards_second: ::prost::alloc::vec::Vec<OperationCardV2>,
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
    pub img_y_len: i32,
    ///
    #[prost(int32, tag = "4")]
    pub img_x_size: i32,
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
pub struct ViewMaterial {
    ///
    #[prost(int64, tag = "1")]
    pub oid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub jump_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewMaterialReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub material_res: ::prost::alloc::vec::Vec<MaterialRes>,
    ///
    #[prost(message, optional, tag = "2")]
    pub material_left: ::core::option::Option<MaterialLeft>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewMaterialReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
}
/// 分P信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewPage {
    /// 分P基本信息
    #[prost(message, optional, tag = "1")]
    pub page: ::core::option::Option<super::super::archive::v1::Page>,
    /// 分P对应的音频稿件
    #[prost(message, optional, tag = "2")]
    pub audio: ::core::option::Option<Audio>,
    /// 分P弹幕信息
    #[prost(message, optional, tag = "3")]
    pub dm: ::core::option::Option<Dm>,
    /// 下载文案
    #[prost(string, tag = "4")]
    pub download_title: ::prost::alloc::string::String,
    /// 分P完整标题(视频标题+分P标题)
    #[prost(string, tag = "5")]
    pub download_subtitle: ::prost::alloc::string::String,
}
/// 稿件播放中数据-回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewProgressReply {
    /// 视频引导信息
    #[prost(message, optional, tag = "1")]
    pub video_guide: ::core::option::Option<VideoGuide>,
    /// Chronos灰度管理
    #[prost(message, optional, tag = "2")]
    pub chronos: ::core::option::Option<Chronos>,
    /// 视频快照
    #[prost(message, optional, tag = "3")]
    pub arc_shot: ::core::option::Option<VideoShot>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub points: ::prost::alloc::vec::Vec<VideoPoint>,
    ///
    #[prost(message, optional, tag = "5")]
    pub point_material: ::core::option::Option<PointMaterial>,
    ///
    #[prost(bool, tag = "6")]
    pub point_permanent: bool,
    /// 名词解释列表
    #[prost(message, repeated, tag = "7")]
    pub buzzword_periods: ::prost::alloc::vec::Vec<BuzzwordConfig>,
}
/// 稿件播放中数据-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewProgressReq {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// UP主mid
    #[prost(int64, tag = "3")]
    pub up_mid: i64,
    ///
    #[prost(string, tag = "4")]
    pub engine_version: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub message_protocol: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub service_key: ::prost::alloc::string::String,
}
/// 视频页信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewReply {
    /// 稿件信息
    #[prost(message, optional, tag = "1")]
    pub arc: ::core::option::Option<super::super::archive::v1::Arc>,
    /// 分P信息
    #[prost(message, repeated, tag = "2")]
    pub pages: ::prost::alloc::vec::Vec<ViewPage>,
    /// UP主扩展信息 ("OnwerExt"为源码中拼写错误)
    #[prost(message, optional, tag = "3")]
    pub owner_ext: ::core::option::Option<OnwerExt>,
    /// 稿件用户操作状态
    #[prost(message, optional, tag = "4")]
    pub req_user: ::core::option::Option<ReqUser>,
    /// 稿件TAG
    #[prost(message, repeated, tag = "5")]
    pub tag: ::prost::alloc::vec::Vec<Tag>,
    /// TAG对应的图标
    #[prost(map = "string, message", tag = "6")]
    pub t_icon: ::std::collections::HashMap<::prost::alloc::string::String, TIcon>,
    /// 稿件映射的PGC剧集信息
    #[prost(message, optional, tag = "7")]
    pub season: ::core::option::Option<Season>,
    /// 充电排行
    #[prost(message, optional, tag = "8")]
    pub elec_rank: ::core::option::Option<ElecRank>,
    /// 历史观看进度
    #[prost(message, optional, tag = "9")]
    pub history: ::core::option::Option<History>,
    /// 视频相关推荐列表
    #[prost(message, repeated, tag = "10")]
    pub relates: ::prost::alloc::vec::Vec<Relate>,
    /// 不感兴趣原因
    #[prost(message, optional, tag = "11")]
    pub dislike: ::core::option::Option<Dislike>,
    /// 播放图标动画配置档
    #[prost(message, optional, tag = "12")]
    pub player_icon: ::core::option::Option<PlayerIcon>,
    ///
    #[prost(string, tag = "13")]
    pub vip_active: ::prost::alloc::string::String,
    /// 稿件bvid
    #[prost(string, tag = "14")]
    pub bvid: ::prost::alloc::string::String,
    /// 获得荣誉信息
    #[prost(message, optional, tag = "15")]
    pub honor: ::core::option::Option<Honor>,
    /// 相关推荐顶部tab
    #[prost(message, repeated, tag = "16")]
    pub relate_tab: ::prost::alloc::vec::Vec<RelateTab>,
    /// 参与的活动页面url
    #[prost(string, tag = "17")]
    pub activity_url: ::prost::alloc::string::String,
    /// 稿件引用bgm列表
    #[prost(message, repeated, tag = "18")]
    pub bgm: ::prost::alloc::vec::Vec<Bgm>,
    /// 联合投稿成员列表
    #[prost(message, repeated, tag = "19")]
    pub staff: ::prost::alloc::vec::Vec<Staff>,
    /// 争议信息
    #[prost(string, tag = "20")]
    pub argue_msg: ::prost::alloc::string::String,
    /// 短链接
    #[prost(string, tag = "21")]
    pub short_link: ::prost::alloc::string::String,
    /// 播放实验
    /// 1:相关推荐自动播放
    #[prost(int32, tag = "22")]
    pub play_param: i32,
    /// 标签
    #[prost(message, optional, tag = "23")]
    pub label: ::core::option::Option<Label>,
    /// UGC视频合集信息
    #[prost(message, optional, tag = "24")]
    pub ugc_season: ::core::option::Option<UgcSeason>,
    /// 配置信息
    #[prost(message, optional, tag = "25")]
    pub config: ::core::option::Option<Config>,
    /// 分享副标题(已观看xxx次)
    #[prost(string, tag = "26")]
    pub share_subtitle: ::prost::alloc::string::String,
    /// 互动视频信息
    #[prost(message, optional, tag = "27")]
    pub interaction: ::core::option::Option<Interaction>,
    /// 错误码
    /// DEFAULT:正常 CODE404:视频被UP主删除
    #[prost(enumeration = "ECode", tag = "28")]
    pub ecode: i32,
    /// 404页信息
    #[prost(message, optional, tag = "29")]
    pub custom_config: ::core::option::Option<CustomConfig>,
    /// 广告
    #[prost(message, repeated, tag = "30")]
    pub cms: ::prost::alloc::vec::Vec<Cm>,
    /// 广告配置
    #[prost(message, optional, tag = "31")]
    pub cm_config: ::core::option::Option<CmConfig>,
    /// 播放页定制tab
    #[prost(message, optional, tag = "32")]
    pub tab: ::core::option::Option<Tab>,
    /// 排行榜
    #[prost(message, optional, tag = "33")]
    pub rank: ::core::option::Option<Rank>,
    /// 免流面板定制
    #[prost(message, optional, tag = "34")]
    pub tf_panel_customized: ::core::option::Option<TfPanelCustomized>,
    /// UP主发起活动
    #[prost(message, optional, tag = "35")]
    pub up_act: ::core::option::Option<UpAct>,
    /// 用户装扮
    #[prost(message, optional, tag = "36")]
    pub user_garb: ::core::option::Option<UserGarb>,
    /// 大型活动合集
    #[prost(message, optional, tag = "37")]
    pub activity_season: ::core::option::Option<ActivitySeason>,
    /// 评论样式
    #[prost(string, tag = "38")]
    pub badge_url: ::prost::alloc::string::String,
    /// 直播预约信息
    #[prost(message, optional, tag = "39")]
    pub live_order_info: ::core::option::Option<LiveOrderInfo>,
    /// 稿件简介v2
    #[prost(message, repeated, tag = "40")]
    pub desc_v2: ::prost::alloc::vec::Vec<DescV2>,
    ///
    #[prost(message, optional, tag = "41")]
    pub cm_ipad: ::core::option::Option<CmIpad>,
    ///
    #[prost(message, repeated, tag = "42")]
    pub sticker: ::prost::alloc::vec::Vec<ViewMaterial>,
    ///
    #[prost(message, optional, tag = "43")]
    pub up_like_img: ::core::option::Option<UpLikeImg>,
    ///
    #[prost(message, optional, tag = "44")]
    pub like_custom: ::core::option::Option<LikeCustom>,
    ///
    #[prost(message, repeated, tag = "45")]
    pub desc_tag: ::prost::alloc::vec::Vec<Tag>,
    ///
    #[prost(message, optional, tag = "46")]
    pub special_cell: ::core::option::Option<SpecialCell>,
    ///
    #[prost(message, optional, tag = "47")]
    pub online: ::core::option::Option<Online>,
    ///
    #[prost(message, optional, tag = "48")]
    pub cm_under_player: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    ///
    #[prost(message, repeated, tag = "49")]
    pub video_source: ::prost::alloc::vec::Vec<ViewMaterial>,
    ///
    #[prost(message, repeated, tag = "50")]
    pub special_cell_new: ::prost::alloc::vec::Vec<SpecialCell>,
    ///
    #[prost(message, optional, tag = "51")]
    pub premiere: ::core::option::Option<PremiereResource>,
    ///
    #[prost(bool, tag = "52")]
    pub refresh_special_cell: bool,
    ///
    #[prost(message, optional, tag = "53")]
    pub material_left: ::core::option::Option<MaterialLeft>,
    ///
    #[prost(int64, tag = "54")]
    pub notes_count: i64,
    ///
    #[prost(message, optional, tag = "55")]
    pub pull_action: ::core::option::Option<PullClientAction>,
    ///
    #[prost(message, optional, tag = "56")]
    pub arc_extra: ::core::option::Option<ArcExtra>,
    ///
    #[prost(message, optional, tag = "57")]
    pub pagination: ::core::option::Option<
        super::super::super::pagination::PaginationReply,
    >,
    ///
    #[prost(message, optional, tag = "58")]
    pub like_animation: ::core::option::Option<LikeAnimation>,
    ///
    #[prost(message, optional, tag = "59")]
    pub reply_preface: ::core::option::Option<ReplyStyle>,
    ///
    #[prost(message, optional, tag = "60")]
    pub refresh_page: ::core::option::Option<RefreshPage>,
    ///
    #[prost(message, optional, tag = "61")]
    pub coin_custom: ::core::option::Option<CoinCustom>,
}
/// 视频页详情页-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewReq {
    /// 稿件avid(av/bv任选其一)
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 稿件bvid(av/bv任选其一)
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    /// 来源
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    /// AI trackid
    #[prost(string, tag = "4")]
    pub trackid: ::prost::alloc::string::String,
    /// 广告扩展数据
    #[prost(string, tag = "5")]
    pub ad_extra: ::prost::alloc::string::String,
    /// 清晰度(旧版)
    #[prost(int32, tag = "6")]
    pub qn: i32,
    /// 流版本(旧版)
    #[prost(int32, tag = "7")]
    pub fnver: i32,
    /// 流类型(旧版)
    #[prost(int32, tag = "8")]
    pub fnval: i32,
    /// 是否强制使用域名(旧版)
    #[prost(int32, tag = "9")]
    pub force_host: i32,
    /// 是否允许4K(旧版)
    #[prost(int32, tag = "10")]
    pub fourk: i32,
    /// 当前页面spm
    #[prost(string, tag = "11")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页面spm
    #[prost(string, tag = "12")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "13")]
    pub autoplay: i32,
    /// 视频秒开参数
    #[prost(message, optional, tag = "14")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    ///
    #[prost(string, tag = "15")]
    pub page_version: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub biz_extra: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "17")]
    pub device_type: i64,
    ///
    #[prost(int64, tag = "18")]
    pub relates_page: i64,
    ///
    #[prost(string, tag = "19")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "20")]
    pub in_feed_play: i32,
    ///
    #[prost(string, tag = "21")]
    pub play_mode: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "22")]
    pub pagination: ::core::option::Option<super::super::super::pagination::Pagination>,
    ///
    #[prost(int32, tag = "23")]
    pub refresh: i32,
    ///
    #[prost(int32, tag = "24")]
    pub refresh_num: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewTagReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub special_cell_new: ::prost::alloc::vec::Vec<SpecialCell>,
    ///
    #[prost(message, optional, tag = "2")]
    pub material_left: ::core::option::Option<MaterialLeft>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewTagReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
    ///
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
}
/// 会员信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vip {
    /// 会员类型
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 到期时间
    #[prost(int64, tag = "2")]
    pub due_date: i64,
    ///
    #[prost(string, tag = "3")]
    pub due_remark: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub access_status: i32,
    /// 会员状态
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
/// 会员类型标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    ///
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub label_theme: ::prost::alloc::string::String,
}
/// 业务类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BizType {
    ///
    None = 0,
    /// 追番追剧
    FollowVideo = 1,
    /// 预约活动
    ReserveActivity = 2,
    /// 跳转链接
    JumpLink = 3,
    /// 收藏合集
    FavSeason = 4,
    /// 预约游戏
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
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Category {
    ///
    Unknown = 0,
    ///
    Season = 1,
}
impl Category {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Category::Unknown => "CategoryUnknown",
            Category::Season => "CategorySeason",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CategoryUnknown" => Some(Self::Unknown),
            "CategorySeason" => Some(Self::Season),
            _ => None,
        }
    }
}
/// 枚举-文本类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DescType {
    /// 占位
    Unknown = 0,
    /// 文本
    Text = 1,
    /// @
    At = 2,
}
impl DescType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DescType::Unknown => "DescTypeUnknown",
            DescType::Text => "DescTypeText",
            DescType::At => "DescTypeAt",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DescTypeUnknown" => Some(Self::Unknown),
            "DescTypeText" => Some(Self::Text),
            "DescTypeAt" => Some(Self::At),
            _ => None,
        }
    }
}
/// 错误代码
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECode {
    /// 正常
    Default = 0,
    /// 稿件被UP主删除
    Code404 = 1,
}
impl ECode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ECode::Default => "DEFAULT",
            ECode::Code404 => "CODE404",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "CODE404" => Some(Self::Code404),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaterialSource {
    ///
    Default = 0,
    /// 必剪
    BiJian = 1,
}
impl MaterialSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MaterialSource::Default => "Default",
            MaterialSource::BiJian => "BiJian",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Default" => Some(Self::Default),
            "BiJian" => Some(Self::BiJian),
            _ => None,
        }
    }
}
/// 卡片样式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationCardType {
    ///
    CardTypeNone = 0,
    /// 标准卡
    CardTypeStandard = 1,
    /// 原跳转卡
    CardTypeSkip = 2,
}
impl OperationCardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationCardType::CardTypeNone => "CardTypeNone",
            OperationCardType::CardTypeStandard => "CardTypeStandard",
            OperationCardType::CardTypeSkip => "CardTypeSkip",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CardTypeNone" => Some(Self::CardTypeNone),
            "CardTypeStandard" => Some(Self::CardTypeStandard),
            "CardTypeSkip" => Some(Self::CardTypeSkip),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayState {
    ///
    Unknown = 0,
    ///
    Active = 1,
}
impl PayState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayState::Unknown => "PayStateUnknown",
            PayState::Active => "PayStateActive",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PayStateUnknown" => Some(Self::Unknown),
            "PayStateActive" => Some(Self::Active),
            _ => None,
        }
    }
}
/// 卡片类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerCardType {
    ///
    NoneValue = 0,
    /// 关注卡
    AttentionValue = 1,
    /// 运营卡
    OperationValue = 2,
    /// 契约卡
    ContractValue = 3,
}
impl PlayerCardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerCardType::NoneValue => "PlayerCardTypeNone_VALUE",
            PlayerCardType::AttentionValue => "PlayerCardTypeAttention_VALUE",
            PlayerCardType::OperationValue => "PlayerCardTypeOperation_VALUE",
            PlayerCardType::ContractValue => "PlayerCardTypeContract_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PlayerCardTypeNone_VALUE" => Some(Self::NoneValue),
            "PlayerCardTypeAttention_VALUE" => Some(Self::AttentionValue),
            "PlayerCardTypeOperation_VALUE" => Some(Self::OperationValue),
            "PlayerCardTypeContract_VALUE" => Some(Self::ContractValue),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PremiereState {
    ///
    PremiereNone = 0,
    ///
    PremiereBefore = 1,
    ///
    PremiereIn = 2,
    ///
    PremiereAfter = 3,
}
impl PremiereState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PremiereState::PremiereNone => "premiere_none",
            PremiereState::PremiereBefore => "premiere_before",
            PremiereState::PremiereIn => "premiere_in",
            PremiereState::PremiereAfter => "premiere_after",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "premiere_none" => Some(Self::PremiereNone),
            "premiere_before" => Some(Self::PremiereBefore),
            "premiere_in" => Some(Self::PremiereIn),
            "premiere_after" => Some(Self::PremiereAfter),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeasonType {
    ///
    Unknown = 0,
    ///
    Base = 1,
    ///
    Good = 2,
}
impl SeasonType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SeasonType::Unknown => "Unknown",
            SeasonType::Base => "Base",
            SeasonType::Good => "Good",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Base" => Some(Self::Base),
            "Good" => Some(Self::Good),
            _ => None,
        }
    }
}
/// TAB跳转类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TabOtype {
    /// 未知类型
    UnknownOtype = 0,
    /// url链接
    Url = 1,
    /// native话题活动
    TopicNa = 2,
    /// 广告url
    CmUri = 3,
}
impl TabOtype {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TabOtype::UnknownOtype => "UnknownOtype",
            TabOtype::Url => "URL",
            TabOtype::TopicNa => "TopicNA",
            TabOtype::CmUri => "CmURI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnknownOtype" => Some(Self::UnknownOtype),
            "URL" => Some(Self::Url),
            "TopicNA" => Some(Self::TopicNa),
            "CmURI" => Some(Self::CmUri),
            _ => None,
        }
    }
}
/// TAB样式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TabStyle {
    /// 未知样式
    UnknownStyle = 0,
    /// 文字样式
    Text = 1,
    /// 图片样式
    Pic = 2,
}
impl TabStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TabStyle::UnknownStyle => "UnknownStyle",
            TabStyle::Text => "Text",
            TabStyle::Pic => "Pic",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnknownStyle" => Some(Self::UnknownStyle),
            "Text" => Some(Self::Text),
            "Pic" => Some(Self::Pic),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod view_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
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
        /// 视频页详情页
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
                "/bilibili.app.view.v1.View/View",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "View"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn view_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewTagReq>,
        ) -> std::result::Result<tonic::Response<super::ViewTagReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/ViewTag",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "ViewTag"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn view_material(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewMaterialReq>,
        ) -> std::result::Result<
            tonic::Response<super::ViewMaterialReply>,
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
                "/bilibili.app.view.v1.View/ViewMaterial",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "ViewMaterial"));
            self.inner.unary(req, path, codec).await
        }
        /// 视频播放过程中的数据
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
                "/bilibili.app.view.v1.View/ViewProgress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "ViewProgress"));
            self.inner.unary(req, path, codec).await
        }
        /// 短视频下载
        pub async fn short_form_video_download(
            &mut self,
            request: impl tonic::IntoRequest<super::ShortFormVideoDownloadReq>,
        ) -> std::result::Result<
            tonic::Response<super::ShortFormVideoDownloadReply>,
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
                "/bilibili.app.view.v1.View/ShortFormVideoDownload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.view.v1.View",
                        "ShortFormVideoDownload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 点击播放器卡片事件
        pub async fn click_player_card(
            &mut self,
            request: impl tonic::IntoRequest<super::ClickPlayerCardReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/ClickPlayerCard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "ClickPlayerCard"));
            self.inner.unary(req, path, codec).await
        }
        /// 点击大型活动页预约
        pub async fn click_activity_season(
            &mut self,
            request: impl tonic::IntoRequest<super::ClickActivitySeasonReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/ClickActivitySeason",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.view.v1.View", "ClickActivitySeason"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 合集详情页
        pub async fn season(
            &mut self,
            request: impl tonic::IntoRequest<super::SeasonReq>,
        ) -> std::result::Result<tonic::Response<super::SeasonReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/Season",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "Season"));
            self.inner.unary(req, path, codec).await
        }
        /// 播放器卡片曝光
        pub async fn expose_player_card(
            &mut self,
            request: impl tonic::IntoRequest<super::ExposePlayerCardReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/ExposePlayerCard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.view.v1.View", "ExposePlayerCard"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 点击签订契约
        pub async fn add_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::AddContractReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/AddContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "AddContract"));
            self.inner.unary(req, path, codec).await
        }
        /// 资源包
        pub async fn chronos_pkg(
            &mut self,
            request: impl tonic::IntoRequest<super::ChronosPkgReq>,
        ) -> std::result::Result<tonic::Response<super::Chronos>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/ChronosPkg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "ChronosPkg"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn cache_view(
            &mut self,
            request: impl tonic::IntoRequest<super::CacheViewReq>,
        ) -> std::result::Result<tonic::Response<super::CacheViewReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/CacheView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "CacheView"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn continuous_play(
            &mut self,
            request: impl tonic::IntoRequest<super::ContinuousPlayReq>,
        ) -> std::result::Result<
            tonic::Response<super::ContinuousPlayReply>,
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
                "/bilibili.app.view.v1.View/ContinuousPlay",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "ContinuousPlay"));
            self.inner.unary(req, path, codec).await
        }
        /// 播放页推荐IFS
        pub async fn relates_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::RelatesFeedReq>,
        ) -> std::result::Result<
            tonic::Response<super::RelatesFeedReply>,
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
                "/bilibili.app.view.v1.View/RelatesFeed",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "RelatesFeed"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn premiere_archive(
            &mut self,
            request: impl tonic::IntoRequest<super::PremiereArchiveReq>,
        ) -> std::result::Result<
            tonic::Response<super::PremiereArchiveReply>,
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
                "/bilibili.app.view.v1.View/PremiereArchive",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "PremiereArchive"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn reserve(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveReq>,
        ) -> std::result::Result<tonic::Response<super::ReserveReply>, tonic::Status> {
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
                "/bilibili.app.view.v1.View/Reserve",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "Reserve"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn player_relates(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerRelatesReq>,
        ) -> std::result::Result<
            tonic::Response<super::PlayerRelatesReply>,
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
                "/bilibili.app.view.v1.View/PlayerRelates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "PlayerRelates"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn season_activity_record(
            &mut self,
            request: impl tonic::IntoRequest<super::SeasonActivityRecordReq>,
        ) -> std::result::Result<
            tonic::Response<super::SeasonActivityRecordReply>,
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
                "/bilibili.app.view.v1.View/SeasonActivityRecord",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.view.v1.View", "SeasonActivityRecord"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn season_widget_expose(
            &mut self,
            request: impl tonic::IntoRequest<super::SeasonWidgetExposeReq>,
        ) -> std::result::Result<
            tonic::Response<super::SeasonWidgetExposeReply>,
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
                "/bilibili.app.view.v1.View/SeasonWidgetExpose",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.view.v1.View", "SeasonWidgetExpose"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_arcs_player(
            &mut self,
            request: impl tonic::IntoRequest<super::GetArcsPlayerReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetArcsPlayerReply>,
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
                "/bilibili.app.view.v1.View/GetArcsPlayer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.view.v1.View", "GetArcsPlayer"));
            self.inner.unary(req, path, codec).await
        }
    }
}
