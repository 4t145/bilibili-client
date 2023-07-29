///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserPreferenceReq {
    ///
    #[prost(string, repeated, tag = "1")]
    pub type_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(map = "string, string", tag = "2")]
    pub extra_context: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserPreferenceReply {
    /// 对应 GetUserPreferenceReq 的请求的类型
    #[prost(message, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<
        super::super::super::super::google::protobuf::Any,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUserPreferenceReq {
    ///
    #[prost(message, repeated, tag = "1")]
    pub preference: ::prost::alloc::vec::Vec<
        super::super::super::super::google::protobuf::Any,
    >,
    ///
    #[prost(map = "string, string", tag = "2")]
    pub extra_context: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetUserPreferenceReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPreferenceReq {}
/// 云控配置下发
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPreferenceReply {
    /// 具体解码需要根据实际请求 type_url 来判断
    #[prost(message, repeated, tag = "1")]
    pub preference: ::prost::alloc::vec::Vec<
        super::super::super::super::google::protobuf::Any,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolValue {
    ///
    #[prost(bool, tag = "1")]
    pub value: bool,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(bool, tag = "3")]
    pub default_value: bool,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesValue {
    ///
    #[prost(bytes = "vec", tag = "1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(bytes = "vec", tag = "3")]
    pub default_value: ::prost::alloc::vec::Vec<u8>,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleValue {
    ///
    #[prost(double, tag = "1")]
    pub value: f64,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(double, tag = "3")]
    pub default_value: f64,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatValue {
    ///
    #[prost(float, tag = "1")]
    pub value: f32,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(float, tag = "3")]
    pub default_value: f32,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Value {
    ///
    #[prost(int32, tag = "1")]
    pub value: i32,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(int32, tag = "3")]
    pub default_value: i32,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Value {
    ///
    #[prost(int64, tag = "1")]
    pub value: i64,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(int64, tag = "3")]
    pub default_value: i64,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringValue {
    ///
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(string, tag = "3")]
    pub default_value: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Value {
    ///
    #[prost(uint32, tag = "1")]
    pub value: u32,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(uint32, tag = "3")]
    pub default_value: u32,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt64Value {
    ///
    #[prost(uint64, tag = "1")]
    pub value: u64,
    ///
    #[prost(int64, tag = "2")]
    pub last_modified: i64,
    ///
    #[prost(uint64, tag = "3")]
    pub default_value: u64,
    ///
    #[prost(string, tag = "4")]
    pub exp: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod distribution_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// APP配置
    #[derive(Debug, Clone)]
    pub struct DistributionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DistributionClient<tonic::transport::Channel> {
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
    impl<T> DistributionClient<T>
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
        ) -> DistributionClient<InterceptedService<T, F>>
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
            DistributionClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取云端储存的用户偏好
        pub async fn get_user_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserPreferenceReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserPreferenceReply>,
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
                "/bilibili.app.distribution.v1.Distribution/GetUserPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.distribution.v1.Distribution",
                        "GetUserPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 设定用户偏好
        pub async fn set_user_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::SetUserPreferenceReq>,
        ) -> std::result::Result<
            tonic::Response<super::SetUserPreferenceReply>,
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
                "/bilibili.app.distribution.v1.Distribution/SetUserPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.distribution.v1.Distribution",
                        "SetUserPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 获取云控配置
        pub async fn user_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::UserPreferenceReq>,
        ) -> std::result::Result<
            tonic::Response<super::UserPreferenceReply>,
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
                "/bilibili.app.distribution.v1.Distribution/UserPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.distribution.v1.Distribution",
                        "UserPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
