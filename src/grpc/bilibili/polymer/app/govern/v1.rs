///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AntiHarassmentInfo {
    ///
    #[prost(int32, tag = "1")]
    pub limit: i32,
    ///
    #[prost(int32, tag = "2")]
    pub follow_time_limit_second: i32,
    ///
    #[prost(int64, tag = "3")]
    pub expire_time: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AntiHarassmentSetting {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(bool, tag = "2")]
    pub auto_limit: bool,
    ///
    #[prost(message, optional, tag = "3")]
    pub im: ::core::option::Option<AntiHarassmentInfo>,
    ///
    #[prost(message, optional, tag = "4")]
    pub reply: ::core::option::Option<AntiHarassmentInfo>,
    ///
    #[prost(message, optional, tag = "5")]
    pub dm: ::core::option::Option<AntiHarassmentInfo>,
    ///
    #[prost(message, optional, tag = "6")]
    pub reply_me: ::core::option::Option<AntiHarassmentInfo>,
    ///
    #[prost(message, optional, tag = "7")]
    pub like_me: ::core::option::Option<AntiHarassmentInfo>,
    ///
    #[prost(message, optional, tag = "8")]
    pub at_me: ::core::option::Option<AntiHarassmentInfo>,
    ///
    #[prost(int64, tag = "9")]
    pub auto_limit_expire_time: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadAntiHarassmentSettingsReq {
    ///
    #[prost(int32, tag = "1")]
    pub biz_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub recv_mid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub send_mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadAntiHarassmentSettingsRsp {
    ///
    #[prost(bool, tag = "1")]
    pub anti_harassment_ret: bool,
    ///
    #[prost(message, optional, tag = "2")]
    pub anti_harassment_setting: ::core::option::Option<AntiHarassmentSetting>,
    ///
    #[prost(int32, tag = "3")]
    pub show_window: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreAntiHarassmentSettingsReq {
    ///
    #[prost(int32, tag = "1")]
    pub biz_type: i32,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(message, optional, tag = "3")]
    pub anti_harassment_setting: ::core::option::Option<AntiHarassmentSetting>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreAntiHarassmentSettingsRsp {}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AntiHarassmentLimit {
    ///
    DefaultLimit = 0,
    ///
    FollowLimit = 1,
    ///
    ReFollowLimit = 2,
    ///
    TwoWayFollow = 3,
    ///
    AllLimit = 4,
}
impl AntiHarassmentLimit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AntiHarassmentLimit::DefaultLimit => "DefaultLimit",
            AntiHarassmentLimit::FollowLimit => "FollowLimit",
            AntiHarassmentLimit::ReFollowLimit => "ReFollowLimit",
            AntiHarassmentLimit::TwoWayFollow => "TwoWayFollow",
            AntiHarassmentLimit::AllLimit => "AllLimit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DefaultLimit" => Some(Self::DefaultLimit),
            "FollowLimit" => Some(Self::FollowLimit),
            "ReFollowLimit" => Some(Self::ReFollowLimit),
            "TwoWayFollow" => Some(Self::TwoWayFollow),
            "AllLimit" => Some(Self::AllLimit),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BizType {
    ///
    InvalidBizType = 0,
    ///
    Im = 1,
    ///
    Dm = 2,
    ///
    Reply = 3,
    ///
    ReplyMe = 4,
    ///
    LikeMe = 5,
    ///
    AtMe = 6,
}
impl BizType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BizType::InvalidBizType => "InvalidBizType",
            BizType::Im => "Im",
            BizType::Dm => "Dm",
            BizType::Reply => "Reply",
            BizType::ReplyMe => "ReplyMe",
            BizType::LikeMe => "LikeMe",
            BizType::AtMe => "AtMe",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidBizType" => Some(Self::InvalidBizType),
            "Im" => Some(Self::Im),
            "Dm" => Some(Self::Dm),
            "Reply" => Some(Self::Reply),
            "ReplyMe" => Some(Self::ReplyMe),
            "LikeMe" => Some(Self::LikeMe),
            "AtMe" => Some(Self::AtMe),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod anti_harassment_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 反骚扰
    #[derive(Debug, Clone)]
    pub struct AntiHarassmentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AntiHarassmentServiceClient<tonic::transport::Channel> {
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
    impl<T> AntiHarassmentServiceClient<T>
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
        ) -> AntiHarassmentServiceClient<InterceptedService<T, F>>
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
            AntiHarassmentServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn store_anti_harassment_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::StoreAntiHarassmentSettingsReq>,
        ) -> std::result::Result<
            tonic::Response<super::StoreAntiHarassmentSettingsRsp>,
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
                "/bilibili.polymer.app.govern.v1.AntiHarassmentService/StoreAntiHarassmentSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.polymer.app.govern.v1.AntiHarassmentService",
                        "StoreAntiHarassmentSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn load_anti_harassment_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::LoadAntiHarassmentSettingsReq>,
        ) -> std::result::Result<
            tonic::Response<super::LoadAntiHarassmentSettingsRsp>,
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
                "/bilibili.polymer.app.govern.v1.AntiHarassmentService/LoadAntiHarassmentSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.polymer.app.govern.v1.AntiHarassmentService",
                        "LoadAntiHarassmentSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
