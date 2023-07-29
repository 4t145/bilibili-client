///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicRenderSpec {
    ///
    #[prost(double, tag = "1")]
    pub opacity: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorConfig {
    ///
    #[prost(bool, tag = "1")]
    pub is_dark_mode_aware: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub day: ::core::option::Option<ColorSpec>,
    ///
    #[prost(message, optional, tag = "3")]
    pub night: ::core::option::Option<ColorSpec>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorSpec {
    ///
    #[prost(string, tag = "1")]
    pub argb: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayerGeneralSpec {
    ///
    #[prost(message, optional, tag = "1")]
    pub pos_spec: ::core::option::Option<PositionSpec>,
    ///
    #[prost(message, optional, tag = "2")]
    pub size_spec: ::core::option::Option<SizeSpec>,
    ///
    #[prost(message, optional, tag = "3")]
    pub render_spec: ::core::option::Option<BasicRenderSpec>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaskProperty {
    ///
    #[prost(message, optional, tag = "1")]
    pub general_spec: ::core::option::Option<LayerGeneralSpec>,
    ///
    #[prost(message, optional, tag = "2")]
    pub mask_src: ::core::option::Option<ResourceSource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativeDrawRes {
    ///
    #[prost(int32, tag = "1")]
    pub draw_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub fill_mode: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub color_config: ::core::option::Option<ColorConfig>,
    ///
    #[prost(double, tag = "4")]
    pub edge_weight: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionSpec {
    ///
    #[prost(int32, tag = "1")]
    pub coordinate_pos: i32,
    ///
    #[prost(double, tag = "2")]
    pub axis_x: f64,
    ///
    #[prost(double, tag = "3")]
    pub axis_y: f64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteRes {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bfs_style: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceSource {
    ///
    #[prost(int32, tag = "1")]
    pub src_type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub placeholder: i32,
    ///
    #[prost(oneof = "resource_source::Res", tags = "3, 4, 5")]
    pub res: ::core::option::Option<resource_source::Res>,
}
/// Nested message and enum types in `ResourceSource`.
pub mod resource_source {
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
    pub enum LocalRes {
        Invalid = 0,
        IconVip = 1,
        IconSmallVip = 2,
        IconPersonalVerify = 3,
        IconEnterpriseVerify = 4,
        IconNftMainland = 5,
        DefaultAvatar = 6,
    }
    impl LocalRes {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocalRes::Invalid => "LOCAL_RES_INVALID",
                LocalRes::IconVip => "LOCAL_RES_ICON_VIP",
                LocalRes::IconSmallVip => "LOCAL_RES_ICON_SMALL_VIP",
                LocalRes::IconPersonalVerify => "LOCAL_RES_ICON_PERSONAL_VERIFY",
                LocalRes::IconEnterpriseVerify => "LOCAL_RES_ICON_ENTERPRISE_VERIFY",
                LocalRes::IconNftMainland => "LOCAL_RES_ICON_NFT_MAINLAND",
                LocalRes::DefaultAvatar => "LOCAL_RES_DEFAULT_AVATAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCAL_RES_INVALID" => Some(Self::Invalid),
                "LOCAL_RES_ICON_VIP" => Some(Self::IconVip),
                "LOCAL_RES_ICON_SMALL_VIP" => Some(Self::IconSmallVip),
                "LOCAL_RES_ICON_PERSONAL_VERIFY" => Some(Self::IconPersonalVerify),
                "LOCAL_RES_ICON_ENTERPRISE_VERIFY" => Some(Self::IconEnterpriseVerify),
                "LOCAL_RES_ICON_NFT_MAINLAND" => Some(Self::IconNftMainland),
                "LOCAL_RES_DEFAULT_AVATAR" => Some(Self::DefaultAvatar),
                _ => None,
            }
        }
    }
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Res {
        ///
        #[prost(message, tag = "3")]
        Remote(super::RemoteRes),
        ///
        #[prost(enumeration = "LocalRes", tag = "4")]
        Local(i32),
        ///
        #[prost(message, tag = "5")]
        Draw(super::NativeDrawRes),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeSpec {
    ///
    #[prost(double, tag = "1")]
    pub width: f64,
    ///
    #[prost(double, tag = "2")]
    pub height: f64,
}
