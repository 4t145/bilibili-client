/// 投屏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjReply {
    /// 投屏命令
    /// 1:起播 2:快进 3:快退 4:seek播放进度 5:暂停 6:暂停恢复
    #[prost(int64, tag = "1")]
    pub cmd_type: i64,
    /// 用户id
    #[prost(int64, tag = "2")]
    pub mid: i64,
    /// 稿件id
    #[prost(int64, tag = "3")]
    pub aid: i64,
    /// 视频id
    #[prost(int64, tag = "4")]
    pub cid: i64,
    /// 视频类型
    /// 0:ugc 1:pgc 2:pugv
    #[prost(int64, tag = "5")]
    pub video_type: i64,
    /// 单集id，pgc和pugv需要传
    #[prost(int64, tag = "6")]
    pub ep_id: i64,
    /// 剧集id
    #[prost(int64, tag = "7")]
    pub season_id: i64,
    /// seek 的位置，cmd位seek时有值，单位秒
    #[prost(int64, tag = "8")]
    pub seek_ts: i64,
    /// 其他指令对应内容
    #[prost(string, tag = "9")]
    pub extra: ::prost::alloc::string::String,
}
/// 直播状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveStatusNotify {
    /// 直播状态
    /// 1:开播 2:关播 3:截流 4:截流恢复
    #[prost(int64, tag = "1")]
    pub status: i64,
    /// 文案
    #[prost(string, tag = "2")]
    pub msg: ::prost::alloc::string::String,
    /// 直播房间号
    #[prost(int64, tag = "3")]
    pub cid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EsportsNotify {
    /// 直播房间号
    #[prost(int64, tag = "1")]
    pub cid: i64,
}
/// 直播插卡
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicityNotify {
    /// 插卡id
    #[prost(int64, tag = "1")]
    pub publicity_id: i64,
    /// 直播房间号
    #[prost(int64, tag = "2")]
    pub room_id: i64,
    /// 直播间状态
    /// 0:未开播 1:直播中 2:轮播中
    #[prost(int64, tag = "3")]
    pub status: i64,
}
/// 直转点
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveSkipNotify {
    /// 直播id
    #[prost(int64, tag = "1")]
    pub live_id: i64,
}
/// Generated client implementations.
pub mod tv_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct TvClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TvClient<tonic::transport::Channel> {
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
    impl<T> TvClient<T>
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
        ) -> TvClient<InterceptedService<T, F>>
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
            TvClient::new(InterceptedService::new(inner, interceptor))
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
        /// 投屏
        pub async fn proj(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ProjReply>>,
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
                "/bilibili.broadcast.message.tv.Tv/Proj",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.message.tv.Tv", "Proj"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// 直播状态
        pub async fn live_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::LiveStatusNotify>>,
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
                "/bilibili.broadcast.message.tv.Tv/LiveStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.broadcast.message.tv.Tv", "LiveStatus"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// 赛事比分通知
        pub async fn esports(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::EsportsNotify>>,
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
                "/bilibili.broadcast.message.tv.Tv/Esports",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.message.tv.Tv", "Esports"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// 直播插卡
        pub async fn publicity(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PublicityNotify>>,
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
                "/bilibili.broadcast.message.tv.Tv/Publicity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.broadcast.message.tv.Tv", "Publicity"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// 直转点
        pub async fn live_skip(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::LiveSkipNotify>>,
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
                "/bilibili.broadcast.message.tv.Tv/LiveSkip",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.message.tv.Tv", "LiveSkip"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
