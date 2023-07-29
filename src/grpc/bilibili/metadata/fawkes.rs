///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FawkesReply {
    /// 客户端在fawkes系统中对应的已发布最新的config版本号
    #[prost(string, tag = "1")]
    pub config: ::prost::alloc::string::String,
    /// 客户端在fawkes系统中对应的已发布最新的ff版本号
    #[prost(string, tag = "2")]
    pub ff: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FawkesReq {
    /// 客户端在fawkes系统的唯一名, 如 `android64`
    #[prost(string, tag = "1")]
    pub appkey: ::prost::alloc::string::String,
    /// 客户端在fawkes系统中的环境参数, 如 `prod`
    #[prost(string, tag = "2")]
    pub env: ::prost::alloc::string::String,
    /// 启动id, 8 位 0~9, a~z 组成的字符串
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
}
