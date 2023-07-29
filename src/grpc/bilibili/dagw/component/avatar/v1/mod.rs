pub mod plugin;
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvatarItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub container_size: ::core::option::Option<super::common::SizeSpec>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub layers: ::prost::alloc::vec::Vec<LayerGroup>,
    ///
    #[prost(message, optional, tag = "3")]
    pub fallback_layers: ::core::option::Option<LayerGroup>,
    ///
    #[prost(int64, tag = "4")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicLayerResource {
    ///
    #[prost(int32, tag = "1")]
    pub res_type: i32,
    ///
    #[prost(oneof = "basic_layer_resource::Payload", tags = "2, 3, 4")]
    pub payload: ::core::option::Option<basic_layer_resource::Payload>,
}
/// Nested message and enum types in `BasicLayerResource`.
pub mod basic_layer_resource {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        ///
        #[prost(message, tag = "2")]
        ResImage(super::ResImage),
        ///
        #[prost(message, tag = "3")]
        ResAnimation(super::ResAnimation),
        /// /
        #[prost(message, tag = "4")]
        ResNativeDraw(super::ResNativeDraw),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralConfig {
    ///
    #[prost(map = "string, string", tag = "1")]
    pub web_css_style: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Layer {
    ///
    #[prost(string, tag = "1")]
    pub layer_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub visible: bool,
    ///
    #[prost(message, optional, tag = "3")]
    pub general_spec: ::core::option::Option<super::common::LayerGeneralSpec>,
    ///
    #[prost(message, optional, tag = "4")]
    pub layer_config: ::core::option::Option<LayerConfig>,
    ///
    #[prost(message, optional, tag = "5")]
    pub resource: ::core::option::Option<BasicLayerResource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayerConfig {
    ///
    #[prost(map = "string, message", tag = "1")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        LayerTagConfig,
    >,
    ///
    #[prost(bool, tag = "2")]
    pub is_critical: bool,
    ///
    #[prost(bool, tag = "3")]
    pub allow_over_paint: bool,
    ///
    #[prost(message, optional, tag = "4")]
    pub layer_mask: ::core::option::Option<super::common::MaskProperty>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayerGroup {
    ///
    #[prost(string, tag = "1")]
    pub group_id: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub layers: ::prost::alloc::vec::Vec<Layer>,
    ///
    #[prost(message, optional, tag = "3")]
    pub group_mask: ::core::option::Option<super::common::MaskProperty>,
    ///
    #[prost(bool, tag = "4")]
    pub is_critical_group: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayerTagConfig {
    ///
    #[prost(int32, tag = "1")]
    pub config_type: i32,
    ///
    #[prost(oneof = "layer_tag_config::Config", tags = "2, 3, 4, 5")]
    pub config: ::core::option::Option<layer_tag_config::Config>,
}
/// Nested message and enum types in `LayerTagConfig`.
pub mod layer_tag_config {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        ///
        #[prost(message, tag = "2")]
        GeneralConfig(super::GeneralConfig),
        ///
        #[prost(message, tag = "3")]
        GyroConfig(super::plugin::GyroConfig),
        ///
        #[prost(message, tag = "4")]
        CommentDoubleClickConfig(super::plugin::CommentDoubleClickConfig),
        ///
        #[prost(message, tag = "5")]
        LiveAnimeConfig(super::plugin::LiveAnimeConfig),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResAnimation {
    ///
    #[prost(message, optional, tag = "1")]
    pub webp_src: ::core::option::Option<super::common::ResourceSource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResImage {
    ///
    #[prost(message, optional, tag = "1")]
    pub image_src: ::core::option::Option<super::common::ResourceSource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResNativeDraw {
    ///
    #[prost(message, optional, tag = "1")]
    pub draw_src: ::core::option::Option<super::common::ResourceSource>,
}
