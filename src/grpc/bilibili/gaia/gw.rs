/// 待加密的pb对象
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceAppList {
    /// 上报类型
    /// first_installation:首次安装上报 first_open:每日启动上报
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// 安装的系统程序列表
    #[prost(string, repeated, tag = "2")]
    pub system_app_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 安装的用户程序列表
    #[prost(string, repeated, tag = "3")]
    pub user_app_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPublicKeyReply {
    /// 版本号
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// RSA公钥
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    /// 公钥过期时间
    #[prost(int64, tag = "3")]
    pub deadline: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaiaDeviceBasicInfo {
    /// 平台&应用信息
    ///
    /// android/ios/web/h5;
    #[prost(string, tag = "1")]
    pub platform: ::prost::alloc::string::String,
    /// 运行设备, 用于区分不同的app, 见客户端传入的对应参数。对于苹果系统，device有效值为phone, pad；安卓无法区分phone和pad，留空即可。
    #[prost(string, tag = "2")]
    pub device: ::prost::alloc::string::String,
    /// 包类型，用于区分不同的app, 见客户端传入的对应参数（mobi_app ）；对于web端请求，请传空
    #[prost(string, tag = "3")]
    pub mobi_app: ::prost::alloc::string::String,
    /// 客户端appkey, 用以区分不同的客户端，对应客户端请求参数中的appkey,如果无法获取可传空“”
    #[prost(string, tag = "4")]
    pub origin: ::prost::alloc::string::String,
    /// app产品编号 //产品编号，由数据平台分配，粉=1，白=2，蓝=3，直播姬=4，HD=5，海外=6，OTT=7，漫画=8，TV野版=9，小视频=10，网易漫画=11，网易漫画lite=12，网易漫画HD=13,国际版=14
    #[prost(string, tag = "5")]
    pub app_id: ::prost::alloc::string::String,
    /// 应用的版本信息
    ///
    /// SDK版本号   "sdkver": "2.6.6"
    #[prost(string, tag = "6")]
    pub sdkver: ::prost::alloc::string::String,
    /// app版本  "app_version":"5.36.0"
    #[prost(string, tag = "7")]
    pub app_version: ::prost::alloc::string::String,
    /// app版本号 "app_version_code":"5360000"
    #[prost(string, tag = "8")]
    pub app_version_code: ::prost::alloc::string::String,
    /// app版本号，见客户端传入的对应参数；对于web端请求，请传空
    #[prost(string, tag = "9")]
    pub build: ::prost::alloc::string::String,
    /// 渠道信息
    ///
    /// 渠道标识，见客户端传入的对应参数；对于web端请求，请传空；对应chid
    #[prost(string, tag = "10")]
    pub channel: ::prost::alloc::string::String,
    /// 机器硬件信息
    ///
    /// 手机品牌，见客户端传入的对应参数；
    #[prost(string, tag = "11")]
    pub brand: ::prost::alloc::string::String,
    /// 手机型号，见客户端传入的对应参数
    #[prost(string, tag = "12")]
    pub model: ::prost::alloc::string::String,
    /// 系统版本，见客户端传入的对应参数
    #[prost(string, tag = "13")]
    pub osver: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub user_agent: ::prost::alloc::string::String,
    /// 设备标识信息
    ///
    /// 本地设备唯一标识
    #[prost(string, tag = "15")]
    pub buvid_local: ::prost::alloc::string::String,
    /// 设备唯一标识
    #[prost(string, tag = "16")]
    pub buvid: ::prost::alloc::string::String,
    /// 登陆用户信息
    ///
    /// 最后一次登陆用户的mid，如果无登陆信息，传0即可
    #[prost(string, tag = "17")]
    pub mid: ::prost::alloc::string::String,
    /// 本次启动信息
    ///
    /// app首次启动时间 "fts":1530447775661
    #[prost(int64, tag = "18")]
    pub fts: i64,
    /// 是否首次启动 是：0 否：1
    #[prost(int32, tag = "19")]
    pub first: i32,
    /// 网络相关的信息
    ///
    /// 网络连接方式, WIFI/CELLULAR/OFFLINE/OTHERNET/ETHERNET "network":"WIFI", ESS_NETWORK_STATE、ACCESS_WIFI_STATE
    #[prost(string, tag = "20")]
    pub network: ::prost::alloc::string::String,
}
/// 应用列表上报-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaiaEncryptMsgReq {
    /// 上报头部
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<GaiaMsgHeader>,
    /// 加密数据
    #[prost(bytes = "vec", tag = "2")]
    pub encrypt_payload: ::prost::alloc::vec::Vec<u8>,
}
/// 风控通用消息头
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaiaMsgHeader {
    /// 加密类型
    #[prost(enumeration = "EncryptType", tag = "1")]
    pub encode_type: i32,
    /// 类型
    #[prost(enumeration = "PayloadType", tag = "2")]
    pub payload_type: i32,
    /// RAS加密后的aes_key
    #[prost(bytes = "vec", tag = "3")]
    pub encoded_aes_key: ::prost::alloc::vec::Vec<u8>,
    /// 当前时间戳(ms)
    #[prost(int64, tag = "4")]
    pub ts: i64,
}
/// 应用列表上报-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadAppListReply {
    /// 上报响应id
    #[prost(string, tag = "1")]
    pub trace_id: ::prost::alloc::string::String,
}
/// 加密方式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncryptType {
    /// 非法值
    InvalidEncryptType = 0,
    /// 同客户端人工约定AES加密私钥，存储在客户端
    ClientAes = 1,
    /// 客户端随机生成一个用于AES加密的私钥，并用服务端下发的RSA公钥来加密
    ServerRsaAes = 2,
}
impl EncryptType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncryptType::InvalidEncryptType => "INVALID_ENCRYPT_TYPE",
            EncryptType::ClientAes => "CLIENT_AES",
            EncryptType::ServerRsaAes => "SERVER_RSA_AES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_ENCRYPT_TYPE" => Some(Self::InvalidEncryptType),
            "CLIENT_AES" => Some(Self::ClientAes),
            "SERVER_RSA_AES" => Some(Self::ServerRsaAes),
            _ => None,
        }
    }
}
/// 负载类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadType {
    /// 非法值
    InvalidPayload = 0,
    /// 设备app列表，对应DeviceAppList
    DeviceAppList = 1,
}
impl PayloadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayloadType::InvalidPayload => "INVALID_PAYLOAD",
            PayloadType::DeviceAppList => "DEVICE_APP_LIST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_PAYLOAD" => Some(Self::InvalidPayload),
            "DEVICE_APP_LIST" => Some(Self::DeviceAppList),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod gaia_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 应用列表上报
    #[derive(Debug, Clone)]
    pub struct GaiaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GaiaClient<tonic::transport::Channel> {
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
    impl<T> GaiaClient<T>
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
        ) -> GaiaClient<InterceptedService<T, F>>
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
            GaiaClient::new(InterceptedService::new(inner, interceptor))
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
        /// 应用列表上报
        pub async fn ex_upload_app_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GaiaEncryptMsgReq>,
        ) -> std::result::Result<
            tonic::Response<super::UploadAppListReply>,
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
                "/bilibili.gaia.gw.Gaia/ExUploadAppList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.gaia.gw.Gaia", "ExUploadAppList"));
            self.inner.unary(req, path, codec).await
        }
        /// 拉取rsa公钥
        pub async fn ex_fetch_public_key(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<super::FetchPublicKeyReply>,
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
                "/bilibili.gaia.gw.Gaia/ExFetchPublicKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.gaia.gw.Gaia", "ExFetchPublicKey"));
            self.inner.unary(req, path, codec).await
        }
    }
}
