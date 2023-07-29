///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicSelect {
    ///
    #[prost(message, optional, tag = "1")]
    pub fold: ::core::option::Option<super::super::v1::BoolValue>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exp {
    ///
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "2")]
    pub bucket: ::core::option::Option<super::super::v1::Int32Value>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentalConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub flag: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub exps: ::prost::alloc::vec::Vec<Exp>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultipleTusConfig {
    ///
    #[prost(message, optional, tag = "1")]
    pub top_left: ::core::option::Option<TopLeft>,
    ///
    #[prost(message, optional, tag = "2")]
    pub dynamic_select: ::core::option::Option<DynamicSelect>,
}
/// APP首页头像跳转信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopLeft {
    ///
    #[prost(message, optional, tag = "1")]
    pub url: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "2")]
    pub story_foreground_image: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "3")]
    pub story_background_image: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub listen_foreground_image: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "5")]
    pub listen_background_image: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "6")]
    pub ios_story_foreground_image: ::core::option::Option<
        super::super::v1::StringValue,
    >,
    ///
    #[prost(message, optional, tag = "7")]
    pub ios_story_background_image: ::core::option::Option<
        super::super::v1::StringValue,
    >,
    ///
    #[prost(message, optional, tag = "8")]
    pub ios_listen_foreground_image: ::core::option::Option<
        super::super::v1::StringValue,
    >,
    ///
    #[prost(message, optional, tag = "9")]
    pub ios_listen_background_image: ::core::option::Option<
        super::super::v1::StringValue,
    >,
    ///
    #[prost(message, optional, tag = "10")]
    pub goto: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "11")]
    pub url_v2: ::core::option::Option<super::super::v1::StringValue>,
    ///
    #[prost(message, optional, tag = "12")]
    pub goto_v2: ::core::option::Option<super::super::v1::Int64Value>,
    ///
    #[prost(message, optional, tag = "13")]
    pub badge: ::core::option::Option<super::super::v1::StringValue>,
}
