///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommentDoubleClickConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub interaction: ::core::option::Option<Interaction>,
    ///
    #[prost(double, tag = "2")]
    pub animation_scale: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GyroConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub gyroscope: ::core::option::Option<NftImageV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GyroscopeContentV2 {
    ///
    #[prost(string, tag = "1")]
    pub file_url: ::prost::alloc::string::String,
    ///
    #[prost(float, tag = "2")]
    pub scale: f32,
    ///
    #[prost(message, repeated, tag = "3")]
    pub physical_orientation: ::prost::alloc::vec::Vec<PhysicalOrientationV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GyroscopeEntityV2 {
    ///
    #[prost(string, tag = "1")]
    pub display_type: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub contents: ::prost::alloc::vec::Vec<GyroscopeContentV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interaction {
    ///
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub enabled: bool,
    ///
    #[prost(string, tag = "3")]
    pub itype: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub metadata_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveAnimeConfig {
    ///
    #[prost(bool, tag = "1")]
    pub is_live: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveAnimeItem {
    ///
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::common::ColorConfig>,
    ///
    #[prost(double, tag = "2")]
    pub start_ratio: f64,
    ///
    #[prost(double, tag = "3")]
    pub end_ratio: f64,
    ///
    #[prost(double, tag = "4")]
    pub start_stroke: f64,
    ///
    #[prost(double, tag = "5")]
    pub start_opacity: f64,
    ///
    #[prost(int64, tag = "6")]
    pub phase: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftImageV2 {
    ///
    #[prost(message, repeated, tag = "1")]
    pub gyroscope: ::prost::alloc::vec::Vec<GyroscopeEntityV2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalOrientationAnimation {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bezier: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalOrientationV2 {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub animations: ::prost::alloc::vec::Vec<PhysicalOrientationAnimation>,
}
