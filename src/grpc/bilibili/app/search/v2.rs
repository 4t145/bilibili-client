///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelChatTaskReq {
    ///
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub from_source: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelChatTaskReply {
    ///
    #[prost(int32, tag = "1")]
    pub code: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChatResultReq {
    ///
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub from_source: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEggInfo {
    ///
    #[prost(int32, tag = "1")]
    pub egg_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub id: i64,
    ///
    #[prost(int32, tag = "3")]
    pub is_commercial: i32,
    ///
    #[prost(string, tag = "4")]
    pub mask_color: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub mask_transparency: i64,
    ///
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub re_type: i32,
    ///
    #[prost(string, tag = "8")]
    pub re_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub re_value: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub show_count: i32,
    ///
    #[prost(int64, tag = "11")]
    pub size: i64,
    ///
    #[prost(int64, tag = "12")]
    pub source: i64,
    ///
    #[prost(string, tag = "13")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEggInfos {
    ///
    #[prost(message, repeated, tag = "1")]
    pub egg_info: ::prost::alloc::vec::Vec<SearchEggInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEggReply {
    ///
    #[prost(int32, tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub seid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<SearchEggInfos>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEggReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitChatTaskReply {
    ///
    #[prost(int32, tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitChatTaskReq {
    ///
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub from_source: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod search_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SearchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchClient<tonic::transport::Channel> {
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
    impl<T> SearchClient<T>
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
        ) -> SearchClient<InterceptedService<T, F>>
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
            SearchClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn cancel_chat_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelChatTaskReq>,
        ) -> std::result::Result<
            tonic::Response<super::CancelChatTaskReply>,
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
                "/bilibili.app.search.v2.Search/CancelChatTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.search.v2.Search", "CancelChatTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_chat_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChatResultReq>,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::broadcast::message::main::ChatResult,
            >,
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
                "/bilibili.app.search.v2.Search/GetChatResult",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.search.v2.Search", "GetChatResult"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn search_egg(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchEggReq>,
        ) -> std::result::Result<tonic::Response<super::SearchEggReply>, tonic::Status> {
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
                "/bilibili.app.search.v2.Search/SearchEgg",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.search.v2.Search", "SearchEgg"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn submit_chat_task(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitChatTaskReq>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitChatTaskReply>,
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
                "/bilibili.app.search.v2.Search/SubmitChatTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.search.v2.Search", "SubmitChatTask"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
