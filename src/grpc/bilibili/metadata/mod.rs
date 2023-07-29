pub mod parabox;
pub mod device;
pub mod locale;
pub mod fawkes;
pub mod network;
pub mod restriction;
/// 请求元数据
/// gRPC头部:x-bili-metadata-bin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// 登录 access_key
    #[prost(string, tag = "1")]
    pub access_key: ::prost::alloc::string::String,
    /// 包类型, 如 `android`
    #[prost(string, tag = "2")]
    pub mobi_app: ::prost::alloc::string::String,
    /// 运行设备, 留空即可
    #[prost(string, tag = "3")]
    pub device: ::prost::alloc::string::String,
    /// 构建id, 如 `7380300`
    #[prost(int32, tag = "4")]
    pub build: i32,
    /// APP分发渠道, 如 `master`
    #[prost(string, tag = "5")]
    pub channel: ::prost::alloc::string::String,
    /// 设备唯一标识
    #[prost(string, tag = "6")]
    pub buvid: ::prost::alloc::string::String,
    /// 平台类型, 如 `android`
    #[prost(string, tag = "7")]
    pub platform: ::prost::alloc::string::String,
}
