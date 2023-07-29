/// 空请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoArgRequest {}
/// 空响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
/// 隐私设置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyConfigItem {
    /// 隐私开关类型
    #[prost(enumeration = "PrivacyConfigType", tag = "1")]
    pub privacy_config_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 隐私开关状态
    #[prost(enumeration = "PrivacyConfigState", tag = "3")]
    pub state: i32,
    ///
    #[prost(string, tag = "4")]
    pub sub_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub sub_title_uri: ::prost::alloc::string::String,
}
/// 获取隐私设置-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyConfigReply {
    /// 隐私设置
    #[prost(message, optional, tag = "1")]
    pub privacy_config_item: ::core::option::Option<PrivacyConfigItem>,
}
/// 修改隐私设置-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPrivacyConfigRequest {
    /// 隐私开关类型
    #[prost(enumeration = "PrivacyConfigType", tag = "1")]
    pub privacy_config_type: i32,
    /// 隐私开关状态
    #[prost(enumeration = "PrivacyConfigState", tag = "2")]
    pub state: i32,
}
/// 隐私开关状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivacyConfigState {
    /// 关闭
    Close = 0,
    /// 打开
    Open = 1,
}
impl PrivacyConfigState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrivacyConfigState::Close => "close",
            PrivacyConfigState::Open => "open",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "close" => Some(Self::Close),
            "open" => Some(Self::Open),
            _ => None,
        }
    }
}
/// 隐私开关类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivacyConfigType {
    ///
    None = 0,
    /// 动态同城
    DynamicCity = 1,
}
impl PrivacyConfigType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrivacyConfigType::None => "none",
            PrivacyConfigType::DynamicCity => "dynamic_city",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "none" => Some(Self::None),
            "dynamic_city" => Some(Self::DynamicCity),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod privacy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 隐私
    #[derive(Debug, Clone)]
    pub struct PrivacyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PrivacyClient<tonic::transport::Channel> {
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
    impl<T> PrivacyClient<T>
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
        ) -> PrivacyClient<InterceptedService<T, F>>
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
            PrivacyClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取隐私设置
        pub async fn privacy_config(
            &mut self,
            request: impl tonic::IntoRequest<super::NoArgRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PrivacyConfigReply>,
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
                "/bilibili.app.resource.privacy.v1.Privacy/PrivacyConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.resource.privacy.v1.Privacy",
                        "PrivacyConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 修改隐私设置
        pub async fn set_privacy_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPrivacyConfigRequest>,
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
                "/bilibili.app.resource.privacy.v1.Privacy/SetPrivacyConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.app.resource.privacy.v1.Privacy",
                        "SetPrivacyConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
