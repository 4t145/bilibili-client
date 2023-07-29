///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Avatar {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "AvatarType", tag = "3")]
    pub avatar_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bubble {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BubbleV2 {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "BubbleType", tag = "3")]
    pub bubble_type: i32,
    ///
    #[prost(bool, tag = "4")]
    pub exposure_once: bool,
    ///
    #[prost(enumeration = "ExposureType", tag = "5")]
    pub exposure_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub action: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuzzwordConfig {
    ///
    #[prost(message, repeated, tag = "1")]
    pub keywords: ::prost::alloc::vec::Vec<BuzzwordShowConfig>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuzzwordShowConfig {
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
    pub id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub buzzword_id: i64,
    ///
    #[prost(int32, tag = "6")]
    pub schema_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBox {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "CheckboxType", tag = "2")]
    pub r#type: i32,
    ///
    #[prost(bool, tag = "3")]
    pub default_value: bool,
    ///
    #[prost(bool, tag = "4")]
    pub show: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBoxV2 {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    ///
    #[prost(bool, tag = "3")]
    pub default_value: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickButton {
    ///
    #[prost(string, repeated, tag = "1")]
    pub portrait_text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "2")]
    pub landscape_text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "3")]
    pub portrait_text_focus: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "4")]
    pub landscape_text_focus: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(enumeration = "RenderType", tag = "5")]
    pub render_type: i32,
    ///
    #[prost(bool, tag = "6")]
    pub show: bool,
    ///
    #[prost(message, optional, tag = "7")]
    pub bubble: ::core::option::Option<Bubble>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickButtonV2 {
    ///
    #[prost(string, repeated, tag = "1")]
    pub portrait_text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "2")]
    pub landscape_text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "3")]
    pub portrait_text_focus: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "4")]
    pub landscape_text_focus: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(int32, tag = "5")]
    pub render_type: i32,
    ///
    #[prost(bool, tag = "6")]
    pub text_input_post: bool,
    ///
    #[prost(bool, tag = "7")]
    pub exposure_once: bool,
    ///
    #[prost(int32, tag = "8")]
    pub exposure_type: i32,
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
    /// 弹幕类型 1 2 3:普通弹幕 4:底部弹幕 5:顶部弹幕 6:逆向弹幕 7:高级弹幕 8:代码弹幕 9:BAS弹幕(pool必须为2)
    #[prost(int32, tag = "3")]
    pub mode: i32,
    /// 弹幕字号
    #[prost(int32, tag = "4")]
    pub fontsize: i32,
    /// 弹幕颜色
    #[prost(uint32, tag = "5")]
    pub color: u32,
    /// 发送者mid hash
    #[prost(string, tag = "6")]
    pub mid_hash: ::prost::alloc::string::String,
    /// 弹幕正文
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
    /// 发送时间
    #[prost(int64, tag = "8")]
    pub ctime: i64,
    /// 权重 用于屏蔽等级 区间:\[1,10\]
    #[prost(int32, tag = "9")]
    pub weight: i32,
    /// 动作
    #[prost(string, tag = "10")]
    pub action: ::prost::alloc::string::String,
    /// 弹幕池 0:普通池 1:字幕池 2:特殊池(代码/BAS弹幕)
    #[prost(int32, tag = "11")]
    pub pool: i32,
    /// 弹幕dmid str
    #[prost(string, tag = "12")]
    pub id_str: ::prost::alloc::string::String,
    /// 弹幕属性位(bin求AND)
    /// bit0:保护 bit1:直播 bit2:高赞
    #[prost(int32, tag = "13")]
    pub attr: i32,
    ///
    #[prost(string, tag = "22")]
    pub animation: ::prost::alloc::string::String,
    /// 大会员专属颜色
    #[prost(enumeration = "DmColorfulType", tag = "24")]
    pub colorful: i32,
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
    ///
    #[prost(int32, tag = "18")]
    pub player_danmaku_ai_recommended_level_v2: i32,
    ///
    #[prost(map = "int32, int32", tag = "19")]
    pub player_danmaku_ai_recommended_level_v2_map: ::std::collections::HashMap<
        i32,
        i32,
    >,
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
    ///
    #[prost(int32, tag = "20")]
    pub player_danmaku_senior_mode_switch: i32,
    ///
    #[prost(int32, tag = "21")]
    pub player_danmaku_ai_recommended_level_v2: i32,
    ///
    #[prost(map = "int32, int32", tag = "22")]
    pub player_danmaku_ai_recommended_level_v2_map: ::std::collections::HashMap<
        i32,
        i32,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmuPlayerConfigPanel {
    ///
    #[prost(string, tag = "1")]
    pub selection_text: ::prost::alloc::string::String,
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
    ///
    #[prost(message, optional, tag = "4")]
    pub danmuku_player_config_panel: ::core::option::Option<DanmuPlayerConfigPanel>,
}
/// web端用户弹幕配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmuWebPlayerConfig {
    /// 是否开启弹幕
    #[prost(bool, tag = "1")]
    pub dm_switch: bool,
    /// 是否开启智能云屏蔽
    #[prost(bool, tag = "2")]
    pub ai_switch: bool,
    /// 智能云屏蔽等级
    #[prost(int32, tag = "3")]
    pub ai_level: i32,
    /// 是否屏蔽顶端弹幕
    #[prost(bool, tag = "4")]
    pub blocktop: bool,
    /// 是否屏蔽滚动弹幕
    #[prost(bool, tag = "5")]
    pub blockscroll: bool,
    /// 是否屏蔽底端弹幕
    #[prost(bool, tag = "6")]
    pub blockbottom: bool,
    /// 是否屏蔽彩色弹幕
    #[prost(bool, tag = "7")]
    pub blockcolor: bool,
    /// 是否屏蔽重复弹幕
    #[prost(bool, tag = "8")]
    pub blockspecial: bool,
    ///
    #[prost(bool, tag = "9")]
    pub preventshade: bool,
    ///
    #[prost(bool, tag = "10")]
    pub dmask: bool,
    ///
    #[prost(float, tag = "11")]
    pub opacity: f32,
    ///
    #[prost(int32, tag = "12")]
    pub dmarea: i32,
    ///
    #[prost(float, tag = "13")]
    pub speedplus: f32,
    /// 弹幕字号
    #[prost(float, tag = "14")]
    pub fontsize: f32,
    ///
    #[prost(bool, tag = "15")]
    pub screensync: bool,
    ///
    #[prost(bool, tag = "16")]
    pub speedsync: bool,
    ///
    #[prost(string, tag = "17")]
    pub fontfamily: ::prost::alloc::string::String,
    /// 是否使用加粗
    #[prost(bool, tag = "18")]
    pub bold: bool,
    ///
    #[prost(int32, tag = "19")]
    pub fontborder: i32,
    /// 弹幕渲染类型
    #[prost(string, tag = "20")]
    pub draw_type: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "21")]
    pub senior_mode_switch: i32,
    ///
    #[prost(int32, tag = "22")]
    pub ai_level_v2: i32,
    ///
    #[prost(map = "int32, int32", tag = "23")]
    pub ai_level_v2_map: ::std::collections::HashMap<i32, i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmColorful {
    /// 颜色类型
    #[prost(enumeration = "DmColorfulType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub src: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmExpoReportReq {
    ///
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub oid: i64,
    ///
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmExpoReportRes {}
/// 修改弹幕配置-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmPlayerConfigReq {
    ///
    #[prost(int64, tag = "1")]
    pub ts: i64,
    /// 是否开启弹幕
    #[prost(message, optional, tag = "2")]
    pub switch: ::core::option::Option<PlayerDanmakuSwitch>,
    /// 是否记录弹幕开关设置
    #[prost(message, optional, tag = "3")]
    pub switch_save: ::core::option::Option<PlayerDanmakuSwitchSave>,
    /// 是否使用推荐弹幕设置
    #[prost(message, optional, tag = "4")]
    pub use_default_config: ::core::option::Option<PlayerDanmakuUseDefaultConfig>,
    /// 是否开启智能云屏蔽
    #[prost(message, optional, tag = "5")]
    pub ai_recommended_switch: ::core::option::Option<PlayerDanmakuAiRecommendedSwitch>,
    /// 智能云屏蔽等级
    #[prost(message, optional, tag = "6")]
    pub ai_recommended_level: ::core::option::Option<PlayerDanmakuAiRecommendedLevel>,
    /// 是否屏蔽顶端弹幕
    #[prost(message, optional, tag = "7")]
    pub blocktop: ::core::option::Option<PlayerDanmakuBlocktop>,
    /// 是否屏蔽滚动弹幕
    #[prost(message, optional, tag = "8")]
    pub blockscroll: ::core::option::Option<PlayerDanmakuBlockscroll>,
    /// 是否屏蔽底端弹幕
    #[prost(message, optional, tag = "9")]
    pub blockbottom: ::core::option::Option<PlayerDanmakuBlockbottom>,
    /// 是否屏蔽彩色弹幕
    #[prost(message, optional, tag = "10")]
    pub blockcolorful: ::core::option::Option<PlayerDanmakuBlockcolorful>,
    /// 是否屏蔽重复弹幕
    #[prost(message, optional, tag = "11")]
    pub blockrepeat: ::core::option::Option<PlayerDanmakuBlockrepeat>,
    /// 是否屏蔽高级弹幕
    #[prost(message, optional, tag = "12")]
    pub blockspecial: ::core::option::Option<PlayerDanmakuBlockspecial>,
    /// 弹幕不透明度
    #[prost(message, optional, tag = "13")]
    pub opacity: ::core::option::Option<PlayerDanmakuOpacity>,
    /// 弹幕缩放比例
    #[prost(message, optional, tag = "14")]
    pub scalingfactor: ::core::option::Option<PlayerDanmakuScalingfactor>,
    /// 弹幕显示区域
    #[prost(message, optional, tag = "15")]
    pub domain: ::core::option::Option<PlayerDanmakuDomain>,
    /// 弹幕速度
    #[prost(message, optional, tag = "16")]
    pub speed: ::core::option::Option<PlayerDanmakuSpeed>,
    /// 是否开启屏蔽列表
    #[prost(message, optional, tag = "17")]
    pub enableblocklist: ::core::option::Option<PlayerDanmakuEnableblocklist>,
    /// 是否开启弹幕
    #[prost(message, optional, tag = "18")]
    pub inline_player_danmaku_switch: ::core::option::Option<InlinePlayerDanmakuSwitch>,
    ///
    #[prost(message, optional, tag = "19")]
    pub senior_mode_switch: ::core::option::Option<PlayerDanmakuSeniorModeSwitch>,
    ///
    #[prost(message, optional, tag = "20")]
    pub ai_recommended_level_v2: ::core::option::Option<
        PlayerDanmakuAiRecommendedLevelV2,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegConfig {
    ///
    #[prost(int64, tag = "1")]
    pub page_size: i64,
    ///
    #[prost(int64, tag = "2")]
    pub total: i64,
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
    #[prost(message, repeated, tag = "5")]
    pub colorful_src: ::prost::alloc::vec::Vec<DmColorful>,
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
    pub ps: i64,
    ///
    #[prost(int64, tag = "7")]
    pub pe: i64,
    ///
    #[prost(int32, tag = "8")]
    pub pull_mode: i32,
    ///
    #[prost(int32, tag = "9")]
    pub from_scene: i32,
}
/// ott弹幕列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegOttReply {
    /// 是否已关闭弹幕
    /// 0:未关闭 1:已关闭
    #[prost(bool, tag = "1")]
    pub closed: bool,
    /// 弹幕列表
    #[prost(message, repeated, tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<DanmakuElem>,
}
/// ott弹幕列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegOttReq {
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
}
/// 弹幕SDK-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegSdkReply {
    /// 是否已关闭弹幕
    /// 0:未关闭 1:已关闭
    #[prost(bool, tag = "1")]
    pub closed: bool,
    /// 弹幕列表
    #[prost(message, repeated, tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<DanmakuElem>,
}
/// 弹幕SDK-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmSegSdkReq {
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
    /// 用户举报弹幕 cid维度屏蔽的正则规则
    #[prost(string, repeated, tag = "13")]
    pub report_filter_content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(message, optional, tag = "14")]
    pub expo_report: ::core::option::Option<ExpoReport>,
    ///
    #[prost(message, optional, tag = "15")]
    pub buzzword_config: ::core::option::Option<BuzzwordConfig>,
    ///
    #[prost(message, repeated, tag = "16")]
    pub expressions: ::prost::alloc::vec::Vec<Expressions>,
    ///
    #[prost(message, repeated, tag = "17")]
    pub post_panel: ::prost::alloc::vec::Vec<PostPanel>,
    ///
    #[prost(string, repeated, tag = "18")]
    pub activity_meta: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(message, repeated, tag = "19")]
    pub post_panel2: ::prost::alloc::vec::Vec<PostPanelV2>,
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
}
/// web端弹幕元数据-响应
/// <https://api.bilibili.com/x/v2/dm/web/view>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DmWebViewReply {
    /// 是否已关闭弹幕
    /// 0:未关闭 1:已关闭
    #[prost(int32, tag = "1")]
    pub state: i32,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_side: ::prost::alloc::string::String,
    /// 分段弹幕配置
    #[prost(message, optional, tag = "4")]
    pub dm_sge: ::core::option::Option<DmSegConfig>,
    /// 云屏蔽配置信息
    #[prost(message, optional, tag = "5")]
    pub flag: ::core::option::Option<DanmakuFlagConfig>,
    /// 高级弹幕专包url(bfs)
    #[prost(string, repeated, tag = "6")]
    pub special_dms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// check box 是否展示
    #[prost(bool, tag = "7")]
    pub check_box: bool,
    /// 弹幕数
    #[prost(int64, tag = "8")]
    pub count: i64,
    /// 互动弹幕
    #[prost(message, repeated, tag = "9")]
    pub command_dms: ::prost::alloc::vec::Vec<CommandDm>,
    /// 用户弹幕配置
    #[prost(message, optional, tag = "10")]
    pub player_config: ::core::option::Option<DanmuWebPlayerConfig>,
    /// 用户举报弹幕 cid维度屏蔽
    #[prost(string, repeated, tag = "11")]
    pub report_filter_content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(message, repeated, tag = "12")]
    pub expressions: ::prost::alloc::vec::Vec<Expressions>,
    ///
    #[prost(message, repeated, tag = "13")]
    pub post_panel: ::prost::alloc::vec::Vec<PostPanel>,
    ///
    #[prost(string, repeated, tag = "14")]
    pub activity_meta: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpoReport {
    ///
    #[prost(bool, tag = "1")]
    pub should_report_at_end: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expression {
    ///
    #[prost(string, repeated, tag = "1")]
    pub keyword: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub period: ::prost::alloc::vec::Vec<Period>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Expressions {
    ///
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<Expression>,
}
/// 是否开启弹幕
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlinePlayerDanmakuSwitch {
    ///
    #[prost(bool, tag = "1")]
    pub value: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "2")]
    pub content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelV2 {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "2")]
    pub content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(bool, tag = "3")]
    pub exposure_once: bool,
    ///
    #[prost(int32, tag = "4")]
    pub exposure_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    ///
    #[prost(int64, tag = "1")]
    pub start: i64,
    ///
    #[prost(int64, tag = "2")]
    pub end: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuAiRecommendedLevel {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuAiRecommendedLevelV2 {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuAiRecommendedSwitch {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuBlockbottom {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuBlockcolorful {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuBlockrepeat {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuBlockscroll {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuBlockspecial {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuBlocktop {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuDomain {
    #[prost(float, tag = "1")]
    pub value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuEnableblocklist {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuOpacity {
    #[prost(float, tag = "1")]
    pub value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuScalingfactor {
    #[prost(float, tag = "1")]
    pub value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuSeniorModeSwitch {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuSpeed {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuSwitch {
    #[prost(bool, tag = "1")]
    pub value: bool,
    #[prost(bool, tag = "2")]
    pub can_ignore: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuSwitchSave {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDanmakuUseDefaultConfig {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostPanel {
    ///
    #[prost(int64, tag = "1")]
    pub start: i64,
    ///
    #[prost(int64, tag = "2")]
    pub end: i64,
    ///
    #[prost(int64, tag = "3")]
    pub priority: i64,
    ///
    #[prost(int64, tag = "4")]
    pub biz_id: i64,
    ///
    #[prost(enumeration = "PostPanelBizType", tag = "5")]
    pub biz_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub click_button: ::core::option::Option<ClickButton>,
    ///
    #[prost(message, optional, tag = "7")]
    pub text_input: ::core::option::Option<TextInput>,
    ///
    #[prost(message, optional, tag = "8")]
    pub check_box: ::core::option::Option<CheckBox>,
    ///
    #[prost(message, optional, tag = "9")]
    pub toast: ::core::option::Option<Toast>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostPanelV2 {
    ///
    #[prost(int64, tag = "1")]
    pub start: i64,
    ///
    #[prost(int64, tag = "2")]
    pub end: i64,
    ///
    #[prost(int32, tag = "3")]
    pub biz_type: i32,
    ///
    #[prost(message, optional, tag = "4")]
    pub click_button: ::core::option::Option<ClickButtonV2>,
    ///
    #[prost(message, optional, tag = "5")]
    pub text_input: ::core::option::Option<TextInputV2>,
    ///
    #[prost(message, optional, tag = "6")]
    pub check_box: ::core::option::Option<CheckBoxV2>,
    ///
    #[prost(message, optional, tag = "7")]
    pub toast: ::core::option::Option<ToastV2>,
    ///
    #[prost(message, optional, tag = "8")]
    pub bubble: ::core::option::Option<BubbleV2>,
    ///
    #[prost(message, optional, tag = "9")]
    pub label: ::core::option::Option<LabelV2>,
    ///
    #[prost(int32, tag = "10")]
    pub post_status: i32,
}
/// 修改弹幕配置-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    ///
    #[prost(int32, tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
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
    /// 字幕类型
    #[prost(enumeration = "SubtitleType", tag = "7")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "8")]
    pub lan_doc_brief: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "SubtitleAiType", tag = "9")]
    pub ai_type: i32,
    ///
    #[prost(enumeration = "SubtitleAiStatus", tag = "10")]
    pub ai_status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    ///
    #[prost(string, repeated, tag = "1")]
    pub portrait_placeholder: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "2")]
    pub landscape_placeholder: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(enumeration = "RenderType", tag = "3")]
    pub render_type: i32,
    ///
    #[prost(bool, tag = "4")]
    pub placeholder_post: bool,
    ///
    #[prost(bool, tag = "5")]
    pub show: bool,
    ///
    #[prost(message, repeated, tag = "6")]
    pub avatar: ::prost::alloc::vec::Vec<Avatar>,
    ///
    #[prost(enumeration = "PostStatus", tag = "7")]
    pub post_status: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub label: ::core::option::Option<Label>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInputV2 {
    ///
    #[prost(string, repeated, tag = "1")]
    pub portrait_placeholder: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, repeated, tag = "2")]
    pub landscape_placeholder: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(enumeration = "RenderType", tag = "3")]
    pub render_type: i32,
    ///
    #[prost(bool, tag = "4")]
    pub placeholder_post: bool,
    ///
    #[prost(message, repeated, tag = "5")]
    pub avatar: ::prost::alloc::vec::Vec<Avatar>,
    ///
    #[prost(int32, tag = "6")]
    pub text_input_limit: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Toast {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub duration: i32,
    ///
    #[prost(bool, tag = "3")]
    pub show: bool,
    ///
    #[prost(message, optional, tag = "4")]
    pub button: ::core::option::Option<Button>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToastButtonV2 {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub action: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToastV2 {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub duration: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub toast_button_v2: ::core::option::Option<ToastButtonV2>,
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
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AvatarType {
    ///
    None = 0,
    ///
    Nft = 1,
}
impl AvatarType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AvatarType::None => "AvatarTypeNone",
            AvatarType::Nft => "AvatarTypeNFT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AvatarTypeNone" => Some(Self::None),
            "AvatarTypeNFT" => Some(Self::Nft),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BubbleType {
    ///
    None = 0,
    ///
    ClickButton = 1,
    ///
    DmSettingPanel = 2,
}
impl BubbleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BubbleType::None => "BubbleTypeNone",
            BubbleType::ClickButton => "BubbleTypeClickButton",
            BubbleType::DmSettingPanel => "BubbleTypeDmSettingPanel",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BubbleTypeNone" => Some(Self::None),
            "BubbleTypeClickButton" => Some(Self::ClickButton),
            "BubbleTypeDmSettingPanel" => Some(Self::DmSettingPanel),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckboxType {
    ///
    None = 0,
    ///
    Encourage = 1,
    ///
    ColorDm = 2,
}
impl CheckboxType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CheckboxType::None => "CheckboxTypeNone",
            CheckboxType::Encourage => "CheckboxTypeEncourage",
            CheckboxType::ColorDm => "CheckboxTypeColorDM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CheckboxTypeNone" => Some(Self::None),
            "CheckboxTypeEncourage" => Some(Self::Encourage),
            "CheckboxTypeColorDM" => Some(Self::ColorDm),
            _ => None,
        }
    }
}
/// 弹幕属性位值
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DmAttrBit {
    /// 保护弹幕
    Protect = 0,
    /// 直播弹幕
    FromLive = 1,
    /// 高赞弹幕
    DmAttrHighLike = 2,
}
impl DmAttrBit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DmAttrBit::Protect => "DMAttrBitProtect",
            DmAttrBit::FromLive => "DMAttrBitFromLive",
            DmAttrBit::DmAttrHighLike => "DMAttrHighLike",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DMAttrBitProtect" => Some(Self::Protect),
            "DMAttrBitFromLive" => Some(Self::FromLive),
            "DMAttrHighLike" => Some(Self::DmAttrHighLike),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DmColorfulType {
    /// 无
    NoneType = 0,
    /// 渐变色
    VipGradualColor = 60001,
}
impl DmColorfulType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DmColorfulType::NoneType => "NoneType",
            DmColorfulType::VipGradualColor => "VipGradualColor",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoneType" => Some(Self::NoneType),
            "VipGradualColor" => Some(Self::VipGradualColor),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExposureType {
    ///
    None = 0,
    ///
    DmSend = 1,
}
impl ExposureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExposureType::None => "ExposureTypeNone",
            ExposureType::DmSend => "ExposureTypeDMSend",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ExposureTypeNone" => Some(Self::None),
            "ExposureTypeDMSend" => Some(Self::DmSend),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostPanelBizType {
    ///
    None = 0,
    ///
    Encourage = 1,
    ///
    ColorDm = 2,
    ///
    Nftdm = 3,
    ///
    FragClose = 4,
    ///
    Recommend = 5,
}
impl PostPanelBizType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PostPanelBizType::None => "PostPanelBizTypeNone",
            PostPanelBizType::Encourage => "PostPanelBizTypeEncourage",
            PostPanelBizType::ColorDm => "PostPanelBizTypeColorDM",
            PostPanelBizType::Nftdm => "PostPanelBizTypeNFTDM",
            PostPanelBizType::FragClose => "PostPanelBizTypeFragClose",
            PostPanelBizType::Recommend => "PostPanelBizTypeRecommend",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PostPanelBizTypeNone" => Some(Self::None),
            "PostPanelBizTypeEncourage" => Some(Self::Encourage),
            "PostPanelBizTypeColorDM" => Some(Self::ColorDm),
            "PostPanelBizTypeNFTDM" => Some(Self::Nftdm),
            "PostPanelBizTypeFragClose" => Some(Self::FragClose),
            "PostPanelBizTypeRecommend" => Some(Self::Recommend),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostStatus {
    ///
    Normal = 0,
    ///
    Closed = 1,
}
impl PostStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PostStatus::Normal => "PostStatusNormal",
            PostStatus::Closed => "PostStatusClosed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PostStatusNormal" => Some(Self::Normal),
            "PostStatusClosed" => Some(Self::Closed),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RenderType {
    ///
    None = 0,
    ///
    Single = 1,
    ///
    Rotation = 2,
}
impl RenderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RenderType::None => "RenderTypeNone",
            RenderType::Single => "RenderTypeSingle",
            RenderType::Rotation => "RenderTypeRotation",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RenderTypeNone" => Some(Self::None),
            "RenderTypeSingle" => Some(Self::Single),
            "RenderTypeRotation" => Some(Self::Rotation),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubtitleAiStatus {
    ///
    None = 0,
    ///
    Exposure = 1,
    ///
    Assist = 2,
}
impl SubtitleAiStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubtitleAiStatus::None => "None",
            SubtitleAiStatus::Exposure => "Exposure",
            SubtitleAiStatus::Assist => "Assist",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "Exposure" => Some(Self::Exposure),
            "Assist" => Some(Self::Assist),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubtitleAiType {
    ///
    Normal = 0,
    ///
    Translate = 1,
}
impl SubtitleAiType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubtitleAiType::Normal => "Normal",
            SubtitleAiType::Translate => "Translate",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Normal" => Some(Self::Normal),
            "Translate" => Some(Self::Translate),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubtitleType {
    /// CC字幕
    Cc = 0,
    /// AI生成字幕
    Ai = 1,
}
impl SubtitleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubtitleType::Cc => "CC",
            SubtitleType::Ai => "AI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CC" => Some(Self::Cc),
            "AI" => Some(Self::Ai),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ToastFunctionType {
    ///
    None = 0,
    ///
    PostPanel = 1,
}
impl ToastFunctionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ToastFunctionType::None => "ToastFunctionTypeNone",
            ToastFunctionType::PostPanel => "ToastFunctionTypePostPanel",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ToastFunctionTypeNone" => Some(Self::None),
            "ToastFunctionTypePostPanel" => Some(Self::PostPanel),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod dm_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 弹幕
    #[derive(Debug, Clone)]
    pub struct DmClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DmClient<tonic::transport::Channel> {
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
    impl<T> DmClient<T>
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
        ) -> DmClient<InterceptedService<T, F>>
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
            DmClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取分段弹幕
        pub async fn dm_seg_mobile(
            &mut self,
            request: impl tonic::IntoRequest<super::DmSegMobileReq>,
        ) -> std::result::Result<
            tonic::Response<super::DmSegMobileReply>,
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
                "/bilibili.community.service.dm.v1.DM/DmSegMobile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.community.service.dm.v1.DM", "DmSegMobile"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 客户端弹幕元数据 字幕、分段、防挡蒙版等
        pub async fn dm_view(
            &mut self,
            request: impl tonic::IntoRequest<super::DmViewReq>,
        ) -> std::result::Result<tonic::Response<super::DmViewReply>, tonic::Status> {
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
                "/bilibili.community.service.dm.v1.DM/DmView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.community.service.dm.v1.DM", "DmView"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 修改弹幕配置
        pub async fn dm_player_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DmPlayerConfigReq>,
        ) -> std::result::Result<tonic::Response<super::Response>, tonic::Status> {
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
                "/bilibili.community.service.dm.v1.DM/DmPlayerConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.community.service.dm.v1.DM",
                        "DmPlayerConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ott弹幕列表
        pub async fn dm_seg_ott(
            &mut self,
            request: impl tonic::IntoRequest<super::DmSegOttReq>,
        ) -> std::result::Result<tonic::Response<super::DmSegOttReply>, tonic::Status> {
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
                "/bilibili.community.service.dm.v1.DM/DmSegOtt",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.community.service.dm.v1.DM", "DmSegOtt"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SDK弹幕列表
        pub async fn dm_seg_sdk(
            &mut self,
            request: impl tonic::IntoRequest<super::DmSegSdkReq>,
        ) -> std::result::Result<tonic::Response<super::DmSegSdkReply>, tonic::Status> {
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
                "/bilibili.community.service.dm.v1.DM/DmSegSDK",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.community.service.dm.v1.DM", "DmSegSDK"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dm_expo_report(
            &mut self,
            request: impl tonic::IntoRequest<super::DmExpoReportReq>,
        ) -> std::result::Result<
            tonic::Response<super::DmExpoReportRes>,
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
                "/bilibili.community.service.dm.v1.DM/DmExpoReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.community.service.dm.v1.DM",
                        "DmExpoReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
