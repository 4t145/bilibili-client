/// 云端保存的播放器配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudPlayConfig {
    /// 启用杜比全景声
    #[prost(message, optional, tag = "1")]
    pub enable_panorama: ::core::option::Option<super::super::v1::BoolValue>,
    /// 启用杜比音效
    #[prost(message, optional, tag = "2")]
    pub enable_dolby: ::core::option::Option<super::super::v1::BoolValue>,
    /// 启用震动
    #[prost(message, optional, tag = "3")]
    pub enable_shake: ::core::option::Option<super::super::v1::BoolValue>,
    /// 启用后台播放
    #[prost(message, optional, tag = "4")]
    pub enable_background: ::core::option::Option<super::super::v1::BoolValue>,
    /// 启用HIRES
    #[prost(message, optional, tag = "5")]
    pub enable_loss_less: ::core::option::Option<super::super::v1::BoolValue>,
}
/// 播放器策略配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub should_auto_play: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "2")]
    pub should_auto_fullscreen: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "3")]
    pub enable_playurl_https: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub enable_danmaku_interaction: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "5")]
    pub small_screen_status: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "6")]
    pub player_codec_mode_key: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "7")]
    pub enable_gravity_rotate_screen: ::core::option::Option<
        super::super::v1::BoolValue,
    >,
    ///
    #[prost(message, optional, tag = "8")]
    pub enable_danmaku_monospaced: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "9")]
    pub enable_edit_subtitle: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "10")]
    pub enable_subtitle: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "11")]
    pub color_filter: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "12")]
    pub should_auto_story: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "13")]
    pub landscape_auto_story: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "14")]
    pub volume_balance: ::core::option::Option<super::super::v1::BoolValue>,
}
/// 灰度测试特殊功能？
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecificPlayConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub enable_segmented_section: ::core::option::Option<super::super::v1::BoolValue>,
}
