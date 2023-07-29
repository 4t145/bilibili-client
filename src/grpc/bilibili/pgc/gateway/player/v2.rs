///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Animation {
    ///
    #[prost(map = "string, string", tag = "1")]
    pub qn_svga_animation_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioMaterialProto {
    ///
    #[prost(string, tag = "1")]
    pub audio_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub edition: ::prost::alloc::string::String,
    ///
    #[prost(uint64, tag = "4")]
    pub person_id: u64,
    ///
    #[prost(string, tag = "5")]
    pub person_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub person_avatar: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "7")]
    pub audio: ::prost::alloc::vec::Vec<DashItem>,
}
/// 角标信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeInfo {
    /// 角标文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 角标色值
    #[prost(string, tag = "2")]
    pub bg_color: ::prost::alloc::string::String,
    /// 角标色值-夜间模式
    #[prost(string, tag = "3")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 文案色值
    #[prost(string, tag = "4")]
    pub text_color: ::prost::alloc::string::String,
    /// ? 新版本客户端已弃用此项
    #[prost(message, optional, tag = "5")]
    pub bg_gradient_color: ::core::option::Option<GradientColor>,
    ///
    #[prost(string, tag = "6")]
    pub img: ::prost::alloc::string::String,
}
/// Dialog组件: 底部显示
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BottomDisplay {
    /// 文案
    #[prost(message, optional, tag = "1")]
    pub title: ::core::option::Option<TextInfo>,
    /// 图标
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
}
/// 按钮信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonInfo {
    /// 按钮文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 按钮字体色值
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    /// 按钮字体色值-夜间模式
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 按钮背景色
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    /// 按钮背景色-夜间模式
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 按钮链接
    #[prost(string, tag = "6")]
    pub link: ::prost::alloc::string::String,
    /// 按钮动作类型
    #[prost(string, tag = "7")]
    pub action_type: ::prost::alloc::string::String,
    /// 角标信息
    #[prost(message, optional, tag = "8")]
    pub badge_info: ::core::option::Option<BadgeInfo>,
    /// 埋点上报信息
    #[prost(message, optional, tag = "9")]
    pub report: ::core::option::Option<Report>,
    /// 左侧删除线样式文案
    #[prost(string, tag = "10")]
    pub left_strikethrough_text: ::prost::alloc::string::String,
    /// 缩略按钮文案信息
    #[prost(message, optional, tag = "11")]
    pub simple_text_info: ::core::option::Option<TextInfo>,
    /// 缩略按钮背景色值
    #[prost(string, tag = "12")]
    pub simple_bg_color: ::prost::alloc::string::String,
    /// 缩略按钮字体色值-夜间模式
    #[prost(string, tag = "13")]
    pub simple_bg_color_night: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "14")]
    pub bg_gradient_color: ::core::option::Option<GradientColor>,
    ///
    #[prost(map = "string, string", tag = "15")]
    pub order_report_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    ///
    #[prost(message, optional, tag = "16")]
    pub task_param: ::core::option::Option<TaskParam>,
    ///
    #[prost(string, tag = "17")]
    pub pc_link: ::prost::alloc::string::String,
}
/// 投屏限制. code = 0 时为无限制, 否则表示不不允许投屏并提示message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CastTips {
    ///
    #[prost(int32, tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// 跳过片头/片尾配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClipInfo {
    ///
    #[prost(int64, tag = "1")]
    pub material_no: i64,
    /// DASH分段始
    #[prost(int32, tag = "2")]
    pub start: i32,
    /// DASH分段终
    #[prost(int32, tag = "3")]
    pub end: i32,
    /// Clip类型
    #[prost(enumeration = "ClipType", tag = "4")]
    pub clip_type: i32,
    /// 跳过片头/片尾时的提示语
    #[prost(string, tag = "5")]
    pub toast_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub multi_view: ::core::option::Option<MultiView>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuePlayInfo {
    ///
    #[prost(int64, tag = "1")]
    pub continue_play_ep_id: i64,
}
/// 优惠券
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coupon {
    /// 优惠券token
    #[prost(string, tag = "1")]
    pub coupon_token: ::prost::alloc::string::String,
    /// 优惠券类型
    /// 1:折扣券 2:满减券 3:兑换券
    #[prost(int64, tag = "2")]
    pub r#type: i64,
    /// 优惠券面值
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    /// 优惠券使用描述
    #[prost(string, tag = "4")]
    pub use_desc: ::prost::alloc::string::String,
    /// 优惠券标题
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    /// 优惠券描述
    #[prost(string, tag = "6")]
    pub desc: ::prost::alloc::string::String,
    /// 优惠券支付按钮文案
    #[prost(string, tag = "7")]
    pub pay_button_text: ::prost::alloc::string::String,
    /// 优惠券支付按钮删除线文案
    #[prost(string, tag = "8")]
    pub pay_button_text_line_through: ::prost::alloc::string::String,
    /// 实付金额
    #[prost(string, tag = "9")]
    pub real_amount: ::prost::alloc::string::String,
    /// 使用过期时间
    #[prost(message, optional, tag = "10")]
    pub expire_time: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Timestamp,
    >,
    ///
    #[prost(int64, tag = "11")]
    pub otype: i64,
    ///
    #[prost(string, tag = "12")]
    pub amount: ::prost::alloc::string::String,
}
/// 优惠券信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CouponInfo {
    /// 提示框信息
    #[prost(message, optional, tag = "1")]
    pub toast: ::core::option::Option<CouponToast>,
    /// 弹窗信息
    #[prost(message, optional, tag = "2")]
    pub pop_win: ::core::option::Option<PopWin>,
}
/// 优惠券提示框文案信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CouponTextInfo {
    /// 提示框文案-播正片6分钟预览
    #[prost(string, tag = "1")]
    pub positive_preview: ::prost::alloc::string::String,
    /// 提示框文案-播非正片分节ep
    #[prost(string, tag = "2")]
    pub section: ::prost::alloc::string::String,
}
/// 优惠券提示框信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CouponToast {
    /// 提示框文案信息
    #[prost(message, optional, tag = "1")]
    pub text_info: ::core::option::Option<CouponTextInfo>,
    /// 提示框按钮
    #[prost(message, optional, tag = "2")]
    pub button: ::core::option::Option<ButtonInfo>,
}
/// dash条目
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
    /// 视频大小
    #[prost(uint64, tag = "7")]
    pub size: u64,
    /// 帧率
    #[prost(string, tag = "8")]
    pub frame_rate: ::prost::alloc::string::String,
    /// DRM widevine 密钥
    #[prost(string, tag = "9")]
    pub widevine_pssh: ::prost::alloc::string::String,
}
/// dash视频流
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
    /// DRM 密钥
    #[prost(string, tag = "12")]
    pub widevine_pssh: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataControl {
    ///
    #[prost(bool, tag = "1")]
    pub need_watch_progress: bool,
}
/// 鉴权浮层
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dialog {
    /// 鉴权限制码
    #[prost(int64, tag = "1")]
    pub code: i64,
    /// 鉴权限制信息
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    /// 浮层类型
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// 浮层样式类型
    #[prost(string, tag = "4")]
    pub style_type: ::prost::alloc::string::String,
    /// 浮层配置
    #[prost(message, optional, tag = "5")]
    pub config: ::core::option::Option<DialogConfig>,
    /// 标题
    #[prost(message, optional, tag = "6")]
    pub title: ::core::option::Option<TextInfo>,
    /// 副标题
    #[prost(message, optional, tag = "7")]
    pub subtitle: ::core::option::Option<TextInfo>,
    /// 图片信息
    #[prost(message, optional, tag = "8")]
    pub image: ::core::option::Option<ImageInfo>,
    /// 按钮列表
    #[prost(message, repeated, tag = "9")]
    pub button: ::prost::alloc::vec::Vec<ButtonInfo>,
    /// 底部描述
    #[prost(message, optional, tag = "10")]
    pub bottom_desc: ::core::option::Option<ButtonInfo>,
    /// 埋点上报信息
    #[prost(message, optional, tag = "11")]
    pub report: ::core::option::Option<Report>,
    /// 倒计时 秒
    #[prost(int32, tag = "12")]
    pub count_down_sec: i32,
    /// 右下描述
    #[prost(message, optional, tag = "13")]
    pub right_bottom_desc: ::core::option::Option<TextInfo>,
    ///
    #[prost(message, repeated, tag = "14")]
    pub bottom_display: ::prost::alloc::vec::Vec<BottomDisplay>,
    ///
    #[prost(message, repeated, tag = "15")]
    pub play_list: ::prost::alloc::vec::Vec<PlayList>,
}
/// 鉴权浮层配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DialogConfig {
    /// 是否显示高斯模糊背景图
    #[prost(bool, tag = "1")]
    pub is_show_cover: bool,
    /// 是否响应转屏
    #[prost(bool, tag = "2")]
    pub is_orientation_enable: bool,
    /// 是否响应上滑吸顶
    #[prost(bool, tag = "3")]
    pub is_nested_scroll_enable: bool,
    /// 是否强制竖屏
    #[prost(bool, tag = "4")]
    pub is_force_halfscreen_enable: bool,
    /// 是否启用背景半透明
    #[prost(bool, tag = "5")]
    pub is_background_translucent_enable: bool,
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
/// 杜比音频信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DolbyItem {
    /// 杜比类型
    #[prost(enumeration = "dolby_item::Type", tag = "1")]
    pub r#type: i32,
    /// 音频流
    #[prost(message, optional, tag = "2")]
    pub audio: ::core::option::Option<DashItem>,
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
/// 播放结束后的尾页Dialog
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndPage {
    ///
    #[prost(message, optional, tag = "1")]
    pub dialog: ::core::option::Option<Dialog>,
    ///
    #[prost(bool, tag = "2")]
    pub hide: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpInlineVideo {
    ///
    #[prost(int64, tag = "1")]
    pub material_no: i64,
    ///
    #[prost(int64, tag = "2")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
}
/// 剧集广告信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeAdvertisementInfo {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub follow_video_bnt_flag: i32,
    ///
    #[prost(string, tag = "5")]
    pub next_video_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub next_video_link: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "8")]
    pub season_id: i32,
    ///
    #[prost(int32, tag = "9")]
    pub follow: i32,
}
/// EP信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeInfo {
    ///
    #[prost(int32, tag = "1")]
    pub ep_id: i32,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub ep_status: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub season_info: ::core::option::Option<SeasonInfo>,
    ///
    #[prost(string, tag = "6")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub interaction: ::core::option::Option<Interaction>,
    ///
    #[prost(string, tag = "9")]
    pub long_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpPreVideo {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpPublicityVideo {
    ///
    #[prost(enumeration = "ep_publicity_video::Type", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(oneof = "ep_publicity_video::Data", tags = "2, 3")]
    pub data: ::core::option::Option<ep_publicity_video::Data>,
}
/// Nested message and enum types in `EpPublicityVideo`.
pub mod ep_publicity_video {
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
    pub enum Type {
        DataNotSet = 0,
        EpPreVideo = 2,
        EpInline = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::DataNotSet => "DATA_NOT_SET",
                Type::EpPreVideo => "EP_PRE_VIDEO",
                Type::EpInline => "EP_INLINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DATA_NOT_SET" => Some(Self::DataNotSet),
                "EP_PRE_VIDEO" => Some(Self::EpPreVideo),
                "EP_INLINE" => Some(Self::EpInline),
                _ => None,
            }
        }
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        ///
        #[prost(message, tag = "2")]
        EpPreVideo(super::EpPreVideo),
        ///
        #[prost(message, tag = "3")]
        EpInlineVideo(super::EpInlineVideo),
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
/// 放映室提示语
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreyaConfig {
    ///
    #[prost(string, tag = "1")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "3")]
    pub issued_cnt: i32,
    ///
    #[prost(bool, tag = "4")]
    pub is_always_show: bool,
    ///
    #[prost(int32, tag = "5")]
    pub screen_number: i32,
    ///
    #[prost(int32, tag = "6")]
    pub full_screen_number: i32,
}
/// 渐变色信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GradientColor {
    ///
    #[prost(string, tag = "1")]
    pub start_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub end_color: ::prost::alloc::string::String,
}
/// 高画质试看信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighDefinitionTrialInfo {
    ///
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
    #[prost(message, optional, tag = "7")]
    pub report: ::core::option::Option<Report>,
    ///
    #[prost(message, optional, tag = "8")]
    pub quality_open_tip_btn: ::core::option::Option<ButtonInfo>,
    ///
    #[prost(message, optional, tag = "9")]
    pub no_longer_trial_btn: ::core::option::Option<ButtonInfo>,
}
/// 历史记录节点
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoryNode {
    /// 节点ID
    #[prost(int64, tag = "1")]
    pub node_id: i64,
    /// 节点标题
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 对应CID
    #[prost(int64, tag = "3")]
    pub cid: i64,
}
/// 图片信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageInfo {
    /// 图片链接
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// 交互信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interaction {
    /// 历史节点
    #[prost(message, optional, tag = "1")]
    pub history_node: ::core::option::Option<HistoryNode>,
    /// 版本
    #[prost(int64, tag = "2")]
    pub graph_version: i64,
    /// 交互消息
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
    /// 是否为交互
    #[prost(bool, tag = "4")]
    pub is_interaction: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiView {
    ///
    #[prost(string, tag = "1")]
    pub button_material: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub ep_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub avid: i64,
}
/// 大会员广告: 支付提示信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayTip {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// 图标
    #[prost(string, tag = "3")]
    pub icon: ::prost::alloc::string::String,
    /// 浮层类型
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    /// 显示类型
    #[prost(int32, tag = "5")]
    pub show_type: i32,
    /// 图片信息
    #[prost(string, tag = "6")]
    pub img: ::prost::alloc::string::String,
    /// 白天背景颜色
    #[prost(string, tag = "7")]
    pub bg_day_color: ::prost::alloc::string::String,
    /// 夜间背景颜色
    #[prost(string, tag = "8")]
    pub bg_night_color: ::prost::alloc::string::String,
    /// 白天线条颜色
    #[prost(string, tag = "9")]
    pub bg_line_color: ::prost::alloc::string::String,
    /// 夜间线条颜色
    #[prost(string, tag = "10")]
    pub bg_night_line_color: ::prost::alloc::string::String,
    /// 文字颜色
    #[prost(string, tag = "11")]
    pub text_color: ::prost::alloc::string::String,
    /// 夜间文字颜色
    #[prost(string, tag = "12")]
    pub text_night_color: ::prost::alloc::string::String,
    /// 视图展示起始时间
    #[prost(int64, tag = "13")]
    pub view_start_time: i64,
    /// 按钮列表
    #[prost(message, repeated, tag = "14")]
    pub button: ::prost::alloc::vec::Vec<ButtonInfo>,
    /// 跳转链接打开方式
    #[prost(int32, tag = "15")]
    pub url_open_type: i32,
    /// 埋点上报信息
    #[prost(message, optional, tag = "16")]
    pub report: ::core::option::Option<Report>,
    /// 角度样式
    #[prost(int32, tag = "17")]
    pub angle_style: i32,
    /// 埋点上报类型
    #[prost(int32, tag = "18")]
    pub report_type: i32,
    /// 订单埋点上报参数
    #[prost(map = "string, string", tag = "19")]
    pub order_report_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// 巨屏图片信息
    #[prost(string, tag = "20")]
    pub giant_screen_img: ::prost::alloc::string::String,
}
/// 禁用功能配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAbilityConf {
    /// 后台播放
    #[prost(bool, tag = "1")]
    pub background_play_disable: bool,
    /// 镜像反转
    #[prost(bool, tag = "2")]
    pub flip_disable: bool,
    /// 投屏
    #[prost(bool, tag = "3")]
    pub cast_disable: bool,
    /// 反馈
    #[prost(bool, tag = "4")]
    pub feedback_disable: bool,
    /// 字幕
    #[prost(bool, tag = "5")]
    pub subtitle_disable: bool,
    /// 播放速度
    #[prost(bool, tag = "6")]
    pub playback_rate_disable: bool,
    /// 定时停止
    #[prost(bool, tag = "7")]
    pub time_up_disable: bool,
    /// 播放方式
    #[prost(bool, tag = "8")]
    pub playback_mode_disable: bool,
    /// 画面尺寸
    #[prost(bool, tag = "9")]
    pub scale_mode_disable: bool,
    /// 赞
    #[prost(bool, tag = "10")]
    pub like_disable: bool,
    /// 踩
    #[prost(bool, tag = "11")]
    pub dislike_disable: bool,
    /// 投币
    #[prost(bool, tag = "12")]
    pub coin_disable: bool,
    /// 充电
    #[prost(bool, tag = "13")]
    pub elec_disable: bool,
    /// 分享
    #[prost(bool, tag = "14")]
    pub share_disable: bool,
    /// 截图
    #[prost(bool, tag = "15")]
    pub screen_shot_disable: bool,
    /// 锁定
    #[prost(bool, tag = "16")]
    pub lock_screen_disable: bool,
    /// 相关推荐
    #[prost(bool, tag = "17")]
    pub recommend_disable: bool,
    /// 播放速度
    #[prost(bool, tag = "18")]
    pub playback_speed_disable: bool,
    /// 清晰度
    #[prost(bool, tag = "19")]
    pub definition_disable: bool,
    /// 选集
    #[prost(bool, tag = "20")]
    pub selections_disable: bool,
    /// 下一集
    #[prost(bool, tag = "21")]
    pub next_disable: bool,
    /// 编辑弹幕
    #[prost(bool, tag = "22")]
    pub edit_dm_disable: bool,
    /// 小窗
    #[prost(bool, tag = "23")]
    pub small_window_disable: bool,
    /// 震动
    #[prost(bool, tag = "24")]
    pub shake_disable: bool,
    /// 外层面板弹幕设置
    #[prost(bool, tag = "25")]
    pub outer_dm_disable: bool,
    /// 三点内弹幕设置
    #[prost(bool, tag = "26")]
    pub inner_dm_disable: bool,
    /// 一起看入口
    #[prost(bool, tag = "27")]
    pub freya_enter_disable: bool,
    /// 杜比音效
    #[prost(bool, tag = "28")]
    pub dolby_disable: bool,
    /// 全屏一起看入口
    #[prost(bool, tag = "29")]
    pub freya_full_disable: bool,
    /// 跳过片头片尾
    #[prost(bool, tag = "30")]
    pub skip_oped_switch_disable: bool,
    /// 录屏
    #[prost(bool, tag = "31")]
    pub record_screen_disable: bool,
    /// 色觉优化
    #[prost(bool, tag = "32")]
    pub color_optimize_disable: bool,
    /// 配音
    #[prost(bool, tag = "33")]
    pub dubbing_disable: bool,
}
/// 云控扩展配置信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAbilityExtConf {
    ///
    #[prost(bool, tag = "1")]
    pub allow_close_subtitle: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub freya_config: ::core::option::Option<FreyaConfig>,
    ///
    #[prost(message, optional, tag = "3")]
    pub cast_tips: ::core::option::Option<CastTips>,
}
/// 播放配音信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayDubbingInfo {
    /// 背景音频
    #[prost(message, optional, tag = "1")]
    pub background_audio: ::core::option::Option<AudioMaterialProto>,
    /// 角色音频列表
    #[prost(message, repeated, tag = "2")]
    pub role_audio_list: ::prost::alloc::vec::Vec<RoleAudioProto>,
    /// 引导文本
    #[prost(string, tag = "3")]
    pub guide_text: ::prost::alloc::string::String,
}
/// 播放扩展信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayExtInfo {
    /// 播放配音信息
    #[prost(message, optional, tag = "1")]
    pub play_dubbing_info: ::core::option::Option<PlayDubbingInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayList {
    ///
    #[prost(int32, tag = "1")]
    pub season_id: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub badge_info: ::core::option::Option<BadgeInfo>,
}
/// 其他业务信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewBusinessInfo {
    /// 当前视频是否是预览
    #[prost(bool, tag = "1")]
    pub is_preview: bool,
    /// 用户是否承包过
    #[prost(bool, tag = "2")]
    pub bp: bool,
    /// drm使用
    #[prost(string, tag = "3")]
    pub marlin_token: ::prost::alloc::string::String,
    /// 倍速动效色值
    #[prost(string, tag = "4")]
    pub playback_speed_color: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub continue_play_info: ::core::option::Option<ContinuePlayInfo>,
    /// 跳过片头/片尾配置
    #[prost(message, repeated, tag = "6")]
    pub clip_info: ::prost::alloc::vec::Vec<ClipInfo>,
    ///
    #[prost(enumeration = "InlineType", tag = "7")]
    pub inline_type: i32,
    ///
    #[prost(int32, tag = "8")]
    pub ep_whole_duration: i32,
    /// 当前分辨率信息
    #[prost(message, optional, tag = "9")]
    pub dimension: ::core::option::Option<Dimension>,
    ///
    #[prost(map = "int32, message", tag = "10")]
    pub quality_ext_map: ::std::collections::HashMap<i32, QualityExtInfo>,
    ///
    #[prost(map = "string, int32", tag = "11")]
    pub exp_map: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// DRM技术类型
    #[prost(enumeration = "DrmTechType", tag = "12")]
    pub drm_tech_type: i32,
    ///
    #[prost(int32, tag = "13")]
    pub limit_action_type: i32,
    ///
    #[prost(bool, tag = "14")]
    pub is_drm: bool,
    ///
    #[prost(message, optional, tag = "15")]
    pub record_info: ::core::option::Option<RecordInfo>,
    ///
    #[prost(int32, tag = "16")]
    pub vip_status: i32,
    ///
    #[prost(bool, tag = "17")]
    pub is_live_pre: bool,
    ///
    #[prost(message, optional, tag = "18")]
    pub episode_info: ::core::option::Option<EpisodeInfo>,
    ///
    #[prost(message, optional, tag = "19")]
    pub episode_advertisement_info: ::core::option::Option<EpisodeAdvertisementInfo>,
    ///
    #[prost(message, optional, tag = "20")]
    pub user_status: ::core::option::Option<UserStatus>,
}
/// 播放页信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReply {
    /// 视频流信息
    #[prost(message, optional, tag = "1")]
    pub video_info: ::core::option::Option<VideoInfo>,
    /// 播放控件用户自定义配置
    #[prost(message, optional, tag = "2")]
    pub play_conf: ::core::option::Option<PlayAbilityConf>,
    /// 业务需要的其他信息
    #[prost(message, optional, tag = "3")]
    pub business: ::core::option::Option<PlayViewBusinessInfo>,
    /// 事件
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<Event>,
    /// 展示信息
    #[prost(message, optional, tag = "5")]
    pub view_info: ::core::option::Option<ViewInfo>,
    /// 自定义配置扩展信息
    #[prost(message, optional, tag = "6")]
    pub play_ext_conf: ::core::option::Option<PlayAbilityExtConf>,
    /// 播放扩展信息
    #[prost(message, optional, tag = "7")]
    pub play_ext_info: ::core::option::Option<PlayExtInfo>,
}
/// 播放页信息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReq {
    /// 剧集epid
    #[prost(int64, tag = "1")]
    pub epid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
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
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 青少年模式
    #[prost(int32, tag = "11")]
    pub teenagers_mode: i32,
    /// 视频编码
    #[prost(enumeration = "CodeType", tag = "12")]
    pub prefer_codec_type: i32,
    /// 是否强制请求预览视频
    #[prost(bool, tag = "13")]
    pub is_preview: bool,
    /// 一起看房间id
    #[prost(int64, tag = "14")]
    pub room_id: i64,
    /// 是否需要展示信息
    #[prost(bool, tag = "15")]
    pub is_need_view_info: bool,
    /// 场景控制
    #[prost(message, optional, tag = "16")]
    pub scene_control: ::core::option::Option<SceneControl>,
    ///
    #[prost(enumeration = "InlineScene", tag = "17")]
    pub inline_scene: i32,
    ///
    #[prost(int64, tag = "18")]
    pub material_no: i64,
    /// DRM 安全等级
    #[prost(int32, tag = "19")]
    pub security_level: i32,
    ///
    #[prost(int64, tag = "20")]
    pub season_id: i64,
    ///
    #[prost(message, optional, tag = "21")]
    pub data_control: ::core::option::Option<DataControl>,
}
/// 弹窗信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopWin {
    /// 弹窗标题 老字段
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 优惠券列表
    #[prost(message, repeated, tag = "2")]
    pub coupon: ::prost::alloc::vec::Vec<Coupon>,
    /// 弹窗按钮列表
    #[prost(message, repeated, tag = "3")]
    pub button: ::prost::alloc::vec::Vec<ButtonInfo>,
    /// 底部文案 老字段
    #[prost(string, tag = "4")]
    pub bottom_text: ::prost::alloc::string::String,
    /// 弹窗标题 新字段
    #[prost(message, optional, tag = "5")]
    pub pop_title: ::core::option::Option<TextInfo>,
    /// 弹窗副标题
    #[prost(message, optional, tag = "6")]
    pub subtitle: ::core::option::Option<TextInfo>,
    /// 底部描述 新字段
    #[prost(message, optional, tag = "7")]
    pub bottom_desc: ::core::option::Option<ButtonInfo>,
    /// 弹窗小图
    #[prost(string, tag = "8")]
    pub cover: ::prost::alloc::string::String,
    /// 弹窗类型
    #[prost(string, tag = "9")]
    pub pop_type: ::prost::alloc::string::String,
}
/// 广告组件: 竖屏时视频下部提示栏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromptBar {
    /// 主标题, 如: "本片含大会员专享内容"
    #[prost(message, optional, tag = "1")]
    pub title: ::core::option::Option<TextInfo>,
    /// 副标题, 如: "成为大会员可免费看全部剧集"
    #[prost(message, optional, tag = "2")]
    pub sub_title: ::core::option::Option<TextInfo>,
    /// 副标题前面的icon
    #[prost(string, tag = "3")]
    pub sub_title_icon: ::prost::alloc::string::String,
    /// 背景图
    #[prost(string, tag = "4")]
    pub bg_image: ::prost::alloc::string::String,
    /// 背景渐变色
    #[prost(message, optional, tag = "5")]
    pub bg_gradient_color: ::core::option::Option<GradientColor>,
    /// 按钮
    #[prost(message, repeated, tag = "6")]
    pub button: ::prost::alloc::vec::Vec<ButtonInfo>,
    /// 埋点上报信息
    #[prost(message, optional, tag = "7")]
    pub report: ::core::option::Option<Report>,
    ///
    #[prost(string, tag = "8")]
    pub full_screen_ip_icon: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub full_screen_bg_gradient_color: ::core::option::Option<GradientColor>,
}
/// 云控拓展视频画质信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityExtInfo {
    /// 是否支持试看
    #[prost(bool, tag = "1")]
    pub trial_support: bool,
}
/// 备案信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordInfo {
    /// 记录
    #[prost(string, tag = "1")]
    pub record: ::prost::alloc::string::String,
    /// 记录图标
    #[prost(string, tag = "2")]
    pub record_icon: ::prost::alloc::string::String,
}
/// 埋点上报信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    /// 曝光事件
    #[prost(string, tag = "1")]
    pub show_event_id: ::prost::alloc::string::String,
    /// 点击事件
    #[prost(string, tag = "2")]
    pub click_event_id: ::prost::alloc::string::String,
    /// 埋点透传参数
    #[prost(string, tag = "3")]
    pub extends: ::prost::alloc::string::String,
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
/// 权限信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rights {
    /// 是否可以观看
    #[prost(int32, tag = "1")]
    pub can_watch: i32,
}
/// 角色配音信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleAudioProto {
    /// 角色ID
    #[prost(int64, tag = "1")]
    pub role_id: i64,
    /// 角色名称
    #[prost(string, tag = "2")]
    pub role_name: ::prost::alloc::string::String,
    /// 角色头像
    #[prost(string, tag = "3")]
    pub role_avatar: ::prost::alloc::string::String,
    /// 音频素材列表
    #[prost(message, repeated, tag = "4")]
    pub audio_material_list: ::prost::alloc::vec::Vec<AudioMaterialProto>,
}
/// 场景控制
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SceneControl {
    /// 是否收藏播单
    #[prost(bool, tag = "1")]
    pub fav_playlist: bool,
    /// 是否小窗
    #[prost(bool, tag = "2")]
    pub small_window: bool,
    /// 是否画中画
    #[prost(bool, tag = "3")]
    pub pip: bool,
    ///
    #[prost(bool, tag = "4")]
    pub was_he_inline: bool,
    ///
    #[prost(bool, tag = "5")]
    pub is_need_trial: bool,
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
/// PGC SEASON 信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonInfo {
    /// PGC SEASON ID
    #[prost(int32, tag = "1")]
    pub season_id: i32,
    /// PGC SEASON 类型
    #[prost(int32, tag = "2")]
    pub season_type: i32,
    /// PGC SEASON 状态
    #[prost(int32, tag = "3")]
    pub season_status: i32,
    /// 封面
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    /// 标题
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    /// 权限信息
    #[prost(message, optional, tag = "6")]
    pub rights: ::core::option::Option<Rights>,
    /// 模式
    #[prost(int32, tag = "7")]
    pub mode: i32,
}
/// 分段视频流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentVideo {
    /// 分段视频流列表
    #[prost(message, repeated, tag = "1")]
    pub segment: ::prost::alloc::vec::Vec<ResponseUrl>,
}
/// 震动
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shake {
    /// 文件地址
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
}
/// 视频流信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// 元数据
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<StreamInfo>,
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
/// 流媒体元数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfo {
    /// 视频质量
    #[prost(int32, tag = "1")]
    pub quality: i32,
    /// 视频格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 描述信息
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// 错误码
    #[prost(int32, tag = "4")]
    pub err_code: i32,
    /// 流限制信息
    #[prost(message, optional, tag = "5")]
    pub limit: ::core::option::Option<StreamLimit>,
    /// 是否需要VIP
    #[prost(bool, tag = "6")]
    pub need_vip: bool,
    /// 是否需要登录
    #[prost(bool, tag = "7")]
    pub need_login: bool,
    /// 是否完整
    #[prost(bool, tag = "8")]
    pub intact: bool,
    /// 权限信息
    #[prost(int64, tag = "10")]
    pub attribute: i64,
    /// 新版描述信息
    #[prost(string, tag = "11")]
    pub new_description: ::prost::alloc::string::String,
    /// 显示描述信息
    #[prost(string, tag = "12")]
    pub display_desc: ::prost::alloc::string::String,
    /// 上标
    #[prost(string, tag = "13")]
    pub superscript: ::prost::alloc::string::String,
    /// 方案信息
    #[prost(message, optional, tag = "14")]
    pub scheme: ::core::option::Option<Scheme>,
    /// 是否支持DRM
    #[prost(bool, tag = "15")]
    pub support_drm: bool,
    /// 字幕信息
    #[prost(string, tag = "16")]
    pub subtitle: ::prost::alloc::string::String,
}
/// 清晰度不满足条件信息
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
/// 任务参数信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskParam {
    /// 任务类型
    #[prost(string, tag = "1")]
    pub task_type: ::prost::alloc::string::String,
    /// 活动ID
    #[prost(int64, tag = "2")]
    pub activity_id: i64,
    /// 提示ID
    #[prost(int64, tag = "3")]
    pub tips_id: i64,
}
/// 文案信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInfo {
    /// 文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 字体色值
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    /// 字体色值-夜间模式
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
}
/// toast
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Toast {
    /// toast文案 老字段
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// toast按钮
    #[prost(message, optional, tag = "2")]
    pub button: ::core::option::Option<ButtonInfo>,
    /// 显示样式类型
    #[prost(int32, tag = "3")]
    pub show_style_type: i32,
    /// 图标
    #[prost(string, tag = "4")]
    pub icon: ::prost::alloc::string::String,
    /// toast文案 新字段
    #[prost(message, optional, tag = "5")]
    pub toast_text: ::core::option::Option<TextInfo>,
    /// 埋点上报信息
    #[prost(message, optional, tag = "6")]
    pub report: ::core::option::Option<Report>,
    ///
    #[prost(map = "string, string", tag = "7")]
    pub order_report_params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// 用户状态信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserStatus {
    /// 是否支付
    #[prost(bool, tag = "1")]
    pub pay_check: bool,
    /// 是否承包
    #[prost(bool, tag = "2")]
    pub sponsor: bool,
    /// 观看进度
    #[prost(message, optional, tag = "3")]
    pub watch_progress: ::core::option::Option<WatchProgress>,
    /// 系列观看进度
    #[prost(message, optional, tag = "4")]
    pub aid_watch_progress: ::core::option::Option<WatchProgress>,
}
/// 视频url信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoInfo {
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
}
/// 展示信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewInfo {
    /// 弹窗
    #[prost(message, optional, tag = "1")]
    pub dialog: ::core::option::Option<Dialog>,
    /// Toast
    #[prost(message, optional, tag = "2")]
    pub toast: ::core::option::Option<Toast>,
    /// 优惠券信息
    #[prost(message, optional, tag = "3")]
    pub coupon_info: ::core::option::Option<CouponInfo>,
    /// 未支付剧集ID列表
    #[prost(int64, repeated, tag = "4")]
    pub demand_no_pay_epids: ::prost::alloc::vec::Vec<i64>,
    /// 结束页
    #[prost(message, optional, tag = "5")]
    pub end_page: ::core::option::Option<EndPage>,
    /// 扩展配置
    #[prost(map = "string, bool", tag = "6")]
    pub exp_config: ::std::collections::HashMap<::prost::alloc::string::String, bool>,
    /// 弹窗
    #[prost(message, optional, tag = "7")]
    pub pop_win: ::core::option::Option<PopWin>,
    /// 试看提示栏
    #[prost(message, optional, tag = "8")]
    pub try_watch_prompt_bar: ::core::option::Option<PromptBar>,
    /// 支付提示信息
    #[prost(message, optional, tag = "9")]
    pub pay_tip: ::core::option::Option<PayTip>,
    /// 高清试看提示信息
    #[prost(message, optional, tag = "10")]
    pub high_definition_trial_info: ::core::option::Option<HighDefinitionTrialInfo>,
    /// 弹窗扩展
    #[prost(map = "string, message", tag = "11")]
    pub ext_dialog: ::std::collections::HashMap<::prost::alloc::string::String, Dialog>,
    /// 动画
    #[prost(message, optional, tag = "12")]
    pub animation: ::core::option::Option<Animation>,
    /// Toast扩展
    #[prost(map = "string, message", tag = "13")]
    pub ext_toast: ::std::collections::HashMap<::prost::alloc::string::String, Toast>,
}
/// 观看进度信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchProgress {
    /// 上次观看的 EP ID
    #[prost(int32, tag = "1")]
    pub last_ep_id: i32,
    /// 上次观看到的EP INDEX
    #[prost(string, tag = "2")]
    pub last_ep_index: ::prost::alloc::string::String,
    /// 上次观看的进度
    #[prost(int64, tag = "3")]
    pub progress: i64,
    /// 上次观看的 CID
    #[prost(int64, tag = "4")]
    pub last_play_cid: i64,
    /// 带时间的提示信息
    #[prost(message, optional, tag = "5")]
    pub toast: ::core::option::Option<Toast>,
    /// 不带时间的提示信息
    #[prost(message, optional, tag = "6")]
    pub toast_without_time: ::core::option::Option<Toast>,
    /// 上次观看的 AID
    #[prost(int64, tag = "7")]
    pub last_play_aid: i64,
}
/// 跳过片头/片尾配置: Clip类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClipType {
    ///
    NtUnknown = 0,
    /// 跳过OP
    Op = 1,
    /// 跳过ED
    Ed = 2,
    ///
    He = 3,
    ///
    MultiView = 4,
    ///
    Ad = 5,
}
impl ClipType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClipType::NtUnknown => "NT_UNKNOWN",
            ClipType::Op => "CLIP_TYPE_OP",
            ClipType::Ed => "CLIP_TYPE_ED",
            ClipType::He => "CLIP_TYPE_HE",
            ClipType::MultiView => "CLIP_TYPE_MULTI_VIEW",
            ClipType::Ad => "CLIP_TYPE_AD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NT_UNKNOWN" => Some(Self::NtUnknown),
            "CLIP_TYPE_OP" => Some(Self::Op),
            "CLIP_TYPE_ED" => Some(Self::Ed),
            "CLIP_TYPE_HE" => Some(Self::He),
            "CLIP_TYPE_MULTI_VIEW" => Some(Self::MultiView),
            "CLIP_TYPE_AD" => Some(Self::Ad),
            _ => None,
        }
    }
}
/// 编码类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodeType {
    /// 默认
    Nocode = 0,
    /// H.264
    Code264 = 1,
    /// H.265
    Code265 = 2,
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
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOCODE" => Some(Self::Nocode),
            "CODE264" => Some(Self::Code264),
            "CODE265" => Some(Self::Code265),
            _ => None,
        }
    }
}
/// DRM技术类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DrmTechType {
    ///
    Non = 0,
    ///
    FairPlay = 1,
    ///
    WideVine = 2,
    ///
    BiliDrm = 3,
}
impl DrmTechType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DrmTechType::Non => "NON",
            DrmTechType::FairPlay => "FAIR_PLAY",
            DrmTechType::WideVine => "WIDE_VINE",
            DrmTechType::BiliDrm => "BILI_DRM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NON" => Some(Self::Non),
            "FAIR_PLAY" => Some(Self::FairPlay),
            "WIDE_VINE" => Some(Self::WideVine),
            "BILI_DRM" => Some(Self::BiliDrm),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EpPublicityVideoType {
    ///
    Pre = 0,
    ///
    Inline = 1,
}
impl EpPublicityVideoType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EpPublicityVideoType::Pre => "PRE",
            EpPublicityVideoType::Inline => "INLINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRE" => Some(Self::Pre),
            "INLINE" => Some(Self::Inline),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InlineScene {
    ///
    Unknown = 0,
    ///
    RelatedEp = 1,
    ///
    He = 2,
    ///
    Skip = 3,
}
impl InlineScene {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InlineScene::Unknown => "UNKNOWN",
            InlineScene::RelatedEp => "RELATED_EP",
            InlineScene::He => "HE",
            InlineScene::Skip => "SKIP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "RELATED_EP" => Some(Self::RelatedEp),
            "HE" => Some(Self::He),
            "SKIP" => Some(Self::Skip),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InlineType {
    ///
    TypeUnknown = 0,
    ///
    TypeWhole = 1,
    ///
    TypeHeClip = 2,
    ///
    TypePreview = 3,
}
impl InlineType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InlineType::TypeUnknown => "TYPE_UNKNOWN",
            InlineType::TypeWhole => "TYPE_WHOLE",
            InlineType::TypeHeClip => "TYPE_HE_CLIP",
            InlineType::TypePreview => "TYPE_PREVIEW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNKNOWN" => Some(Self::TypeUnknown),
            "TYPE_WHOLE" => Some(Self::TypeWhole),
            "TYPE_HE_CLIP" => Some(Self::TypeHeClip),
            "TYPE_PREVIEW" => Some(Self::TypePreview),
            _ => None,
        }
    }
}
/// 限制操作类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LimitActionType {
    ///
    LatUnknown = 0,
    ///
    ShowLimitDialog = 1,
    ///
    SkipCurrentEp = 2,
}
impl LimitActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LimitActionType::LatUnknown => "LAT_UNKNOWN",
            LimitActionType::ShowLimitDialog => "SHOW_LIMIT_DIALOG",
            LimitActionType::SkipCurrentEp => "SKIP_CURRENT_EP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LAT_UNKNOWN" => Some(Self::LatUnknown),
            "SHOW_LIMIT_DIALOG" => Some(Self::ShowLimitDialog),
            "SKIP_CURRENT_EP" => Some(Self::SkipCurrentEp),
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
/// DRM 安全等级
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityLevel {
    ///
    LevelUnknown = 0,
    ///
    LevelL1 = 1,
    ///
    LevelL2 = 2,
    ///
    LevelL3 = 3,
}
impl SecurityLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityLevel::LevelUnknown => "LEVEL_UNKNOWN",
            SecurityLevel::LevelL1 => "LEVEL_L1",
            SecurityLevel::LevelL2 => "LEVEL_L2",
            SecurityLevel::LevelL3 => "LEVEL_L3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEVEL_UNKNOWN" => Some(Self::LevelUnknown),
            "LEVEL_L1" => Some(Self::LevelL1),
            "LEVEL_L2" => Some(Self::LevelL2),
            "LEVEL_L3" => Some(Self::LevelL3),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod play_url_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 视频url
    #[derive(Debug, Clone)]
    pub struct PlayUrlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayUrlClient<tonic::transport::Channel> {
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
    impl<T> PlayUrlClient<T>
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
        ) -> PlayUrlClient<InterceptedService<T, F>>
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
            PlayUrlClient::new(InterceptedService::new(inner, interceptor))
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
        /// 播放页信息
        pub async fn play_view(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewReq>,
        ) -> std::result::Result<tonic::Response<super::PlayViewReply>, tonic::Status> {
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
                "/bilibili.pgc.gateway.player.v2.PlayURL/PlayView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.pgc.gateway.player.v2.PlayURL", "PlayView"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn play_view_comic(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewReq>,
        ) -> std::result::Result<tonic::Response<super::PlayViewReply>, tonic::Status> {
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
                "/bilibili.pgc.gateway.player.v2.PlayURL/PlayViewComic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pgc.gateway.player.v2.PlayURL",
                        "PlayViewComic",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
