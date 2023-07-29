///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeReply {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub id1: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub code: i64,
    ///
    #[prost(string, tag = "4")]
    pub message_s: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeReq {
    ///
    #[prost(int64, tag = "1")]
    pub code: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopic {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatTask {
    ///
    #[prost(string, tag = "1")]
    pub task: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicMessageUpdate {
    ///
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<SimpleMessage>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Embedded {
    ///
    #[prost(bool, tag = "1")]
    pub bool_val: bool,
    ///
    #[prost(int32, tag = "2")]
    pub int32_val: i32,
    ///
    #[prost(int64, tag = "3")]
    pub int64_val: i64,
    ///
    #[prost(float, tag = "4")]
    pub float_val: f32,
    ///
    #[prost(double, tag = "5")]
    pub double_val: f64,
    ///
    #[prost(string, tag = "6")]
    pub string_val: ::prost::alloc::string::String,
    ///
    #[prost(bool, repeated, tag = "7")]
    pub repeated_bool_val: ::prost::alloc::vec::Vec<bool>,
    ///
    #[prost(int32, repeated, tag = "8")]
    pub repeated_int32_val: ::prost::alloc::vec::Vec<i32>,
    ///
    #[prost(int64, repeated, tag = "9")]
    pub repeated_int64_val: ::prost::alloc::vec::Vec<i64>,
    ///
    #[prost(float, repeated, tag = "10")]
    pub repeated_float_val: ::prost::alloc::vec::Vec<f32>,
    ///
    #[prost(double, repeated, tag = "11")]
    pub repeated_double_val: ::prost::alloc::vec::Vec<f64>,
    ///
    #[prost(string, repeated, tag = "12")]
    pub repeated_string_val: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(map = "string, string", tag = "13")]
    pub map_string_val: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    ///
    #[prost(map = "string, message", tag = "14")]
    pub map_error_val: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ErrorMessage,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorMessage {
    ///
    #[prost(int64, tag = "1")]
    pub code: i64,
    ///
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeReply {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub buvid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeStreamReply {
    ///
    #[prost(int64, tag = "1")]
    pub sequence: i64,
    ///
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
    ///
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeStreamReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub sequence: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeSubReply {
    ///
    #[prost(int64, tag = "1")]
    pub message_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProbeSubReq {
    ///
    #[prost(string, tag = "1")]
    pub buvid: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleMessage {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(int64, tag = "2")]
    pub num: i64,
    ///
    #[prost(string, tag = "3")]
    pub lang: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub cate: i32,
    ///
    #[prost(message, optional, tag = "5")]
    pub embedded: ::core::option::Option<Embedded>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "3")]
    pub cache: bool,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Category {
    ///
    Unspecified = 0,
    ///
    One = 1,
    ///
    Two = 2,
    ///
    Three = 3,
    ///
    Four = 4,
}
impl Category {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Category::Unspecified => "CATEGORY_UNSPECIFIED",
            Category::One => "CATEGORY_ONE",
            Category::Two => "CATEGORY_TWO",
            Category::Three => "CATEGORY_THREE",
            Category::Four => "CATEGORY_FOUR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
            "CATEGORY_ONE" => Some(Self::One),
            "CATEGORY_TWO" => Some(Self::Two),
            "CATEGORY_THREE" => Some(Self::Three),
            "CATEGORY_FOUR" => Some(Self::Four),
            _ => None,
        }
    }
}
/// Deprecated
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorReason {
    ///
    ProbeUnspecified = 0,
    ///
    ProbeCategoryNotfound = 1,
}
impl ErrorReason {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorReason::ProbeUnspecified => "PROBE_UNSPECIFIED",
            ErrorReason::ProbeCategoryNotfound => "PROBE_CATEGORY_NOTFOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROBE_UNSPECIFIED" => Some(Self::ProbeUnspecified),
            "PROBE_CATEGORY_NOTFOUND" => Some(Self::ProbeCategoryNotfound),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod probe_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 服务可用性探针
    #[derive(Debug, Clone)]
    pub struct ProbeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProbeClient<tonic::transport::Channel> {
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
    impl<T> ProbeClient<T>
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
        ) -> ProbeClient<InterceptedService<T, F>>
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
            ProbeClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn test_code(
            &mut self,
            request: impl tonic::IntoRequest<super::CodeReq>,
        ) -> std::result::Result<tonic::Response<super::CodeReply>, tonic::Status> {
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
                "/bilibili.api.probe.v1.Probe/TestCode",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.probe.v1.Probe", "TestCode"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn test_req(
            &mut self,
            request: impl tonic::IntoRequest<super::ProbeReq>,
        ) -> std::result::Result<tonic::Response<super::ProbeReply>, tonic::Status> {
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
                "/bilibili.api.probe.v1.Probe/TestReq",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.probe.v1.Probe", "TestReq"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn test_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::ProbeStreamReq>,
        ) -> std::result::Result<
            tonic::Response<super::ProbeStreamReply>,
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
                "/bilibili.api.probe.v1.Probe/TestStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.probe.v1.Probe", "TestStream"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn test_sub(
            &mut self,
            request: impl tonic::IntoRequest<super::ProbeSubReq>,
        ) -> std::result::Result<tonic::Response<super::ProbeSubReply>, tonic::Status> {
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
                "/bilibili.api.probe.v1.Probe/TestSub",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.probe.v1.Probe", "TestSub"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod probe_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 服务可用性探针
    #[derive(Debug, Clone)]
    pub struct ProbeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProbeServiceClient<tonic::transport::Channel> {
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
    impl<T> ProbeServiceClient<T>
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
        ) -> ProbeServiceClient<InterceptedService<T, F>>
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
            ProbeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn echo(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleMessage>,
        ) -> std::result::Result<tonic::Response<super::SimpleMessage>, tonic::Status> {
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
                "/bilibili.api.probe.v1.ProbeService/Echo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.probe.v1.ProbeService", "Echo"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn echo_body(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleMessage>,
        ) -> std::result::Result<tonic::Response<super::SimpleMessage>, tonic::Status> {
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
                "/bilibili.api.probe.v1.ProbeService/EchoBody",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.api.probe.v1.ProbeService", "EchoBody"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn echo_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleMessage>,
        ) -> std::result::Result<tonic::Response<super::SimpleMessage>, tonic::Status> {
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
                "/bilibili.api.probe.v1.ProbeService/EchoDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.api.probe.v1.ProbeService", "EchoDelete"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn echo_error(
            &mut self,
            request: impl tonic::IntoRequest<super::ErrorMessage>,
        ) -> std::result::Result<tonic::Response<super::ErrorMessage>, tonic::Status> {
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
                "/bilibili.api.probe.v1.ProbeService/EchoError",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.api.probe.v1.ProbeService", "EchoError"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn echo_patch(
            &mut self,
            request: impl tonic::IntoRequest<super::DynamicMessageUpdate>,
        ) -> std::result::Result<
            tonic::Response<super::DynamicMessageUpdate>,
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
                "/bilibili.api.probe.v1.ProbeService/EchoPatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.api.probe.v1.ProbeService", "EchoPatch"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
