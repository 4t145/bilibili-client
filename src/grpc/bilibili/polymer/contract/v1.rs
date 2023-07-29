///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddContractReply {
    ///
    #[prost(bool, tag = "1")]
    pub allow_message: bool,
    ///
    #[prost(bool, tag = "2")]
    pub allow_reply: bool,
    ///
    #[prost(string, tag = "3")]
    pub input_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub input_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddContractReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonReq>,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub up_mid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub aid: i64,
    ///
    #[prost(int32, tag = "5")]
    pub source: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonReq {
    ///
    #[prost(string, tag = "1")]
    pub platform: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub build: i32,
    ///
    #[prost(string, tag = "3")]
    pub buvid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub mobi_app: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub device: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub ip: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub spmid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub sub_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractConfigReply {
    ///
    #[prost(int32, tag = "1")]
    pub is_follow_display: i32,
    ///
    #[prost(int32, tag = "2")]
    pub is_triple_display: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub contract_card: ::core::option::Option<ContractCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractConfigReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonReq>,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub up_mid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub aid: i64,
    ///
    #[prost(int32, tag = "5")]
    pub source: i32,
}
/// Generated client implementations.
pub mod contract_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 契约
    #[derive(Debug, Clone)]
    pub struct ContractClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContractClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ContractClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ContractClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ContractClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        pub async fn add_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::AddContractReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.polymer.contract.v1.Contract/AddContract",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.polymer.contract.v1.Contract",
                        "AddContract",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn add_contract_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::AddContractReq>,
        ) -> std::result::Result<
            tonic::Response<super::AddContractReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.polymer.contract.v1.Contract/AddContractV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.polymer.contract.v1.Contract",
                        "AddContractV2",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn contract_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ContractConfigReq>,
        ) -> std::result::Result<
            tonic::Response<super::ContractConfigReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.polymer.contract.v1.Contract/ContractConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.polymer.contract.v1.Contract",
                        "ContractConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
