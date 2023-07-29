///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OtherSettingsConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub watermark_type: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "2")]
    pub web_image_quality_type: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "3")]
    pub enable_read_pasteboard: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub paste_auto_jump: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "5")]
    pub mini_screen_play_when_back: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "6")]
    pub enable_resume_playing: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "7")]
    pub enable_wifi_auto_update: ::core::option::Option<super::super::v1::BoolValue>,
    ///
    #[prost(message, optional, tag = "8")]
    pub enable_guide_screenshot_share: ::core::option::Option<
        super::super::v1::BoolValue,
    >,
}
