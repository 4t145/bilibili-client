///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionConfig {
    ///
    #[prost(string, tag = "1")]
    pub scenes_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub scenes_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfo {
    ///
    #[prost(int32, tag = "1")]
    pub tid: i32,
    ///
    #[prost(int32, tag = "2")]
    pub reid: i32,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub logo: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "9")]
    pub is_bangumi: i32,
    ///
    #[prost(message, repeated, tag = "10")]
    pub children: ::prost::alloc::vec::Vec<RegionInfo>,
    ///
    #[prost(message, repeated, tag = "11")]
    pub config: ::prost::alloc::vec::Vec<RegionConfig>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub regions: ::prost::alloc::vec::Vec<RegionInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionReq {
    ///
    #[prost(string, tag = "1")]
    pub lang: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod region_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct RegionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RegionClient<tonic::transport::Channel> {
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
    impl<T> RegionClient<T>
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
        ) -> RegionClient<InterceptedService<T, F>>
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
            RegionClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn region(
            &mut self,
            request: impl tonic::IntoRequest<super::RegionReq>,
        ) -> std::result::Result<tonic::Response<super::RegionReply>, tonic::Status> {
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
                "/bilibili.app.show.region.v1.Region/Region",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.show.region.v1.Region", "Region"));
            self.inner.unary(req, path, codec).await
        }
    }
}
