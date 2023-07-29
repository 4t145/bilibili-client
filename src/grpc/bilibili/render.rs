///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Render {
    ///
    #[prost(int64, tag = "1")]
    pub code: i64,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub ttl: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub data: ::core::option::Option<super::super::google::protobuf::Any>,
}
