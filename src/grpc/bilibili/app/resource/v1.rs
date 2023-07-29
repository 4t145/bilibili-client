#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReply {
    ///
    #[prost(string, tag = "1")]
    pub env: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<PoolReply>,
    ///
    #[prost(int64, tag = "3")]
    pub list_version: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReq {
    ///
    #[prost(string, tag = "1")]
    pub pool_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub version_list: ::prost::alloc::vec::Vec<VersionListReq>,
    ///
    #[prost(enumeration = "EnvType", tag = "4")]
    pub env: i32,
    ///
    #[prost(int32, tag = "5")]
    pub sys_ver: i32,
    ///
    #[prost(int32, tag = "6")]
    pub scale: i32,
    ///
    #[prost(int32, tag = "7")]
    pub arch: i32,
    ///
    #[prost(int64, tag = "8")]
    pub list_version: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleReply {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub version: i64,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub md5: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub total_md5: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "IncrementType", tag = "6")]
    pub increment: i32,
    ///
    #[prost(bool, tag = "7")]
    pub is_wifi: bool,
    ///
    #[prost(enumeration = "LevelType", tag = "8")]
    pub level: i32,
    ///
    #[prost(string, tag = "9")]
    pub filename: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub file_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub file_size: i64,
    ///
    #[prost(enumeration = "CompressType", tag = "12")]
    pub compress: i32,
    ///
    #[prost(int64, tag = "13")]
    pub publish_time: i64,
    /// 上报使用
    #[prost(int64, tag = "14")]
    pub pool_id: i64,
    ///
    #[prost(int64, tag = "15")]
    pub module_id: i64,
    ///
    #[prost(int64, tag = "16")]
    pub version_id: i64,
    ///
    #[prost(int64, tag = "17")]
    pub file_id: i64,
    ///
    #[prost(bool, tag = "18")]
    pub zip_check: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolReply {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub modules: ::prost::alloc::vec::Vec<ModuleReply>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionListReq {
    ///
    #[prost(string, tag = "1")]
    pub pool_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<VersionReq>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReq {
    ///
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub version: i64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressType {
    /// unzip
    Unzip = 0,
    /// 不操作
    Original = 1,
}
impl CompressType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompressType::Unzip => "Unzip",
            CompressType::Original => "Original",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unzip" => Some(Self::Unzip),
            "Original" => Some(Self::Original),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnvType {
    ///
    Unknown = 0,
    ///
    Release = 1,
    ///
    Test = 2,
}
impl EnvType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnvType::Unknown => "Unknown",
            EnvType::Release => "Release",
            EnvType::Test => "Test",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Release" => Some(Self::Release),
            "Test" => Some(Self::Test),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IncrementType {
    /// 全量包
    Total = 0,
    /// 增量包
    Incremental = 1,
}
impl IncrementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IncrementType::Total => "Total",
            IncrementType::Incremental => "Incremental",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Total" => Some(Self::Total),
            "Incremental" => Some(Self::Incremental),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LevelType {
    ///
    Undefined = 0,
    /// 高 需立即下载
    High = 1,
    /// 中 可以延迟下载
    Middle = 2,
    /// 低 仅在业务方使用到时由业务方手动进行下载
    Low = 3,
}
impl LevelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LevelType::Undefined => "Undefined",
            LevelType::High => "High",
            LevelType::Middle => "Middle",
            LevelType::Low => "Low",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Undefined" => Some(Self::Undefined),
            "High" => Some(Self::High),
            "Middle" => Some(Self::Middle),
            "Low" => Some(Self::Low),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod module_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct ModuleClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModuleClient<tonic::transport::Channel> {
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
    impl<T> ModuleClient<T>
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
        ) -> ModuleClient<InterceptedService<T, F>>
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
            ModuleClient::new(InterceptedService::new(inner, interceptor))
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
            request: impl tonic::IntoRequest<super::ListReq>,
        ) -> std::result::Result<tonic::Response<super::ListReply>, tonic::Status> {
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
                "/bilibili.app.resource.v1.Module/List",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.resource.v1.Module", "List"));
            self.inner.unary(req, path, codec).await
        }
    }
}
