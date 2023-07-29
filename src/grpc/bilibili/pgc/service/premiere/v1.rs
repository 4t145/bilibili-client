/// 获取首播状态-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereStatusReq {
    /// 剧集epid
    #[prost(int64, tag = "1")]
    pub ep_id: i64,
}
/// 获取首播状态-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereStatusReply {
    /// 服务端播放进度 单位ms 用户实际播放进度：progress - delay_time
    #[prost(int64, tag = "1")]
    pub progress: i64,
    /// 起播时间戳 单位ms
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    /// 延迟播放时长 单位ms
    #[prost(int64, tag = "3")]
    pub delay_time: i64,
    /// 首播在线人数
    #[prost(int64, tag = "4")]
    pub online_count: i64,
    /// 首播状态
    /// 1:预热 2:首播中 3:紧急停播 4:已结束
    #[prost(int32, tag = "5")]
    pub status: i32,
    /// 首播结束后跳转类型
    /// 1:下架 2:转点播
    #[prost(int32, tag = "6")]
    pub after_premiere_type: i32,
}
/// Generated client implementations.
pub mod premiere_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 首播服务
    #[derive(Debug, Clone)]
    pub struct PremiereClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PremiereClient<tonic::transport::Channel> {
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
    impl<T> PremiereClient<T>
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
        ) -> PremiereClient<InterceptedService<T, F>>
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
            PremiereClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取首播状态
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::PremiereStatusReq>,
        ) -> std::result::Result<
            tonic::Response<super::PremiereStatusReply>,
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
                "/bilibili.pgc.service.premiere.v1.Premiere/Status",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pgc.service.premiere.v1.Premiere",
                        "Status",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
