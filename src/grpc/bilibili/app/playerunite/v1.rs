///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewUniteReq {
    /// 请求资源VOD信息
    #[prost(message, optional, tag = "1")]
    pub vod: ::core::option::Option<super::super::super::playershared::VideoVod>,
    ///
    #[prost(string, tag = "2")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 补充信息, 如ep_id等
    #[prost(map = "string, string", tag = "4")]
    pub extra_content: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewUniteReply {
    /// 音视频流信息
    #[prost(message, optional, tag = "1")]
    pub vod_info: ::core::option::Option<super::super::super::playershared::VodInfo>,
    ///
    #[prost(message, optional, tag = "2")]
    pub play_arc_conf: ::core::option::Option<
        super::super::super::playershared::PlayArcConf,
    >,
    ///
    #[prost(message, optional, tag = "3")]
    pub play_device_conf: ::core::option::Option<
        super::super::super::playershared::PlayDeviceConf,
    >,
    ///
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<super::super::super::playershared::Event>,
    /// 使用 pgcanymodel / ugcanymodel 进行proto any转换成对应业务码结构体
    #[prost(message, optional, tag = "5")]
    pub supplement: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    ///
    #[prost(message, optional, tag = "6")]
    pub play_arc: ::core::option::Option<super::super::super::playershared::PlayArc>,
    ///
    #[prost(message, optional, tag = "7")]
    pub qn_trial_info: ::core::option::Option<
        super::super::super::playershared::QnTrialInfo,
    >,
    ///
    #[prost(message, optional, tag = "8")]
    pub history: ::core::option::Option<super::super::super::playershared::History>,
}
/// Generated client implementations.
pub mod player_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 统一视频url
    #[derive(Debug, Clone)]
    pub struct PlayerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayerClient<tonic::transport::Channel> {
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
    impl<T> PlayerClient<T>
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
        ) -> PlayerClient<InterceptedService<T, F>>
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
            PlayerClient::new(InterceptedService::new(inner, interceptor))
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
        /// 视频地址
        pub async fn play_view_unite(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewUniteReq>,
        ) -> std::result::Result<
            tonic::Response<super::PlayViewUniteReply>,
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
                "/bilibili.app.playerunite.v1.Player/PlayViewUnite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.playerunite.v1.Player",
                        "PlayViewUnite",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
