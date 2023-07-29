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
    #[prost(string, tag = "3")]
    pub mid: ::prost::alloc::string::String,
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
    ///
    #[prost(int64, tag = "11")]
    pub display: i64,
}
/// ott互动弹幕条目信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDmOtt {
    /// 弹幕id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 对象视频cid
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 发送者mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    /// 互动弹幕指令
    #[prost(string, tag = "5")]
    pub command: ::prost::alloc::string::String,
    /// 互动弹幕正文
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub state: i32,
    /// 出现时间
    #[prost(int32, tag = "8")]
    pub progress: i32,
    /// 创建时间
    #[prost(string, tag = "9")]
    pub ctime: ::prost::alloc::string::String,
    /// 发布时间
    #[prost(string, tag = "10")]
    pub mtime: ::prost::alloc::string::String,
    /// 扩展json数据
    #[prost(string, tag = "11")]
    pub extra: ::prost::alloc::string::String,
    /// 弹幕id str类型
    #[prost(string, tag = "12")]
    pub id_str: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDmsOttReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub command_dms_ott: ::prost::alloc::vec::Vec<CommandDmOtt>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDmsOttReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub mid: i64,
}
/// 弹幕ai云屏蔽列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmakuAiFlag {
    /// 弹幕ai云屏蔽条目
    #[prost(message, repeated, tag = "1")]
    pub dm_flags: ::prost::alloc::vec::Vec<DanmakuFlag>,
}
/// 弹幕条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmakuElem {
    /// 弹幕dmid
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 弹幕出现位置(单位ms)
    #[prost(int32, tag = "2")]
    pub progress: i32,
    /// 弹幕类型
    #[prost(int32, tag = "3")]
    pub mode: i32,
    /// 弹幕字号
    #[prost(int32, tag = "4")]
    pub fontsize: i32,
    /// 弹幕颜色
    #[prost(uint32, tag = "5")]
    pub color: u32,
    /// 发送着mid hash
    #[prost(string, tag = "6")]
    pub mid_hash: ::prost::alloc::string::String,
    /// 弹幕正文
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
    /// 发送时间
    #[prost(int64, tag = "8")]
    pub ctime: i64,
    /// 权重 区间:\[1,10\]
    #[prost(int32, tag = "9")]
    pub weight: i32,
    /// 动作
    #[prost(string, tag = "10")]
    pub action: ::prost::alloc::string::String,
    /// 弹幕池
    #[prost(int32, tag = "11")]
    pub pool: i32,
    /// 弹幕dmid str
    #[prost(string, tag = "12")]
    pub id_str: ::prost::alloc::string::String,
    /// 弹幕属性位(bin求AND)
    /// bit0:保护 bit1:直播 bit2:高赞
    #[prost(int32, tag = "13")]
    pub attr: i32,
}
/// 弹幕ai云屏蔽条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmakuFlag {
    /// 弹幕dmid
    #[prost(int64, tag = "1")]
    pub dmid: i64,
    /// 评分
    #[prost(uint32, tag = "2")]
    pub flag: u32,
}
/// 云屏蔽配置信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmakuFlagConfig {
    /// 云屏蔽等级
    #[prost(int32, tag = "1")]
    pub rec_flag: i32,
    /// 云屏蔽文案
    #[prost(string, tag = "2")]
    pub rec_text: ::prost::alloc::string::String,
    /// 云屏蔽开关
    #[prost(int32, tag = "3")]
    pub rec_switch: i32,
}
/// 弹幕默认配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmuDefaultPlayerConfig {
    /// 是否使用推荐弹幕设置
    #[prost(bool, tag = "1")]
    pub player_danmaku_use_default_config: bool,
    /// 是否开启智能云屏蔽
    #[prost(bool, tag = "4")]
    pub player_danmaku_ai_recommended_switch: bool,
    /// 智能云屏蔽等级
    #[prost(int32, tag = "5")]
    pub player_danmaku_ai_recommended_level: i32,
    /// 是否屏蔽顶端弹幕
    #[prost(bool, tag = "6")]
    pub player_danmaku_blocktop: bool,
    /// 是否屏蔽滚动弹幕
    #[prost(bool, tag = "7")]
    pub player_danmaku_blockscroll: bool,
    /// 是否屏蔽底端弹幕
    #[prost(bool, tag = "8")]
    pub player_danmaku_blockbottom: bool,
    /// 是否屏蔽彩色弹幕
    #[prost(bool, tag = "9")]
    pub player_danmaku_blockcolorful: bool,
    /// 是否屏蔽重复弹幕
    #[prost(bool, tag = "10")]
    pub player_danmaku_blockrepeat: bool,
    /// 是否屏蔽高级弹幕
    #[prost(bool, tag = "11")]
    pub player_danmaku_blockspecial: bool,
    /// 弹幕不透明度
    #[prost(float, tag = "12")]
    pub player_danmaku_opacity: f32,
    /// 弹幕缩放比例
    #[prost(float, tag = "13")]
    pub player_danmaku_scalingfactor: f32,
    /// 弹幕显示区域
    #[prost(float, tag = "14")]
    pub player_danmaku_domain: f32,
    /// 弹幕速度
    #[prost(int32, tag = "15")]
    pub player_danmaku_speed: i32,
    /// 是否开启弹幕
    #[prost(bool, tag = "16")]
    pub inline_player_danmaku_switch: bool,
    ///
    #[prost(int32, tag = "17")]
    pub player_danmaku_senior_mode_switch: i32,
}
/// 弹幕配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmuPlayerConfig {
    /// 是否开启弹幕
    #[prost(bool, tag = "1")]
    pub player_danmaku_switch: bool,
    /// 是否记录弹幕开关设置
    #[prost(bool, tag = "2")]
    pub player_danmaku_switch_save: bool,
    /// 是否使用推荐弹幕设置
    #[prost(bool, tag = "3")]
    pub player_danmaku_use_default_config: bool,
    /// 是否开启智能云屏蔽
    #[prost(bool, tag = "4")]
    pub player_danmaku_ai_recommended_switch: bool,
    /// 智能云屏蔽等级
    #[prost(int32, tag = "5")]
    pub player_danmaku_ai_recommended_level: i32,
    /// 是否屏蔽顶端弹幕
    #[prost(bool, tag = "6")]
    pub player_danmaku_blocktop: bool,
    /// 是否屏蔽滚动弹幕
    #[prost(bool, tag = "7")]
    pub player_danmaku_blockscroll: bool,
    /// 是否屏蔽底端弹幕
    #[prost(bool, tag = "8")]
    pub player_danmaku_blockbottom: bool,
    /// 是否屏蔽彩色弹幕
    #[prost(bool, tag = "9")]
    pub player_danmaku_blockcolorful: bool,
    /// 是否屏蔽重复弹幕
    #[prost(bool, tag = "10")]
    pub player_danmaku_blockrepeat: bool,
    /// 是否屏蔽高级弹幕
    #[prost(bool, tag = "11")]
    pub player_danmaku_blockspecial: bool,
    /// 弹幕不透明度
    #[prost(float, tag = "12")]
    pub player_danmaku_opacity: f32,
    /// 弹幕缩放比例
    #[prost(float, tag = "13")]
    pub player_danmaku_scalingfactor: f32,
    /// 弹幕显示区域
    #[prost(float, tag = "14")]
    pub player_danmaku_domain: f32,
    /// 弹幕速度
    #[prost(int32, tag = "15")]
    pub player_danmaku_speed: i32,
    /// 是否开启屏蔽列表
    #[prost(bool, tag = "16")]
    pub player_danmaku_enableblocklist: bool,
    /// 是否开启弹幕
    #[prost(bool, tag = "17")]
    pub inline_player_danmaku_switch: bool,
    ///
    #[prost(int32, tag = "18")]
    pub inline_player_danmaku_config: i32,
    ///
    #[prost(int32, tag = "19")]
    pub player_danmaku_ios_switch_save: i32,
}
/// 弹幕显示区域自动配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmuPlayerDynamicConfig {
    /// 时间
    #[prost(int32, tag = "1")]
    pub progress: i32,
    /// 弹幕显示区域
    #[prost(float, tag = "14")]
    pub player_danmaku_domain: f32,
}
/// 弹幕配置信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmuPlayerViewConfig {
    /// 弹幕默认配置
    #[prost(message, optional, tag = "1")]
    pub danmuku_default_player_config: ::core::option::Option<DanmuDefaultPlayerConfig>,
    /// 弹幕用户配置
    #[prost(message, optional, tag = "2")]
    pub danmuku_player_config: ::core::option::Option<DanmuPlayerConfig>,
    /// 弹幕显示区域自动配置列表
    #[prost(message, repeated, tag = "3")]
    pub danmuku_player_dynamic_config: ::prost::alloc::vec::Vec<
        DanmuPlayerDynamicConfig,
    >,
}
/// 获取弹幕-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegMobileReply {
    /// 弹幕列表
    #[prost(message, repeated, tag = "1")]
    pub elems: ::prost::alloc::vec::Vec<DanmakuElem>,
    /// 是否已关闭弹幕
    /// 0:未关闭 1:已关闭
    #[prost(int32, tag = "2")]
    pub state: i32,
    /// 弹幕云屏蔽ai评分值
    #[prost(message, optional, tag = "3")]
    pub ai_flag: ::core::option::Option<DanmakuAiFlag>,
}
/// 获取弹幕-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegMobileReq {
    /// 稿件avid/漫画epid
    #[prost(int64, tag = "1")]
    pub pid: i64,
    /// 视频cid/漫画cid
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 弹幕类型
    /// 1:视频 2:漫画
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    /// 分段(6min)
    #[prost(int64, tag = "4")]
    pub segment_index: i64,
    /// 是否青少年模式
    #[prost(int32, tag = "5")]
    pub teenagers_mode: i32,
    ///
    #[prost(int64, tag = "6")]
    pub from: i64,
}
/// 客户端弹幕元数据-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmViewReply {
    /// 是否已关闭弹幕
    /// 0:未关闭 1:已关闭
    #[prost(bool, tag = "1")]
    pub closed: bool,
    /// 智能防挡弹幕蒙版信息
    #[prost(message, optional, tag = "2")]
    pub mask: ::core::option::Option<VideoMask>,
    /// 视频字幕
    #[prost(message, optional, tag = "3")]
    pub subtitle: ::core::option::Option<VideoSubtitle>,
    /// 高级弹幕专包url(bfs)
    #[prost(string, repeated, tag = "4")]
    pub special_dms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 云屏蔽配置信息
    #[prost(message, optional, tag = "5")]
    pub ai_flag: ::core::option::Option<DanmakuFlagConfig>,
    /// 弹幕配置信息
    #[prost(message, optional, tag = "6")]
    pub player_config: ::core::option::Option<DanmuPlayerViewConfig>,
    /// 弹幕发送框样式
    #[prost(int32, tag = "7")]
    pub send_box_style: i32,
    /// 是否允许
    #[prost(bool, tag = "8")]
    pub allow: bool,
    /// check box 是否展示
    #[prost(string, tag = "9")]
    pub check_box: ::prost::alloc::string::String,
    /// check box 展示文本
    #[prost(string, tag = "10")]
    pub check_box_show_msg: ::prost::alloc::string::String,
    /// 展示文案
    #[prost(string, tag = "11")]
    pub text_placeholder: ::prost::alloc::string::String,
    /// 弹幕输入框文案
    #[prost(string, tag = "12")]
    pub input_placeholder: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "13")]
    pub command_close: bool,
}
/// 客户端弹幕元数据-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmViewReq {
    /// 稿件avid/漫画epid
    #[prost(int64, tag = "1")]
    pub pid: i64,
    /// 视频cid/漫画cid
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 弹幕类型
    /// 1:视频 2:漫画
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    /// 页面spm
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
    /// 是否冷启
    #[prost(int32, tag = "5")]
    pub is_hard_boot: i32,
    ///
    #[prost(int64, tag = "6")]
    pub from: i64,
}
/// 单个字幕信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubtitleItem {
    /// 字幕id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 字幕id str
    #[prost(string, tag = "2")]
    pub id_str: ::prost::alloc::string::String,
    /// 字幕语言代码
    #[prost(string, tag = "3")]
    pub lan: ::prost::alloc::string::String,
    /// 字幕语言
    #[prost(string, tag = "4")]
    pub lan_doc: ::prost::alloc::string::String,
    /// 字幕文件url
    #[prost(string, tag = "5")]
    pub subtitle_url: ::prost::alloc::string::String,
    /// 字幕作者信息
    #[prost(message, optional, tag = "6")]
    pub author: ::core::option::Option<UserInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TvViewProgressReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub video_guide: ::core::option::Option<VideoGuide>,
    ///
    #[prost(message, optional, tag = "2")]
    pub chronos: ::core::option::Option<Chronos>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TvViewProgressReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
    ///
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
    ///
    #[prost(int64, tag = "7")]
    pub sid: i64,
    ///
    #[prost(int64, tag = "8")]
    pub pid: i64,
    ///
    #[prost(int64, tag = "9")]
    pub from: i64,
    ///
    #[prost(string, tag = "10")]
    pub guest_access_key: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub epid: i64,
}
/// 字幕作者信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// 用户mid
    #[prost(int64, tag = "1")]
    pub mid: i64,
    /// 用户昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 用户性别
    #[prost(string, tag = "3")]
    pub sex: ::prost::alloc::string::String,
    /// 用户头像url
    #[prost(string, tag = "4")]
    pub face: ::prost::alloc::string::String,
    /// 用户签名
    #[prost(string, tag = "5")]
    pub sign: ::prost::alloc::string::String,
    /// 用户等级
    #[prost(int32, tag = "6")]
    pub rank: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoGuide {
    ///
    #[prost(message, repeated, tag = "2")]
    pub command_dms: ::prost::alloc::vec::Vec<CommandDm>,
}
/// 智能防挡弹幕蒙版信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMask {
    /// 视频cid
    #[prost(int64, tag = "1")]
    pub cid: i64,
    /// 平台
    /// 0:web端 1:客户端
    #[prost(int32, tag = "2")]
    pub plat: i32,
    /// 帧率
    #[prost(int32, tag = "3")]
    pub fps: i32,
    /// 间隔时间
    #[prost(int64, tag = "4")]
    pub time: i64,
    /// 蒙版url
    #[prost(string, tag = "5")]
    pub mask_url: ::prost::alloc::string::String,
}
/// 视频字幕信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSubtitle {
    /// 视频原语言代码
    #[prost(string, tag = "1")]
    pub lan: ::prost::alloc::string::String,
    /// 视频原语言
    #[prost(string, tag = "2")]
    pub lan_doc: ::prost::alloc::string::String,
    /// 视频字幕列表
    #[prost(message, repeated, tag = "3")]
    pub subtitles: ::prost::alloc::vec::Vec<SubtitleItem>,
}
