///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloWorldReq {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloWorldResp {
    ///
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
}
