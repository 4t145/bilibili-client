/// -响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub item: ::prost::alloc::vec::Vec<BiliSpaceVideo>,
    ///
    #[prost(int32, tag = "2")]
    pub count: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub episodic_button: ::core::option::Option<EpisodicButton>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub order: ::prost::alloc::vec::Vec<OrderConfig>,
}
/// -请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveReq {
    ///
    #[prost(int64, tag = "1")]
    pub vmid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub pn: i32,
    ///
    #[prost(int32, tag = "3")]
    pub ps: i32,
    ///
    #[prost(string, tag = "4")]
    pub order: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Badge {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub border_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub bg_style: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiliSpaceVideo {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub tname: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub duration: i64,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub danmaku: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub play: i32,
    ///
    #[prost(int64, tag = "9")]
    pub ctime: i64,
    ///
    #[prost(bool, tag = "10")]
    pub state: bool,
    ///
    #[prost(bool, tag = "11")]
    pub is_popular: bool,
    ///
    #[prost(message, repeated, tag = "12")]
    pub badges: ::prost::alloc::vec::Vec<Badge>,
    ///
    #[prost(string, tag = "13")]
    pub cover_right: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "14")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "15")]
    pub is_steins: bool,
    ///
    #[prost(bool, tag = "16")]
    pub is_ugcpay: bool,
    ///
    #[prost(bool, tag = "17")]
    pub is_cooperation: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodicButton {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderConfig {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod space_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct SpaceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SpaceClient<tonic::transport::Channel> {
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
    impl<T> SpaceClient<T>
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
        ) -> SpaceClient<InterceptedService<T, F>>
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
            SpaceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn archive(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveReq>,
        ) -> std::result::Result<tonic::Response<super::ArchiveReply>, tonic::Status> {
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
                "/bilibili.app.space.v1.Space/Archive",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.space.v1.Space", "Archive"));
            self.inner.unary(req, path, codec).await
        }
    }
}
