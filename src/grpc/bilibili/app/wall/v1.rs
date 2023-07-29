/// 免流规则信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleInfo {
    /// 是否支持免流
    #[prost(bool, tag = "1")]
    pub tf: bool,
    /// 操作模式
    /// break:无 replace:替换 proxy:代理
    #[prost(string, tag = "2")]
    pub m: ::prost::alloc::string::String,
    /// 操作参数
    #[prost(string, tag = "3")]
    pub a: ::prost::alloc::string::String,
    /// 匹配目标正则
    #[prost(string, tag = "4")]
    pub p: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "5")]
    pub a_backup: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 获取免流规则信息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleRequest {}
/// 免流规则信息组
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RulesInfo {
    /// 免流规则信息
    #[prost(message, repeated, tag = "1")]
    pub rules_info: ::prost::alloc::vec::Vec<RuleInfo>,
}
/// 获取免流规则信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RulesReply {
    /// 各ISP的免流规则信息组
    /// ISP如: cu ct cm
    #[prost(map = "string, message", tag = "1")]
    pub rules_info: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        RulesInfo,
    >,
    ///
    #[prost(string, tag = "2")]
    pub hash_value: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod wall_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 免流规则
    #[derive(Debug, Clone)]
    pub struct WallClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WallClient<tonic::transport::Channel> {
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
    impl<T> WallClient<T>
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
        ) -> WallClient<InterceptedService<T, F>>
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
            WallClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取免流规则信息
        pub async fn rule_info(
            &mut self,
            request: impl tonic::IntoRequest<super::RuleRequest>,
        ) -> std::result::Result<tonic::Response<super::RulesReply>, tonic::Status> {
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
                "/bilibili.app.wall.v1.Wall/RuleInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.wall.v1.Wall", "RuleInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
