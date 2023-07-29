pub mod gateway;
/// at分组信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtGroup {
    /// 分组类型
    #[prost(enumeration = "AtGroupType", tag = "1")]
    pub group_type: i32,
    /// 分组名称
    #[prost(string, tag = "2")]
    pub group_name: ::prost::alloc::string::String,
    /// items
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<AtItem>,
}
/// at返回单条信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtItem {
    /// mid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 头像
    #[prost(string, tag = "3")]
    pub face: ::prost::alloc::string::String,
    /// 粉丝数
    #[prost(int32, tag = "4")]
    pub fans: i32,
    /// 认证信息
    #[prost(int32, tag = "5")]
    pub official_verify_type: i32,
}
/// at列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtListReq {
    /// mid
    #[prost(int64, tag = "1")]
    pub uid: i64,
}
/// at列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtListRsp {
    /// 分组信息
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<AtGroup>,
}
/// at搜索-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AtSearchReq {
    /// mid
    #[prost(int64, tag = "1")]
    pub uid: i64,
    /// 关键字
    #[prost(string, tag = "2")]
    pub keyword: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BottomBusiness {
    /// 业务方资源id
    #[prost(int64, tag = "1")]
    pub rid: i64,
    /// 业务方类型，定义在BottomBizType中
    #[prost(int64, tag = "2")]
    pub r#type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateActivity {
    ///
    #[prost(int64, tag = "1")]
    pub activity_id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub activity_state: i32,
    ///
    #[prost(int32, tag = "3")]
    pub is_new_activity: i32,
    ///
    #[prost(int32, tag = "4")]
    pub action: i32,
}
/// 动态附带的附加大卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAttachCard {
    /// 商品大卡
    #[prost(message, optional, tag = "1")]
    pub goods: ::core::option::Option<CreateGoodsCard>,
    /// 通用附加大卡，目前仅限定Match,Game,Ugc,Pugv,Reserve，且同时只能有一个
    #[prost(message, optional, tag = "2")]
    pub common_card: ::core::option::Option<CreateCommonAttachCard>,
}
/// 发布页预校验-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCheckResp {
    /// 发布相关的配置项
    #[prost(message, optional, tag = "1")]
    pub setting: ::core::option::Option<PublishSetting>,
    /// 用户具有的发布权限
    #[prost(message, optional, tag = "2")]
    pub permission: ::core::option::Option<UpPermission>,
    /// 分享渠道信息
    #[prost(message, optional, tag = "3")]
    pub share_info: ::core::option::Option<ShareChannel>,
    /// 小黄条
    #[prost(message, optional, tag = "4")]
    pub yellow_bar: ::core::option::Option<PublishYellowBar>,
    ///
    #[prost(message, optional, tag = "5")]
    pub plus_red_dot: ::core::option::Option<PlusRedDot>,
}
/// 创建动态时附带的通用附加卡详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommonAttachCard {
    /// 通用附加卡的类型
    #[prost(enumeration = "AttachCardType", tag = "1")]
    pub r#type: i32,
    /// 通用附加卡的业务id
    #[prost(int64, tag = "2")]
    pub biz_id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub reserve_source: i32,
    ///
    #[prost(int32, tag = "4")]
    pub reserve_lottery: i32,
}
/// 动态-描述文字模块
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContent {
    /// 描述信息（已按高亮拆分）
    #[prost(message, repeated, tag = "1")]
    pub contents: ::prost::alloc::vec::Vec<CreateContentItem>,
}
/// 文本描述
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContentItem {
    /// 原始文案
    #[prost(string, tag = "1")]
    pub raw_text: ::prost::alloc::string::String,
    /// 类型
    #[prost(enumeration = "ContentType", tag = "2")]
    pub r#type: i32,
    /// 简单内容，可能为文字，BVID，AVID，uid等；复杂内容需要单独定义结构体
    #[prost(string, tag = "3")]
    pub biz_id: ::prost::alloc::string::String,
    /// 商品内容
    #[prost(message, optional, tag = "4")]
    pub goods: ::core::option::Option<GoodsContent>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDynVideo {
    /// 投稿平台来源，具体写什么@产品
    #[prost(string, tag = "1")]
    pub relation_from: ::prost::alloc::string::String,
    /// 1 — 投稿入口 + 相册选择视频 2 — 投稿入口 + 拍摄 3 — 小视频入口 + 相册选择视频 4 — 小视频入口 + 拍摄
    #[prost(int32, tag = "3")]
    pub biz_from: i32,
    /// 投稿类型:  2-转载、1-自制
    #[prost(int32, tag = "4")]
    pub copyright: i32,
    /// 是否公开投稿 0允许公开，1不允许公开 默认 0公开
    #[prost(int32, tag = "5")]
    pub no_public: i32,
    /// 是否允许转载字段 0允许，1不允许，默认为0    copyright = 1 自制的时候默认勾选上no_reprint=1
    #[prost(int32, tag = "6")]
    pub no_reprint: i32,
    /// 转载的时候必须填写，非空字符串
    #[prost(string, tag = "7")]
    pub source: ::prost::alloc::string::String,
    /// 稿件封面必须填写,不能为空 封面不支持其他源站链接 请确保 cover 是 先经过上传接口
    #[prost(string, tag = "8")]
    pub cover: ::prost::alloc::string::String,
    /// 稿件标题
    #[prost(string, tag = "9")]
    pub title: ::prost::alloc::string::String,
    /// 稿件分区ID 必须是有效的二级分区ID
    #[prost(int64, tag = "10")]
    pub tid: i64,
    /// 标签 多个标签请使用英文逗号连接
    #[prost(string, tag = "11")]
    pub tag: ::prost::alloc::string::String,
    /// 稿件描述
    #[prost(string, tag = "12")]
    pub desc: ::prost::alloc::string::String,
    /// 当前输入环境下有，就输入<http://domain/x/app/archive/desc/format返回的desc_format值>
    /// 如果返回null就输入默认为0， 表示当前环境（分区+投稿类型）不参与简介格式化
    #[prost(int64, tag = "13")]
    pub desc_format_id: i64,
    /// 稿件是否开启充电面板，1为是, 0为否
    #[prost(int32, tag = "14")]
    pub open_elec: i32,
    /// 定时发布的时间
    #[prost(int32, tag = "15")]
    pub dtime: i32,
    /// 分P聚合字段
    #[prost(message, repeated, tag = "16")]
    pub videos: ::prost::alloc::vec::Vec<DynVideoMultiP>,
    /// 水印信息
    #[prost(message, optional, tag = "17")]
    pub watermark: ::core::option::Option<DynVideoWatermark>,
    /// 新增加通过tag来参加活动
    #[prost(int64, tag = "18")]
    pub mission_id: i64,
    /// 新增加可以添加动态内容
    #[prost(string, tag = "19")]
    pub dynamic: ::prost::alloc::string::String,
    /// 序列化后的extend_info扩展信息
    #[prost(string, tag = "20")]
    pub dynamic_extension: ::prost::alloc::string::String,
    /// 客户端控制字段
    #[prost(string, tag = "21")]
    pub dynamic_ctrl: ::prost::alloc::string::String,
    /// 动态来源
    #[prost(string, tag = "22")]
    pub dynamic_from: ::prost::alloc::string::String,
    /// 抽奖服务生成的ID
    #[prost(int64, tag = "23")]
    pub lottery_id: i64,
    ///
    #[prost(message, optional, tag = "24")]
    pub vote: ::core::option::Option<DynVideoVote>,
    /// 精选评论开关, true为开
    #[prost(bool, tag = "25")]
    pub up_selection_reply: bool,
    /// up主关闭评论
    #[prost(bool, tag = "26")]
    pub up_close_reply: bool,
    /// up主关闭弹幕
    #[prost(bool, tag = "27")]
    pub up_close_danmu: bool,
    /// 稿件投稿来源
    #[prost(int64, tag = "28")]
    pub up_from: i64,
    ///
    #[prost(int64, tag = "29")]
    pub duration: i64,
}
/// 创建动态视频的应答包（透传给客户端）
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDynVideoResult {
    /// 稿件id
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 说明信息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// 推荐的活动信息
    #[prost(message, optional, tag = "3")]
    pub submitact_banner: ::core::option::Option<DynVideoSubmitActBanner>,
    ///
    #[prost(message, optional, tag = "4")]
    pub push_intro: ::core::option::Option<DynVideoPushIntro>,
}
/// 创建动态时附带的商品大卡详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGoodsCard {
    /// 商品大卡中的商品id
    #[prost(string, repeated, tag = "1")]
    pub item_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 动态创建时的特殊选项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOption {
    /// 评论区展示UP自己精选的评论
    #[prost(int32, tag = "1")]
    pub up_choose_comment: i32,
    /// 初始评论区是关闭状态
    #[prost(int32, tag = "2")]
    pub close_comment: i32,
    /// 该动态不会被折叠
    /// 目前仅抽奖开奖动态不会被折叠
    #[prost(int32, tag = "3")]
    pub fold_exclude: i32,
    /// 审核等级，仅服务端发布时有效
    /// 100：自动过审
    /// 非100：默认的内网审核
    /// 默认为0
    #[prost(int32, tag = "4")]
    pub audit_level: i32,
    /// 根据转发内容同步生成一条源动态/资源的评论
    /// 仅转发和分享时有效
    #[prost(int32, tag = "5")]
    pub sync_to_comment: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub video_share_info: ::core::option::Option<VideoShareInfo>,
    ///
    #[prost(message, optional, tag = "7")]
    pub activity: ::core::option::Option<CreateActivity>,
}
/// 创建图文动态时的图片信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePic {
    /// 上传图片URL
    #[prost(string, tag = "1")]
    pub img_src: ::prost::alloc::string::String,
    /// 图片宽度
    #[prost(double, tag = "2")]
    pub img_width: f64,
    /// 图片高度
    #[prost(double, tag = "3")]
    pub img_height: f64,
    /// 图片大小，单位KB
    #[prost(double, tag = "4")]
    pub img_size: f64,
    ///
    #[prost(message, repeated, tag = "5")]
    pub img_tags: ::prost::alloc::vec::Vec<CreatePicTag>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePicTag {
    ///
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub tid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub text_string: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub r#type: i64,
    ///
    #[prost(int64, tag = "7")]
    pub source_type: i64,
    ///
    #[prost(string, tag = "8")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub schema_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub jump_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub orientation: i64,
    ///
    #[prost(int64, tag = "12")]
    pub x: i64,
    ///
    #[prost(int64, tag = "13")]
    pub y: i64,
    ///
    #[prost(string, tag = "14")]
    pub poi: ::prost::alloc::string::String,
}
/// 创建动态-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResp {
    /// 动态id
    #[prost(int64, tag = "1")]
    pub dyn_id: i64,
    /// 动态id str
    #[prost(string, tag = "2")]
    pub dyn_id_str: ::prost::alloc::string::String,
    /// 动态的类型
    #[prost(int64, tag = "3")]
    pub dyn_type: i64,
    /// 动态id
    #[prost(int64, tag = "4")]
    pub dyn_rid: i64,
    /// 假卡
    #[prost(message, optional, tag = "5")]
    pub fake_card: ::core::option::Option<super::app::dynamic::v2::DynamicItem>,
    /// 视频
    #[prost(message, optional, tag = "6")]
    pub video_result: ::core::option::Option<CreateDynVideoResult>,
}
/// 动态附带的小卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTag {
    /// lbs小卡
    #[prost(message, optional, tag = "1")]
    pub lbs: ::core::option::Option<ExtLbs>,
    /// 游戏通过SDK发布的动态需要带上游戏小卡
    #[prost(message, optional, tag = "2")]
    pub sdk_game: ::core::option::Option<BottomBusiness>,
    /// 必剪发布的动态需要带上必剪小卡
    #[prost(message, optional, tag = "3")]
    pub diversion: ::core::option::Option<BottomBusiness>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopic {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// 动态的标识
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynIdentity {
    /// 动态id
    #[prost(int64, tag = "1")]
    pub dyn_id: i64,
    /// 动态反向id，通过(type+rid组合)也可以唯一标识一个动态，与dyn_id出现任意一个即可
    #[prost(message, optional, tag = "2")]
    pub revs_id: ::core::option::Option<DynRevsId>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynRevsId {
    /// 动态类型
    #[prost(int64, tag = "1")]
    pub dyn_type: i64,
    /// 业务id
    #[prost(int64, tag = "2")]
    pub rid: i64,
}
/// 动态视频分P视频编辑环境上报信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoEditor {
    ///
    #[prost(int64, tag = "1")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub upfrom: i32,
    /// 滤镜
    #[prost(string, tag = "3")]
    pub filters: ::prost::alloc::string::String,
    /// 字体
    #[prost(string, tag = "4")]
    pub fonts: ::prost::alloc::string::String,
    /// 字幕
    #[prost(string, tag = "5")]
    pub subtitles: ::prost::alloc::string::String,
    /// bgm
    #[prost(string, tag = "6")]
    pub bgms: ::prost::alloc::string::String,
    /// 3d拍摄贴纸
    #[prost(string, tag = "7")]
    pub stickers: ::prost::alloc::string::String,
    /// 2d投稿贴纸
    #[prost(string, tag = "8")]
    pub videoup_stickers: ::prost::alloc::string::String,
    /// 视频转场特效
    #[prost(string, tag = "9")]
    pub trans: ::prost::alloc::string::String,
    /// 编辑器的主题使用相关
    #[prost(string, tag = "10")]
    pub makeups: ::prost::alloc::string::String,
    /// 整容之外科手术
    #[prost(string, tag = "11")]
    pub surgerys: ::prost::alloc::string::String,
    /// 美摄特定的videofx
    #[prost(string, tag = "12")]
    pub videofxs: ::prost::alloc::string::String,
    /// 编辑器的主题使用相关
    #[prost(string, tag = "13")]
    pub themes: ::prost::alloc::string::String,
    /// 拍摄之稿件合拍
    #[prost(string, tag = "14")]
    pub cooperates: ::prost::alloc::string::String,
    /// 拍摄之音乐卡点视频
    #[prost(string, tag = "15")]
    pub rhythms: ::prost::alloc::string::String,
    /// mvp特效
    #[prost(string, tag = "16")]
    pub effects: ::prost::alloc::string::String,
    /// mvp背景
    #[prost(string, tag = "17")]
    pub backgrounds: ::prost::alloc::string::String,
    /// mvp视频
    #[prost(string, tag = "18")]
    pub videos: ::prost::alloc::string::String,
    /// mvp音效
    #[prost(string, tag = "19")]
    pub sounds: ::prost::alloc::string::String,
    /// mvp花字
    #[prost(string, tag = "20")]
    pub flowers: ::prost::alloc::string::String,
    /// mvp封面模板
    #[prost(string, tag = "21")]
    pub cover_templates: ::prost::alloc::string::String,
    /// tts
    #[prost(string, tag = "22")]
    pub tts: ::prost::alloc::string::String,
    /// openings
    #[prost(string, tag = "23")]
    pub openings: ::prost::alloc::string::String,
    /// 录音题词
    #[prost(bool, tag = "24")]
    pub record_text: bool,
    /// 虚拟形象上报
    #[prost(string, tag = "25")]
    pub vupers: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "26")]
    pub features: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "27")]
    pub bcut_features: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "28")]
    pub audio_record: i32,
    ///
    #[prost(int32, tag = "29")]
    pub camera: i32,
    ///
    #[prost(int32, tag = "30")]
    pub speed: i32,
    ///
    #[prost(int32, tag = "31")]
    pub camera_rotate: i32,
    ///
    #[prost(int32, tag = "32")]
    pub screen_record: i32,
    ///
    #[prost(int32, tag = "33")]
    pub default_end: i32,
    ///
    #[prost(int32, tag = "34")]
    pub duration: i32,
    ///
    #[prost(int32, tag = "35")]
    pub pic_count: i32,
    ///
    #[prost(int32, tag = "36")]
    pub video_count: i32,
    ///
    #[prost(int32, tag = "37")]
    pub shot_duration: i32,
    ///
    #[prost(string, tag = "38")]
    pub shot_game: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "39")]
    pub highlight: bool,
    ///
    #[prost(int32, tag = "40")]
    pub highlight_cnt: i32,
    ///
    #[prost(int32, tag = "41")]
    pub pip_count: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoHotAct {
    ///
    #[prost(int64, tag = "1")]
    pub act_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub etime: i64,
    ///
    #[prost(int64, tag = "3")]
    pub id: i64,
    ///
    #[prost(string, tag = "4")]
    pub pic: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub stime: i64,
    ///
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub link: ::prost::alloc::string::String,
}
/// 动态视频分P聚合字段
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoMultiP {
    /// 分P标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 分P的文件名
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
    /// 编辑环境上报信息
    #[prost(message, optional, tag = "4")]
    pub editor: ::core::option::Option<DynVideoEditor>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoPushIntro {
    ///
    #[prost(int32, tag = "1")]
    pub show: i32,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoSubmitActBanner {
    ///
    #[prost(string, tag = "1")]
    pub hotact_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub hotact_url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<DynVideoHotAct>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoVote {
    ///
    #[prost(int64, tag = "1")]
    pub vote_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub vote_title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub top_for_reply: i32,
}
/// 动态视频水印信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynVideoWatermark {
    /// 水印状态
    /// 0-关闭 1-打开 2-预览
    #[prost(int32, tag = "1")]
    pub state: i32,
    /// 类型
    /// 1-用户昵称类型 2-用户id类型 3-用户名在logo下面
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    /// 位置
    /// 1-左上 2-右上 3-左下 4-右下
    #[prost(int32, tag = "3")]
    pub position: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtLbs {
    ///
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub distance: i64,
    ///
    #[prost(int64, tag = "3")]
    pub r#type: i64,
    ///
    #[prost(string, tag = "4")]
    pub poi: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub location: ::core::option::Option<LbsLoc>,
    ///
    #[prost(string, tag = "6")]
    pub show_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub show_distance: ::prost::alloc::string::String,
}
/// 根据name取uid-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUidByNameReq {
    /// 查询昵称列表
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 根据name取uid-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUidByNameRsp {
    /// k:昵称 v:mid
    #[prost(map = "string, int64", tag = "1")]
    pub uids: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
/// 发布时附带的商品卡的详细内容
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsContent {
    /// 商品类型
    /// 1淘宝、2会员购
    #[prost(int32, tag = "1")]
    pub source_type: i32,
    /// 商品的id
    #[prost(int64, tag = "2")]
    pub item_id: i64,
    /// 店铺的id，兼容老版本
    #[prost(int64, tag = "3")]
    pub shop_id: i64,
}
/// UP已经创建的活动列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchedActivity {
    /// 模块名称，示例："已创建的活动"
    #[prost(string, tag = "1")]
    pub module_title: ::prost::alloc::string::String,
    /// 已创建的活动列表
    #[prost(message, repeated, tag = "2")]
    pub activities: ::prost::alloc::vec::Vec<LaunchedActivityItem>,
    /// 展示更多按钮
    /// 已创建活动大于5个时下发
    #[prost(message, optional, tag = "3")]
    pub show_more: ::core::option::Option<ShowMoreLaunchedActivity>,
}
/// UP已经创建的活动详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaunchedActivityItem {
    /// 活动id
    #[prost(int64, tag = "1")]
    pub activity_id: i64,
    /// 活动名称
    #[prost(string, tag = "2")]
    pub activity_name: ::prost::alloc::string::String,
    /// 活动是否已上线
    /// 0未上线 1已上线
    #[prost(int32, tag = "3")]
    pub activity_state: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbsLoc {
    /// 经度
    #[prost(double, tag = "1")]
    pub lat: f64,
    /// 纬度
    #[prost(double, tag = "2")]
    pub lng: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaDataCtrl {
    /// 客户端平台
    #[prost(string, tag = "1")]
    pub platform: ::prost::alloc::string::String,
    /// 客户端build号
    #[prost(string, tag = "2")]
    pub build: ::prost::alloc::string::String,
    /// 客户端移动设备类型
    #[prost(string, tag = "3")]
    pub mobi_app: ::prost::alloc::string::String,
    /// 客户端buvid
    #[prost(string, tag = "4")]
    pub buvid: ::prost::alloc::string::String,
    /// 用户设备信息
    #[prost(string, tag = "5")]
    pub device: ::prost::alloc::string::String,
    /// 请求来源页面的spmid
    #[prost(string, tag = "6")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 请求来源页面
    #[prost(string, tag = "7")]
    pub from: ::prost::alloc::string::String,
    /// 请求的trace_id
    #[prost(string, tag = "8")]
    pub trace_id: ::prost::alloc::string::String,
    /// 青少年模式
    #[prost(int32, tag = "9")]
    pub teenager_mode: i32,
    /// 0:正常 1:冷启动
    #[prost(int32, tag = "10")]
    pub cold_start: i32,
    /// 客户端版本号
    #[prost(string, tag = "11")]
    pub version: ::prost::alloc::string::String,
    /// 网络状态
    /// Unknown=0 WIFI=1 WWAN=2
    #[prost(int32, tag = "12")]
    pub network: i32,
    /// 用户ip地址
    #[prost(string, tag = "13")]
    pub ip: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlusRedDot {
    ///
    #[prost(int64, tag = "1")]
    pub plus_has_red_dot: i64,
}
/// 小程序内容定义
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Program {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 描述文字
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 封面图
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "4")]
    pub target_url: ::prost::alloc::string::String,
    /// 小程序icon
    #[prost(string, tag = "5")]
    pub icon: ::prost::alloc::string::String,
    /// 小程序名称
    #[prost(string, tag = "6")]
    pub program_text: ::prost::alloc::string::String,
    /// 跳转链接文案，如：去看看
    #[prost(string, tag = "7")]
    pub jump_text: ::prost::alloc::string::String,
}
/// 发布相关的设置项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishSetting {
    /// 提示转为专栏的最小字数，使用utf-16编码计算字符数
    #[prost(int32, tag = "1")]
    pub min_words_to_article: i32,
    /// 提示转为专栏的最大字数，使用utf-16编码计算字符数
    #[prost(int32, tag = "2")]
    pub max_words_to_article: i32,
    /// gif上传的最大值，单位：MB
    #[prost(int32, tag = "3")]
    pub upload_size: i32,
}
/// 发布页小黄条
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishYellowBar {
    /// 展示文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// 展示图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepostInitCheck {
    ///
    #[prost(message, optional, tag = "1")]
    pub repost_src: ::core::option::Option<DynIdentity>,
    ///
    #[prost(string, tag = "2")]
    pub share_id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub share_mode: i32,
}
/// 分享渠道信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareChannel {
    /// 业务类型，如动态是"dynamic"
    #[prost(string, tag = "1")]
    pub share_origin: ::prost::alloc::string::String,
    /// 业务资源id
    #[prost(string, tag = "2")]
    pub oid: ::prost::alloc::string::String,
    /// 辅助id, 非必返回字段
    #[prost(string, tag = "3")]
    pub sid: ::prost::alloc::string::String,
    /// 渠道列表
    #[prost(message, repeated, tag = "4")]
    pub share_channels: ::prost::alloc::vec::Vec<ShareChannelItem>,
}
/// 渠道
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareChannelItem {
    /// 展示文案
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 展示图标
    #[prost(string, tag = "2")]
    pub picture: ::prost::alloc::string::String,
    /// 渠道名称
    #[prost(string, tag = "3")]
    pub share_channel: ::prost::alloc::string::String,
    /// 预约卡分享图信息，仅分享有预约信息的动态时存在
    #[prost(message, optional, tag = "4")]
    pub reserve: ::core::option::Option<ShareReserve>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareReserve {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 描述（时间+类型）
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    /// 二维码附带icon
    #[prost(string, tag = "3")]
    pub qr_code_icon: ::prost::alloc::string::String,
    /// 二维码附带文本
    #[prost(string, tag = "4")]
    pub qr_code_text: ::prost::alloc::string::String,
    /// 二维码链接
    #[prost(string, tag = "5")]
    pub qr_code_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub poster: ::core::option::Option<ShareReservePoster>,
    ///
    #[prost(message, optional, tag = "9")]
    pub reserve_lottery: ::core::option::Option<ShareReserveLottery>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareReserveLottery {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareReservePoster {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "2")]
    pub width: f64,
    ///
    #[prost(double, tag = "3")]
    pub height: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareResult {
    ///
    #[prost(int64, tag = "1")]
    pub share_enable: i64,
    ///
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
}
/// UP已经创建的活动列表中的展示更多按钮详情
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowMoreLaunchedActivity {
    /// 按钮的文案
    #[prost(string, tag = "1")]
    pub button_text: ::prost::alloc::string::String,
    /// 按钮的跳转链接
    #[prost(string, tag = "2")]
    pub jump_url: ::prost::alloc::string::String,
}
/// 通用模板的网页元内容(sketch结构)定义
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sketch {
    /// 元内容标题，长度30限制
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 描述文字（文本内容第二行），长度233限制
    #[prost(string, tag = "2")]
    pub desc_text: ::prost::alloc::string::String,
    /// 文本文字（文本内容第三行），仅限竖图通用卡片使用，长度233限制
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
    /// 表示业务方的id表示，对于在业务方有唯一标示的必填
    #[prost(int64, tag = "4")]
    pub biz_id: i64,
    /// 业务类型，与展示时的右上角标有关，需要业务方向动态申请
    #[prost(int64, tag = "5")]
    pub biz_type: i64,
    /// 封面图片链接地址，域名需要符合白名单
    #[prost(string, tag = "6")]
    pub cover_url: ::prost::alloc::string::String,
    /// 跳转链接地址，域名需要符合白名单
    #[prost(string, tag = "7")]
    pub target_url: ::prost::alloc::string::String,
}
/// 发布相关的权限内容
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpPermission {
    /// 通用权限列表
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<UpPermissionItem>,
    /// 已经创建的活动列表
    #[prost(message, optional, tag = "2")]
    pub launched_activity: ::core::option::Option<LaunchedActivity>,
    ///
    #[prost(message, optional, tag = "3")]
    pub share_result: ::core::option::Option<ShareResult>,
}
/// 通用发布权限内容的详细定义
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpPermissionItem {
    /// 类型，enum UpPermissionType
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// UP是否有权限
    /// 1-有，2-限制（展示但不可点，仅预约使用）
    #[prost(int32, tag = "2")]
    pub permission: i32,
    /// 按钮文案
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// 功能开关的副标题
    #[prost(string, tag = "4")]
    pub subtitle: ::prost::alloc::string::String,
    /// 按钮图标的url地址
    #[prost(string, tag = "5")]
    pub icon: ::prost::alloc::string::String,
    /// 跳转链接，permission=1时点击按钮跳到此链接
    #[prost(string, tag = "6")]
    pub jump_url: ::prost::alloc::string::String,
    /// 错误提示，permission=2时点击按钮会弹出此提示，目前仅预约使用
    #[prost(string, tag = "7")]
    pub toast: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub has_red_dot: i64,
}
/// 用户主动发布（app/web发布）时的meta信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCreateMeta {
    /// 用户发布客户端的meta信息
    #[prost(message, optional, tag = "1")]
    pub app_meta: ::core::option::Option<MetaDataCtrl>,
    /// 用户发布时的位置信息（经纬度）
    #[prost(message, optional, tag = "2")]
    pub loc: ::core::option::Option<LbsLoc>,
    /// 1-发布页转发 2-立即转发
    #[prost(int32, tag = "3")]
    pub repost_mode: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoShareInfo {
    ///
    #[prost(int64, tag = "1")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub part: i32,
}
/// at分组类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AtGroupType {
    /// 默认
    Default = 0,
    /// 最近联系
    Recent = 1,
    /// 我的关注（互相关注 > 单向关注）
    Follow = 2,
    /// 我的粉丝
    Fans = 3,
    /// 其他
    Others = 4,
}
impl AtGroupType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AtGroupType::Default => "AT_GROUP_TYPE_DEFAULT",
            AtGroupType::Recent => "AT_GROUP_TYPE_RECENT",
            AtGroupType::Follow => "AT_GROUP_TYPE_FOLLOW",
            AtGroupType::Fans => "AT_GROUP_TYPE_FANS",
            AtGroupType::Others => "AT_GROUP_TYPE_OTHERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AT_GROUP_TYPE_DEFAULT" => Some(Self::Default),
            "AT_GROUP_TYPE_RECENT" => Some(Self::Recent),
            "AT_GROUP_TYPE_FOLLOW" => Some(Self::Follow),
            "AT_GROUP_TYPE_FANS" => Some(Self::Fans),
            "AT_GROUP_TYPE_OTHERS" => Some(Self::Others),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttachCardType {
    /// 无
    AttachCardNone = 0,
    /// 商品卡
    AttachCardGoods = 1,
    /// 投票卡
    AttachCardVote = 2,
    /// ugc视频卡
    AttachCardUgc = 3,
    /// 帮推
    AttachCardActivity = 4,
    /// 官方活动
    AttachCardOfficialActivity = 5,
    /// 话题活动
    AttachCardTopic = 6,
    /// OGV
    AttachCardOgv = 7,
    /// OGV自动出卡
    AttachCardAutoOgv = 8,
    /// 游戏
    AttachCardGame = 9,
    /// 漫画
    AttachCardManga = 10,
    /// 装扮
    AttachCardDecoration = 11,
    /// 赛事
    AttachCardMatch = 12,
    /// 课程
    AttachCardPugv = 13,
    /// 预约
    AttachCardReserve = 14,
    /// up主话题活动
    AttachCardUpTopic = 15,
}
impl AttachCardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttachCardType::AttachCardNone => "ATTACH_CARD_NONE",
            AttachCardType::AttachCardGoods => "ATTACH_CARD_GOODS",
            AttachCardType::AttachCardVote => "ATTACH_CARD_VOTE",
            AttachCardType::AttachCardUgc => "ATTACH_CARD_UGC",
            AttachCardType::AttachCardActivity => "ATTACH_CARD_ACTIVITY",
            AttachCardType::AttachCardOfficialActivity => "ATTACH_CARD_OFFICIAL_ACTIVITY",
            AttachCardType::AttachCardTopic => "ATTACH_CARD_TOPIC",
            AttachCardType::AttachCardOgv => "ATTACH_CARD_OGV",
            AttachCardType::AttachCardAutoOgv => "ATTACH_CARD_AUTO_OGV",
            AttachCardType::AttachCardGame => "ATTACH_CARD_GAME",
            AttachCardType::AttachCardManga => "ATTACH_CARD_MANGA",
            AttachCardType::AttachCardDecoration => "ATTACH_CARD_DECORATION",
            AttachCardType::AttachCardMatch => "ATTACH_CARD_MATCH",
            AttachCardType::AttachCardPugv => "ATTACH_CARD_PUGV",
            AttachCardType::AttachCardReserve => "ATTACH_CARD_RESERVE",
            AttachCardType::AttachCardUpTopic => "ATTACH_CARD_UP_TOPIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ATTACH_CARD_NONE" => Some(Self::AttachCardNone),
            "ATTACH_CARD_GOODS" => Some(Self::AttachCardGoods),
            "ATTACH_CARD_VOTE" => Some(Self::AttachCardVote),
            "ATTACH_CARD_UGC" => Some(Self::AttachCardUgc),
            "ATTACH_CARD_ACTIVITY" => Some(Self::AttachCardActivity),
            "ATTACH_CARD_OFFICIAL_ACTIVITY" => Some(Self::AttachCardOfficialActivity),
            "ATTACH_CARD_TOPIC" => Some(Self::AttachCardTopic),
            "ATTACH_CARD_OGV" => Some(Self::AttachCardOgv),
            "ATTACH_CARD_AUTO_OGV" => Some(Self::AttachCardAutoOgv),
            "ATTACH_CARD_GAME" => Some(Self::AttachCardGame),
            "ATTACH_CARD_MANGA" => Some(Self::AttachCardManga),
            "ATTACH_CARD_DECORATION" => Some(Self::AttachCardDecoration),
            "ATTACH_CARD_MATCH" => Some(Self::AttachCardMatch),
            "ATTACH_CARD_PUGV" => Some(Self::AttachCardPugv),
            "ATTACH_CARD_RESERVE" => Some(Self::AttachCardReserve),
            "ATTACH_CARD_UP_TOPIC" => Some(Self::AttachCardUpTopic),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    /// 占位
    None = 0,
    /// 文本，简单内容，biz_id就是文本
    Text = 1,
    /// @用户，简单内容，biz_id是用户uid
    At = 2,
    /// 抽奖，简单内容，biz_id是抽奖id
    Lottery = 3,
    /// 投票，简单内容，biz_id是投票id
    Vote = 4,
    /// 话题，简单内容，biz_id是话题id
    Topic = 5,
    /// 商品文字链，复杂内容，定义在GoodsContent结构，biz_id为空
    Goods = 6,
    /// bv，简单内容，biz_id是bvid，包括"BV1"等内容
    Bv = 7,
    /// av，简单内容，biz_id是avid
    Av = 8,
    /// 表情，简单内容，biz_id为空
    Emoji = 9,
    /// 外露用户，暂未使用
    User = 10,
    /// 专栏，简单内容，biz_id是cvid
    Cv = 11,
    /// 废弃业务，无用
    Vc = 12,
    /// 网址，简单内容，biz_id是网页链接
    Web = 13,
    /// 淘宝内容，暂时不用
    Taobao = 14,
    /// 邮箱，简单内容，biz_id是邮箱地址
    Mail = 15,
    /// 番剧season，简单内容，biz_id是番剧的season_id
    OgvSeason = 16,
    /// 番剧ep，简单内容，biz_id是番剧的epid
    OgvEp = 17,
}
impl ContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentType::None => "CONTENT_TYPE_NONE",
            ContentType::Text => "TEXT",
            ContentType::At => "AT",
            ContentType::Lottery => "LOTTERY",
            ContentType::Vote => "VOTE",
            ContentType::Topic => "TOPIC",
            ContentType::Goods => "GOODS",
            ContentType::Bv => "BV",
            ContentType::Av => "AV",
            ContentType::Emoji => "EMOJI",
            ContentType::User => "USER",
            ContentType::Cv => "CV",
            ContentType::Vc => "VC",
            ContentType::Web => "WEB",
            ContentType::Taobao => "TAOBAO",
            ContentType::Mail => "MAIL",
            ContentType::OgvSeason => "OGV_SEASON",
            ContentType::OgvEp => "OGV_EP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTENT_TYPE_NONE" => Some(Self::None),
            "TEXT" => Some(Self::Text),
            "AT" => Some(Self::At),
            "LOTTERY" => Some(Self::Lottery),
            "VOTE" => Some(Self::Vote),
            "TOPIC" => Some(Self::Topic),
            "GOODS" => Some(Self::Goods),
            "BV" => Some(Self::Bv),
            "AV" => Some(Self::Av),
            "EMOJI" => Some(Self::Emoji),
            "USER" => Some(Self::User),
            "CV" => Some(Self::Cv),
            "VC" => Some(Self::Vc),
            "WEB" => Some(Self::Web),
            "TAOBAO" => Some(Self::Taobao),
            "MAIL" => Some(Self::Mail),
            "OGV_SEASON" => Some(Self::OgvSeason),
            "OGV_EP" => Some(Self::OgvEp),
            _ => None,
        }
    }
}
/// 发布页预校验场景
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CreateInitCheckScene {
    ///
    Invalid = 0,
    /// 动态页面右上角点击进入发布页
    Normal = 1,
    /// 动态feed流转发、三点分享，动态详情页转发
    Repost = 2,
    /// 其他页面分享到动态
    Share = 3,
    ///
    ReserveShare = 4,
}
impl CreateInitCheckScene {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CreateInitCheckScene::Invalid => "CREATE_INIT_CHECK_SCENE_INVALID",
            CreateInitCheckScene::Normal => "CREATE_INIT_CHECK_SCENE_NORMAL",
            CreateInitCheckScene::Repost => "CREATE_INIT_CHECK_SCENE_REPOST",
            CreateInitCheckScene::Share => "CREATE_INIT_CHECK_SCENE_SHARE",
            CreateInitCheckScene::ReserveShare => "CREATE_INIT_CHECK_SCENE_RESERVE_SHARE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREATE_INIT_CHECK_SCENE_INVALID" => Some(Self::Invalid),
            "CREATE_INIT_CHECK_SCENE_NORMAL" => Some(Self::Normal),
            "CREATE_INIT_CHECK_SCENE_REPOST" => Some(Self::Repost),
            "CREATE_INIT_CHECK_SCENE_SHARE" => Some(Self::Share),
            "CREATE_INIT_CHECK_SCENE_RESERVE_SHARE" => Some(Self::ReserveShare),
            _ => None,
        }
    }
}
/// 发布类型（场景）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CreateScene {
    ///
    Invalid = 0,
    /// 发布纯文字动态
    CreateWord = 1,
    /// 发布图文动态
    CreateDraw = 2,
    /// 发布动态视频
    CreateDynVideo = 3,
    /// 转发动态
    Repost = 4,
    /// 分享业务方资源
    ShareBiz = 5,
    /// 分享网页（通用模板）
    SharePage = 6,
    /// 分享小程序
    ShareProgram = 7,
    /// 评论同步到动态
    ReplySync = 8,
    /// 评论同步到动态并且发起活动
    ReplyCreateActivity = 9,
}
impl CreateScene {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CreateScene::Invalid => "CREATE_SCENE_INVALID",
            CreateScene::CreateWord => "CREATE_SCENE_CREATE_WORD",
            CreateScene::CreateDraw => "CREATE_SCENE_CREATE_DRAW",
            CreateScene::CreateDynVideo => "CREATE_SCENE_CREATE_DYN_VIDEO",
            CreateScene::Repost => "CREATE_SCENE_REPOST",
            CreateScene::ShareBiz => "CREATE_SCENE_SHARE_BIZ",
            CreateScene::SharePage => "CREATE_SCENE_SHARE_PAGE",
            CreateScene::ShareProgram => "CREATE_SCENE_SHARE_PROGRAM",
            CreateScene::ReplySync => "CREATE_SCENE_REPLY_SYNC",
            CreateScene::ReplyCreateActivity => "CREATE_SCENE_REPLY_CREATE_ACTIVITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREATE_SCENE_INVALID" => Some(Self::Invalid),
            "CREATE_SCENE_CREATE_WORD" => Some(Self::CreateWord),
            "CREATE_SCENE_CREATE_DRAW" => Some(Self::CreateDraw),
            "CREATE_SCENE_CREATE_DYN_VIDEO" => Some(Self::CreateDynVideo),
            "CREATE_SCENE_REPOST" => Some(Self::Repost),
            "CREATE_SCENE_SHARE_BIZ" => Some(Self::ShareBiz),
            "CREATE_SCENE_SHARE_PAGE" => Some(Self::SharePage),
            "CREATE_SCENE_SHARE_PROGRAM" => Some(Self::ShareProgram),
            "CREATE_SCENE_REPLY_SYNC" => Some(Self::ReplySync),
            "CREATE_SCENE_REPLY_CREATE_ACTIVITY" => Some(Self::ReplyCreateActivity),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveSource {
    ///
    New = 0,
    ///
    Associated = 1,
}
impl ReserveSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReserveSource::New => "RESERVE_SOURCE_NEW",
            ReserveSource::Associated => "RESERVE_SOURCE_ASSOCIATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVE_SOURCE_NEW" => Some(Self::New),
            "RESERVE_SOURCE_ASSOCIATED" => Some(Self::Associated),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpPermissionType {
    /// 占位
    None = 0,
    /// 是否是抽奖的灰度用户，默认不是
    Lottery = 1,
    /// 之前是否发过小视频，默认没发过
    ClipPublished = 2,
    /// 是否可以添加ugc附加卡，默认不可以
    UgcAttachCard = 3,
    /// 是否有权限添加商品附加卡
    GoodsAttachCard = 4,
    /// 是否有权限自主精选评论白名单，默认没有
    ChooseComment = 5,
    /// 是否有权限关闭评论区，默认有
    ControlComment = 6,
    /// 是否有权限关闭弹幕（仅对动态视频生效），默认有
    ControlDanmu = 7,
    /// 是否可以发起稿件预约
    VideoReserve = 8,
    /// 是否可以发起直播预约
    LiveReserve = 9,
}
impl UpPermissionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpPermissionType::None => "UP_PERMISSION_TYPE_NONE",
            UpPermissionType::Lottery => "UP_PERMISSION_TYPE_LOTTERY",
            UpPermissionType::ClipPublished => "UP_PERMISSION_TYPE_CLIP_PUBLISHED",
            UpPermissionType::UgcAttachCard => "UP_PERMISSION_TYPE_UGC_ATTACH_CARD",
            UpPermissionType::GoodsAttachCard => "UP_PERMISSION_TYPE_GOODS_ATTACH_CARD",
            UpPermissionType::ChooseComment => "UP_PERMISSION_TYPE_CHOOSE_COMMENT",
            UpPermissionType::ControlComment => "UP_PERMISSION_TYPE_CONTROL_COMMENT",
            UpPermissionType::ControlDanmu => "UP_PERMISSION_TYPE_CONTROL_DANMU",
            UpPermissionType::VideoReserve => "UP_PERMISSION_TYPE_VIDEO_RESERVE",
            UpPermissionType::LiveReserve => "UP_PERMISSION_TYPE_LIVE_RESERVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UP_PERMISSION_TYPE_NONE" => Some(Self::None),
            "UP_PERMISSION_TYPE_LOTTERY" => Some(Self::Lottery),
            "UP_PERMISSION_TYPE_CLIP_PUBLISHED" => Some(Self::ClipPublished),
            "UP_PERMISSION_TYPE_UGC_ATTACH_CARD" => Some(Self::UgcAttachCard),
            "UP_PERMISSION_TYPE_GOODS_ATTACH_CARD" => Some(Self::GoodsAttachCard),
            "UP_PERMISSION_TYPE_CHOOSE_COMMENT" => Some(Self::ChooseComment),
            "UP_PERMISSION_TYPE_CONTROL_COMMENT" => Some(Self::ControlComment),
            "UP_PERMISSION_TYPE_CONTROL_DANMU" => Some(Self::ControlDanmu),
            "UP_PERMISSION_TYPE_VIDEO_RESERVE" => Some(Self::VideoReserve),
            "UP_PERMISSION_TYPE_LIVE_RESERVE" => Some(Self::LiveReserve),
            _ => None,
        }
    }
}
