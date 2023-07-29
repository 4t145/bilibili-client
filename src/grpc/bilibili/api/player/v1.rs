/// 客户端心跳上报-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatReply {
    /// 时间戳
    #[prost(int64, tag = "1")]
    pub ts: i64,
}
/// 客户端心跳上报-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatReq {
    ///
    #[prost(int64, tag = "1")]
    pub server_time: i64,
    ///
    #[prost(string, tag = "2")]
    pub session: ::prost::alloc::string::String,
    /// 用户 mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    /// 稿件 avid
    #[prost(int64, tag = "4")]
    pub aid: i64,
    /// 视频 cid
    #[prost(int64, tag = "5")]
    pub cid: i64,
    ///
    #[prost(string, tag = "6")]
    pub sid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub epid: i64,
    ///
    #[prost(string, tag = "8")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub sub_type: i32,
    ///
    #[prost(int32, tag = "10")]
    pub quality: i32,
    ///
    #[prost(int64, tag = "11")]
    pub total_time: i64,
    ///
    #[prost(int64, tag = "12")]
    pub paused_time: i64,
    ///
    #[prost(int64, tag = "13")]
    pub played_time: i64,
    ///
    #[prost(int64, tag = "14")]
    pub video_duration: i64,
    ///
    #[prost(string, tag = "15")]
    pub play_type: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "16")]
    pub network_type: i32,
    ///
    #[prost(int64, tag = "17")]
    pub last_play_progress_time: i64,
    ///
    #[prost(int64, tag = "18")]
    pub max_play_progress_time: i64,
    ///
    #[prost(int32, tag = "19")]
    pub from: i32,
    ///
    #[prost(string, tag = "20")]
    pub from_spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "21")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "22")]
    pub epid_status: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub play_status: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "24")]
    pub user_status: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "25")]
    pub actual_played_time: i64,
    ///
    #[prost(int32, tag = "26")]
    pub auto_play: i32,
    ///
    #[prost(int64, tag = "27")]
    pub list_play_time: i64,
    ///
    #[prost(int64, tag = "28")]
    pub detail_play_time: i64,
}
/// Generated client implementations.
pub mod heartbeat_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 心跳上报
    #[derive(Debug, Clone)]
    pub struct HeartbeatClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HeartbeatClient<tonic::transport::Channel> {
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
    impl<T> HeartbeatClient<T>
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
        ) -> HeartbeatClient<InterceptedService<T, F>>
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
            HeartbeatClient::new(InterceptedService::new(inner, interceptor))
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
        /// 客户端心跳上报
        pub async fn mobile(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartbeatReq>,
        ) -> std::result::Result<tonic::Response<super::HeartbeatReply>, tonic::Status> {
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
                "/bilibili.api.player.v1.Heartbeat/Mobile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.player.v1.Heartbeat", "Mobile"));
            self.inner.unary(req, path, codec).await
        }
    }
}
