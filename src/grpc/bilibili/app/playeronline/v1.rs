/// 空回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
/// 获取在线人数-回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerOnlineReply {
    ///
    #[prost(string, tag = "1")]
    pub total_text: ::prost::alloc::string::String,
    /// 下次轮询间隔时间
    #[prost(int64, tag = "2")]
    pub sec_next: i64,
    /// 是否底部显示
    #[prost(bool, tag = "3")]
    pub bottom_show: bool,
    ///
    #[prost(bool, tag = "4")]
    pub sdm_show: bool,
    ///
    #[prost(string, tag = "5")]
    pub sdm_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub total_number: i64,
    ///
    #[prost(string, tag = "7")]
    pub total_number_text: ::prost::alloc::string::String,
}
/// 获取在线人数-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerOnlineReq {
    /// 稿件 avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 视频 cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 是否在播放中
    #[prost(bool, tag = "3")]
    pub play_open: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereInfoReply {
    ///
    #[prost(string, tag = "1")]
    pub premiere_over_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub participant: i64,
    ///
    #[prost(int64, tag = "3")]
    pub interaction: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PremiereInfoReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportWatchReq {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub biz: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub buvid: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod player_online_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 在线人数
    #[derive(Debug, Clone)]
    pub struct PlayerOnlineClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerOnlineClient<tonic::transport::Channel> {
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
    impl<T> PlayerOnlineClient<T>
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
        ) -> PlayerOnlineClient<InterceptedService<T, F>>
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
            PlayerOnlineClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取在线人数
        pub async fn player_online(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayerOnlineReq>,
        ) -> std::result::Result<
            tonic::Response<super::PlayerOnlineReply>,
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
                "/bilibili.app.playeronline.v1.PlayerOnline/PlayerOnline",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.playeronline.v1.PlayerOnline",
                        "PlayerOnline",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn premiere_info(
            &mut self,
            request: impl tonic::IntoRequest<super::PremiereInfoReq>,
        ) -> std::result::Result<
            tonic::Response<super::PremiereInfoReply>,
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
                "/bilibili.app.playeronline.v1.PlayerOnline/PremiereInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.playeronline.v1.PlayerOnline",
                        "PremiereInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn report_watch(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportWatchReq>,
        ) -> std::result::Result<tonic::Response<super::NoReply>, tonic::Status> {
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
                "/bilibili.app.playeronline.v1.PlayerOnline/ReportWatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.playeronline.v1.PlayerOnline",
                        "ReportWatch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
