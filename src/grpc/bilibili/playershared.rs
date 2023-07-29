/// ArcConf消息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArcConf {
    /// 是否支持
    #[prost(bool, tag = "1")]
    pub is_support: bool,
    /// 是否禁用
    #[prost(bool, tag = "2")]
    pub disabled: bool,
    /// 额外内容
    #[prost(message, optional, tag = "3")]
    pub extra_content: ::core::option::Option<ExtraContent>,
    /// 不支持场景列表
    #[prost(int32, repeated, tag = "4")]
    pub unsupport_scene: ::prost::alloc::vec::Vec<i32>,
}
/// 按钮组件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    /// 按钮文本
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 按钮跳转链接
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
    /// 埋点上报相关
    #[prost(map = "string, string", tag = "3")]
    pub report_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfValue {
    #[prost(oneof = "conf_value::Value", tags = "1, 2")]
    pub value: ::core::option::Option<conf_value::Value>,
}
/// Nested message and enum types in `ConfValue`.
pub mod conf_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        ///
        #[prost(int32, tag = "1")]
        SwitchVal(i32),
        ///
        #[prost(int32, tag = "2")]
        SelectedVal(i32),
    }
}
/// Dash条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashItem {
    /// 清晰度
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// 主线流
    #[prost(string, tag = "2")]
    pub base_url: ::prost::alloc::string::String,
    /// 备用流
    #[prost(string, repeated, tag = "3")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 带宽
    #[prost(uint32, tag = "4")]
    pub bandwidth: u32,
    /// 编码id
    #[prost(uint32, tag = "5")]
    pub codecid: u32,
    /// md5
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
    /// 大小
    #[prost(uint64, tag = "7")]
    pub size: u64,
    /// 帧率
    #[prost(string, tag = "8")]
    pub frame_rate: ::prost::alloc::string::String,
    /// DRM密钥
    #[prost(string, tag = "9")]
    pub widevine_pssh: ::prost::alloc::string::String,
}
/// 视频流信息: dash流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashVideo {
    /// 主线流
    #[prost(string, tag = "1")]
    pub base_url: ::prost::alloc::string::String,
    /// 备用流
    #[prost(string, repeated, tag = "2")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 带宽
    #[prost(uint32, tag = "3")]
    pub bandwidth: u32,
    /// 编码id
    #[prost(uint32, tag = "4")]
    pub codecid: u32,
    /// md5
    #[prost(string, tag = "5")]
    pub md5: ::prost::alloc::string::String,
    /// 大小
    #[prost(uint64, tag = "6")]
    pub size: u64,
    /// 伴音质量id
    #[prost(uint32, tag = "7")]
    pub audio_id: u32,
    /// 是否非全二压
    #[prost(bool, tag = "8")]
    pub no_rexcode: bool,
    /// 帧率
    #[prost(string, tag = "9")]
    pub frame_rate: ::prost::alloc::string::String,
    /// 宽
    #[prost(int32, tag = "10")]
    pub width: i32,
    /// 高
    #[prost(int32, tag = "11")]
    pub height: i32,
    /// DRM密钥
    #[prost(string, tag = "12")]
    pub widevine_pssh: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceConf {
    #[prost(message, optional, tag = "1")]
    pub conf_value: ::core::option::Option<ConfValue>,
}
/// 当前分辨率信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    /// 宽
    #[prost(int32, tag = "1")]
    pub width: i32,
    /// 长
    #[prost(int32, tag = "2")]
    pub height: i32,
    /// 旋转角度
    #[prost(int32, tag = "3")]
    pub rotate: i32,
}
/// 杜比伴音流信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DolbyItem {
    /// 杜比类型
    #[prost(enumeration = "dolby_item::Type", tag = "1")]
    pub r#type: i32,
    /// 音频流
    #[prost(message, repeated, tag = "2")]
    pub audio: ::prost::alloc::vec::Vec<DashItem>,
}
/// Nested message and enum types in `DolbyItem`.
pub mod dolby_item {
    /// 杜比类型
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
    pub enum Type {
        /// NONE
        None = 0,
        /// 普通杜比音效
        Common = 1,
        /// 全景杜比音效
        Atmos = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::None => "NONE",
                Type::Common => "COMMON",
                Type::Atmos => "ATMOS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "COMMON" => Some(Self::Common),
                "ATMOS" => Some(Self::Atmos),
                _ => None,
            }
        }
    }
}
/// 事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// 震动
    #[prost(message, optional, tag = "1")]
    pub shake: ::core::option::Option<Shake>,
}
/// ? 错误码补充信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraContent {
    ///
    #[prost(string, tag = "1")]
    pub disable_reason: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub disable_code: i64,
}
/// 播放历史
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct History {
    ///
    #[prost(message, optional, tag = "1")]
    pub current_video: ::core::option::Option<HistoryInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub related_video: ::core::option::Option<HistoryInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoryInfo {
    ///
    #[prost(int64, tag = "1")]
    pub progress: i64,
    ///
    #[prost(int64, tag = "2")]
    pub last_play_cid: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub toast: ::core::option::Option<Toast>,
    ///
    #[prost(message, optional, tag = "4")]
    pub toast_without_time: ::core::option::Option<Toast>,
    ///
    #[prost(int64, tag = "5")]
    pub last_play_aid: i64,
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
    #[prost(int64, tag = "4")]
    pub mark: i64,
}
/// HIRES伴音流信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LossLessItem {
    /// 是否为hires
    #[prost(bool, tag = "1")]
    pub is_lossless_audio: bool,
    /// 音频流信息
    #[prost(message, optional, tag = "2")]
    pub audio: ::core::option::Option<DashItem>,
    /// 是否需要大会员
    #[prost(bool, tag = "3")]
    pub need_vip: bool,
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
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayArc {
    ///
    #[prost(enumeration = "VideoType", tag = "1")]
    pub video_type: i32,
    ///
    #[prost(uint64, tag = "2")]
    pub aid: u64,
    ///
    #[prost(uint64, tag = "3")]
    pub cid: u64,
    ///
    #[prost(enumeration = "DrmTechType", tag = "4")]
    pub drm_tech_type: i32,
    ///
    #[prost(enumeration = "ArcType", tag = "5")]
    pub arc_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub interaction: ::core::option::Option<Interaction>,
    ///
    #[prost(message, optional, tag = "7")]
    pub dimension: ::core::option::Option<Dimension>,
}
/// 播放页信息-响应: PlayArcConf
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayArcConf {
    #[prost(map = "int32, message", tag = "1")]
    pub arc_confs: ::std::collections::HashMap<i32, ArcConf>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayDeviceConf {
    ///
    #[prost(map = "int32, message", tag = "1")]
    pub device_confs: ::std::collections::HashMap<i32, DeviceConf>,
}
/// 播放页信息-响应: 高画质试看信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QnTrialInfo {
    /// 能否试看高画质
    #[prost(bool, tag = "1")]
    pub trial_able: bool,
    ///
    #[prost(int32, tag = "2")]
    pub remaining_times: i32,
    ///
    #[prost(int32, tag = "3")]
    pub start: i32,
    ///
    #[prost(int32, tag = "4")]
    pub time_length: i32,
    ///
    #[prost(message, optional, tag = "5")]
    pub start_toast: ::core::option::Option<Toast>,
    ///
    #[prost(message, optional, tag = "6")]
    pub end_toast: ::core::option::Option<Toast>,
    ///
    #[prost(message, optional, tag = "8")]
    pub quality_open_tip_btn: ::core::option::Option<Button>,
}
/// Dash Response, 未使用
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDash {
    #[prost(message, repeated, tag = "1")]
    pub video: ::prost::alloc::vec::Vec<DashItem>,
    #[prost(message, repeated, tag = "2")]
    pub audio: ::prost::alloc::vec::Vec<DashItem>,
}
/// 分段流条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseUrl {
    /// 分段序号
    #[prost(uint32, tag = "1")]
    pub order: u32,
    /// 分段时长
    #[prost(uint64, tag = "2")]
    pub length: u64,
    /// 分段大小
    #[prost(uint64, tag = "3")]
    pub size: u64,
    /// 主线流
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    /// 备用流
    #[prost(string, repeated, tag = "5")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// md5
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
}
/// 方案
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scheme {
    ///
    #[prost(enumeration = "scheme::ActionType", tag = "1")]
    pub action_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub toast: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Scheme`.
pub mod scheme {
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
    pub enum ActionType {
        Unknown = 0,
        ShowToast = 1,
    }
    impl ActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ActionType::Unknown => "UNKNOWN",
                ActionType::ShowToast => "SHOW_TOAST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SHOW_TOAST" => Some(Self::ShowToast),
                _ => None,
            }
        }
    }
}
/// 视频流信息: 分段流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentVideo {
    #[prost(message, repeated, tag = "1")]
    pub segment: ::prost::alloc::vec::Vec<ResponseUrl>,
}
/// 震动
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shake {
    ///
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
}
/// 视频流信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// 元数据
    #[prost(message, optional, tag = "1")]
    pub stream_info: ::core::option::Option<StreamInfo>,
    /// 流数据
    #[prost(oneof = "stream::Content", tags = "2, 3")]
    pub content: ::core::option::Option<stream::Content>,
}
/// Nested message and enum types in `Stream`.
pub mod stream {
    /// 流数据
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// dash流
        #[prost(message, tag = "2")]
        DashVideo(super::DashVideo),
        /// 分段流
        #[prost(message, tag = "3")]
        SegmentVideo(super::SegmentVideo),
    }
}
/// 视频流信息: 元数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfo {
    /// 清晰度
    #[prost(uint32, tag = "1")]
    pub quality: u32,
    /// 格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 格式描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// 错误码
    #[prost(uint32, tag = "4")]
    pub err_code: u32,
    /// 不满足条件信息
    #[prost(message, optional, tag = "5")]
    pub limit: ::core::option::Option<StreamLimit>,
    /// 是否需要vip
    #[prost(bool, tag = "6")]
    pub need_vip: bool,
    /// 是否需要登录
    #[prost(bool, tag = "7")]
    pub need_login: bool,
    /// 是否完整
    #[prost(bool, tag = "8")]
    pub intact: bool,
    /// 是否非全二压
    #[prost(bool, tag = "9")]
    pub no_rexcode: bool,
    /// 清晰度属性位
    #[prost(int64, tag = "10")]
    pub attribute: i64,
    /// 新版格式描述
    #[prost(string, tag = "11")]
    pub new_description: ::prost::alloc::string::String,
    /// 格式文字
    #[prost(string, tag = "12")]
    pub display_desc: ::prost::alloc::string::String,
    /// 新版格式描述备注
    #[prost(string, tag = "13")]
    pub superscript: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "14")]
    pub vip_free: bool,
    ///
    #[prost(string, tag = "15")]
    pub subtitle: ::prost::alloc::string::String,
    /// 方案
    #[prost(message, optional, tag = "16")]
    pub scheme: ::core::option::Option<Scheme>,
    /// 支持drm
    #[prost(bool, tag = "17")]
    pub support_drm: bool,
}
/// 视频流信息: 流媒体元数据: 清晰度不满足条件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLimit {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 提示信息
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
}
/// Toast信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Toast {
    /// toast文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// toast按钮
    #[prost(message, optional, tag = "2")]
    pub button: ::core::option::Option<Button>,
}
/// 播放页信息-请求: 音视频VOD
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoVod {
    /// 视频aid
    #[prost(int32, tag = "1")]
    pub aid: i32,
    /// 视频cid
    #[prost(int32, tag = "2")]
    pub cid: i32,
    /// 清晰度
    #[prost(uint64, tag = "3")]
    pub qn: u64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 视频编码
    #[prost(enumeration = "CodeType", tag = "9")]
    pub prefer_codec_type: i32,
    /// 响度均衡
    #[prost(uint64, tag = "10")]
    pub voice_balance: u64,
}
/// 播放页信息-响应: VOD音视频信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VodInfo {
    /// 视频清晰度
    #[prost(uint32, tag = "1")]
    pub quality: u32,
    /// 视频格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 视频时长
    #[prost(uint64, tag = "3")]
    pub timelength: u64,
    /// 视频编码id
    #[prost(uint32, tag = "4")]
    pub video_codecid: u32,
    /// 视频流
    #[prost(message, repeated, tag = "5")]
    pub stream_list: ::prost::alloc::vec::Vec<Stream>,
    /// 伴音流
    #[prost(message, repeated, tag = "6")]
    pub dash_audio: ::prost::alloc::vec::Vec<DashItem>,
    /// 杜比伴音流
    #[prost(message, optional, tag = "7")]
    pub dolby: ::core::option::Option<DolbyItem>,
    /// 响度均衡操作信息
    #[prost(message, optional, tag = "8")]
    pub volume: ::core::option::Option<VolumeInfo>,
    /// HIRES伴音流信息
    #[prost(message, optional, tag = "9")]
    pub loss_less_item: ::core::option::Option<LossLessItem>,
    /// 是否支持投屏
    #[prost(bool, tag = "10")]
    pub support_project: bool,
}
/// 响度均衡操作信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeInfo {
    /// Measured integrated loudness 实际综合响度
    #[prost(double, tag = "1")]
    pub measured_i: f64,
    /// Measured loudness range 实际响度范围
    #[prost(double, tag = "2")]
    pub measured_lra: f64,
    /// Measured true peak 实际响度真峰值
    #[prost(double, tag = "3")]
    pub measured_tp: f64,
    /// Measured threshold 实际响度阈值
    #[prost(double, tag = "4")]
    pub measured_threshold: f64,
    /// Target offset gain(Gain is applied before the true-peak limiter) 目标增益Offset(增益在真实峰值限制器之前应用)
    #[prost(double, tag = "5")]
    pub target_offset: f64,
    /// Target integrated loudness 目标综合响度
    #[prost(double, tag = "6")]
    pub target_i: f64,
    /// Target true peak 目标响度真峰值
    #[prost(double, tag = "7")]
    pub target_tp: f64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArcType {
    ///
    Normal = 0,
    ///
    Interact = 1,
}
impl ArcType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ArcType::Normal => "ARC_TYPE_NORMAL",
            ArcType::Interact => "ARC_TYPE_INTERACT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ARC_TYPE_NORMAL" => Some(Self::Normal),
            "ARC_TYPE_INTERACT" => Some(Self::Interact),
            _ => None,
        }
    }
}
/// 视频编码
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodeType {
    /// 不指定
    Nocode = 0,
    /// H264
    Code264 = 1,
    /// H265
    Code265 = 2,
    /// AV1
    Codeav1 = 3,
}
impl CodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CodeType::Nocode => "NOCODE",
            CodeType::Code264 => "CODE264",
            CodeType::Code265 => "CODE265",
            CodeType::Codeav1 => "CODEAV1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOCODE" => Some(Self::Nocode),
            "CODE264" => Some(Self::Code264),
            "CODE265" => Some(Self::Code265),
            "CODEAV1" => Some(Self::Codeav1),
            _ => None,
        }
    }
}
/// 功能类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConfType {
    NoType = 0,
    Flipconf = 1,
    Castconf = 2,
    Feedback = 3,
    Subtitle = 4,
    Playbackrate = 5,
    Timeup = 6,
    Playbackmode = 7,
    Scalemode = 8,
    Backgroundplay = 9,
    Like = 10,
    Dislike = 11,
    Coin = 12,
    Elec = 13,
    Share = 14,
    Screenshot = 15,
    Lockscreen = 16,
    Recommend = 17,
    Playbackspeed = 18,
    Definition = 19,
    Selections = 20,
    Next = 21,
    Editdm = 22,
    Smallwindow = 23,
    Shake = 24,
    Outerdm = 25,
    Innerdm = 26,
    Panorama = 27,
    Dolby = 28,
    Colorfilter = 29,
    Lossless = 30,
    Freyaenter = 31,
    Freyafullenter = 32,
    Skipoped = 33,
    Recordscreen = 34,
    Dubbing = 35,
    Listen = 36,
}
impl ConfType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConfType::NoType => "NoType",
            ConfType::Flipconf => "FLIPCONF",
            ConfType::Castconf => "CASTCONF",
            ConfType::Feedback => "FEEDBACK",
            ConfType::Subtitle => "SUBTITLE",
            ConfType::Playbackrate => "PLAYBACKRATE",
            ConfType::Timeup => "TIMEUP",
            ConfType::Playbackmode => "PLAYBACKMODE",
            ConfType::Scalemode => "SCALEMODE",
            ConfType::Backgroundplay => "BACKGROUNDPLAY",
            ConfType::Like => "LIKE",
            ConfType::Dislike => "DISLIKE",
            ConfType::Coin => "COIN",
            ConfType::Elec => "ELEC",
            ConfType::Share => "SHARE",
            ConfType::Screenshot => "SCREENSHOT",
            ConfType::Lockscreen => "LOCKSCREEN",
            ConfType::Recommend => "RECOMMEND",
            ConfType::Playbackspeed => "PLAYBACKSPEED",
            ConfType::Definition => "DEFINITION",
            ConfType::Selections => "SELECTIONS",
            ConfType::Next => "NEXT",
            ConfType::Editdm => "EDITDM",
            ConfType::Smallwindow => "SMALLWINDOW",
            ConfType::Shake => "SHAKE",
            ConfType::Outerdm => "OUTERDM",
            ConfType::Innerdm => "INNERDM",
            ConfType::Panorama => "PANORAMA",
            ConfType::Dolby => "DOLBY",
            ConfType::Colorfilter => "COLORFILTER",
            ConfType::Lossless => "LOSSLESS",
            ConfType::Freyaenter => "FREYAENTER",
            ConfType::Freyafullenter => "FREYAFULLENTER",
            ConfType::Skipoped => "SKIPOPED",
            ConfType::Recordscreen => "RECORDSCREEN",
            ConfType::Dubbing => "DUBBING",
            ConfType::Listen => "LISTEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoType" => Some(Self::NoType),
            "FLIPCONF" => Some(Self::Flipconf),
            "CASTCONF" => Some(Self::Castconf),
            "FEEDBACK" => Some(Self::Feedback),
            "SUBTITLE" => Some(Self::Subtitle),
            "PLAYBACKRATE" => Some(Self::Playbackrate),
            "TIMEUP" => Some(Self::Timeup),
            "PLAYBACKMODE" => Some(Self::Playbackmode),
            "SCALEMODE" => Some(Self::Scalemode),
            "BACKGROUNDPLAY" => Some(Self::Backgroundplay),
            "LIKE" => Some(Self::Like),
            "DISLIKE" => Some(Self::Dislike),
            "COIN" => Some(Self::Coin),
            "ELEC" => Some(Self::Elec),
            "SHARE" => Some(Self::Share),
            "SCREENSHOT" => Some(Self::Screenshot),
            "LOCKSCREEN" => Some(Self::Lockscreen),
            "RECOMMEND" => Some(Self::Recommend),
            "PLAYBACKSPEED" => Some(Self::Playbackspeed),
            "DEFINITION" => Some(Self::Definition),
            "SELECTIONS" => Some(Self::Selections),
            "NEXT" => Some(Self::Next),
            "EDITDM" => Some(Self::Editdm),
            "SMALLWINDOW" => Some(Self::Smallwindow),
            "SHAKE" => Some(Self::Shake),
            "OUTERDM" => Some(Self::Outerdm),
            "INNERDM" => Some(Self::Innerdm),
            "PANORAMA" => Some(Self::Panorama),
            "DOLBY" => Some(Self::Dolby),
            "COLORFILTER" => Some(Self::Colorfilter),
            "LOSSLESS" => Some(Self::Lossless),
            "FREYAENTER" => Some(Self::Freyaenter),
            "FREYAFULLENTER" => Some(Self::Freyafullenter),
            "SKIPOPED" => Some(Self::Skipoped),
            "RECORDSCREEN" => Some(Self::Recordscreen),
            "DUBBING" => Some(Self::Dubbing),
            "LISTEN" => Some(Self::Listen),
            _ => None,
        }
    }
}
/// DRM类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DrmTechType {
    ///
    UnknownDrm = 0,
    ///
    FairPlay = 1,
    ///
    WideVine = 2,
    /// 哔哩哔哩自研DRM
    BiliDrm = 3,
}
impl DrmTechType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DrmTechType::UnknownDrm => "UNKNOWN_DRM",
            DrmTechType::FairPlay => "FAIR_PLAY",
            DrmTechType::WideVine => "WIDE_VINE",
            DrmTechType::BiliDrm => "BILI_DRM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_DRM" => Some(Self::UnknownDrm),
            "FAIR_PLAY" => Some(Self::FairPlay),
            "WIDE_VINE" => Some(Self::WideVine),
            "BILI_DRM" => Some(Self::BiliDrm),
            _ => None,
        }
    }
}
/// 错误码
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayErr {
    ///
    NoErr = 0,
    /// 管控类型的错误码
    WithMultiDeviceLoginErr = 1,
}
impl PlayErr {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayErr::NoErr => "NoErr",
            PlayErr::WithMultiDeviceLoginErr => "WithMultiDeviceLoginErr",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoErr" => Some(Self::NoErr),
            "WithMultiDeviceLoginErr" => Some(Self::WithMultiDeviceLoginErr),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UnsupportScene {
    ///
    UnknownScene = 0,
    ///
    Premiere = 1,
}
impl UnsupportScene {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UnsupportScene::UnknownScene => "UNKNOWN_SCENE",
            UnsupportScene::Premiere => "PREMIERE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_SCENE" => Some(Self::UnknownScene),
            "PREMIERE" => Some(Self::Premiere),
            _ => None,
        }
    }
}
/// 视频类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoType {
    Unknown = 0,
    /// 用户生成内容
    Ugc = 1,
    /// 专业生产内容
    Pgc = 2,
}
impl VideoType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoType::Unknown => "UNKNOWN",
            VideoType::Ugc => "UGC",
            VideoType::Pgc => "PGC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "UGC" => Some(Self::Ugc),
            "PGC" => Some(Self::Pgc),
            _ => None,
        }
    }
}
