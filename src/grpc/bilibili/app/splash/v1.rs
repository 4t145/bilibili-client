///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowStrategy {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(int64, tag = "2")]
    pub stime: i64,
    ///
    #[prost(int64, tag = "3")]
    pub etime: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplashItem {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "3")]
    pub card_type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub duration: i32,
    ///
    #[prost(int64, tag = "5")]
    pub begin_time: i64,
    ///
    #[prost(int64, tag = "6")]
    pub end_time: i64,
    ///
    #[prost(string, tag = "7")]
    pub thumb: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub logo_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub logo_hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub video_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub video_hash: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "13")]
    pub video_width: i32,
    ///
    #[prost(int32, tag = "14")]
    pub video_height: i32,
    ///
    #[prost(string, tag = "15")]
    pub schema: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub schema_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "17")]
    pub schema_package_name: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "18")]
    pub schema_callup_white_list: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    ///
    #[prost(int32, tag = "19")]
    pub skip: i32,
    ///
    #[prost(string, tag = "20")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "21")]
    pub uri_title: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "22")]
    pub source: i32,
    ///
    #[prost(int32, tag = "23")]
    pub cm_mark: i32,
    ///
    #[prost(string, tag = "24")]
    pub ad_cb: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "25")]
    pub resource_id: i64,
    ///
    #[prost(string, tag = "26")]
    pub request_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "27")]
    pub client_ip: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "28")]
    pub is_ad: bool,
    ///
    #[prost(bool, tag = "29")]
    pub is_ad_loc: bool,
    ///
    #[prost(message, optional, tag = "30")]
    pub extra: ::core::option::Option<super::super::super::super::google::protobuf::Any>,
    ///
    #[prost(int64, tag = "31")]
    pub card_index: i64,
    ///
    #[prost(int64, tag = "32")]
    pub server_type: i64,
    ///
    #[prost(int64, tag = "33")]
    pub index: i64,
    ///
    #[prost(string, tag = "34")]
    pub click_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "35")]
    pub show_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "36")]
    pub time_target: i32,
    ///
    #[prost(int32, tag = "37")]
    pub encryption: i32,
    ///
    #[prost(bool, tag = "38")]
    pub enable_pre_download: bool,
    ///
    #[prost(bool, tag = "39")]
    pub enable_background_download: bool,
}
/// -响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplashReply {
    ///
    #[prost(int32, tag = "1")]
    pub max_time: i32,
    ///
    #[prost(int32, tag = "2")]
    pub min_interval: i32,
    ///
    #[prost(int32, tag = "3")]
    pub pull_interval: i32,
    ///
    #[prost(message, repeated, tag = "4")]
    pub list: ::prost::alloc::vec::Vec<SplashItem>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub show: ::prost::alloc::vec::Vec<ShowStrategy>,
}
/// -请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplashReq {
    ///
    #[prost(int32, tag = "1")]
    pub width: i32,
    ///
    #[prost(int32, tag = "2")]
    pub height: i32,
    ///
    #[prost(string, tag = "3")]
    pub birth: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub ad_extra: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub network: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod splash_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct SplashClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SplashClient<tonic::transport::Channel> {
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
    impl<T> SplashClient<T>
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
        ) -> SplashClient<InterceptedService<T, F>>
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
            SplashClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SplashReq>,
        ) -> std::result::Result<tonic::Response<super::SplashReply>, tonic::Status> {
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
                "/bilibili.app.splash.v1.Splash/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.splash.v1.Splash", "List"));
            self.inner.unary(req, path, codec).await
        }
    }
}
