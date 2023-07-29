/// 账户信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    ///
    #[prost(uint64, tag = "1")]
    pub mid: u64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppInfo {
    ///
    #[prost(string, tag = "1")]
    pub top_page_class: ::prost::alloc::string::String,
    /// 客户端首次启动时的毫秒时间戳
    #[prost(int64, tag = "2")]
    pub ftime: i64,
    ///
    #[prost(string, tag = "3")]
    pub did: ::prost::alloc::string::String,
}
/// 心跳补充信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    ///
    #[prost(string, tag = "1")]
    pub session: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub refer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartBeatReq {
    ///
    #[prost(string, tag = "1")]
    pub session_v2: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "Stage", tag = "2")]
    pub stage: i32,
    /// 流加载失败timeout
    #[prost(uint64, tag = "3")]
    pub stream_timeout: u64,
    ///
    #[prost(uint64, tag = "4")]
    pub batch_frequency: u64,
    ///
    #[prost(float, tag = "5")]
    pub frequency: f32,
    ///
    #[prost(message, optional, tag = "6")]
    pub video_meta: ::core::option::Option<VideoMeta>,
    ///
    #[prost(message, optional, tag = "7")]
    pub app_info: ::core::option::Option<AppInfo>,
    ///
    #[prost(message, optional, tag = "8")]
    pub account_info: ::core::option::Option<AccountInfo>,
    ///
    #[prost(message, optional, tag = "9")]
    pub pre_process_result: ::core::option::Option<PreProcessResult>,
    ///
    #[prost(message, repeated, tag = "10")]
    pub player_status: ::prost::alloc::vec::Vec<PlayerStatus>,
    ///
    #[prost(message, optional, tag = "11")]
    pub video_info: ::core::option::Option<VideoInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerStatus {
    ///
    #[prost(float, tag = "1")]
    pub playback_rate: f32,
    ///
    #[prost(uint64, tag = "2")]
    pub progress: u64,
    ///
    #[prost(enumeration = "PlayState", tag = "3")]
    pub play_state: i32,
    ///
    #[prost(bool, tag = "4")]
    pub is_buffering: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreProcessResult {
    ///
    #[prost(int64, tag = "1")]
    pub vt: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoInfo {
    ///
    #[prost(uint64, tag = "1")]
    pub cid_duration: u64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMeta {
    ///
    #[prost(uint64, tag = "1")]
    pub aid: u64,
    ///
    #[prost(uint64, tag = "2")]
    pub cid: u64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayState {
    ///
    StateUnknown = 0,
    ///
    Preparing = 1,
    ///
    Prepared = 2,
    ///
    Playing = 3,
    ///
    Paused = 4,
    ///
    Stopped = 5,
    ///
    Failed = 6,
}
impl PlayState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayState::StateUnknown => "STATE_UNKNOWN",
            PlayState::Preparing => "PREPARING",
            PlayState::Prepared => "PREPARED",
            PlayState::Playing => "PLAYING",
            PlayState::Paused => "PAUSED",
            PlayState::Stopped => "STOPPED",
            PlayState::Failed => "FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_UNKNOWN" => Some(Self::StateUnknown),
            "PREPARING" => Some(Self::Preparing),
            "PREPARED" => Some(Self::Prepared),
            "PLAYING" => Some(Self::Playing),
            "PAUSED" => Some(Self::Paused),
            "STOPPED" => Some(Self::Stopped),
            "FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Stage {
    ///
    Unknown = 0,
    ///
    Start = 1,
    ///
    End = 2,
    ///
    Sample = 3,
}
impl Stage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Stage::Unknown => "STAGE_UNKNOWN",
            Stage::Start => "START",
            Stage::End => "END",
            Stage::Sample => "SAMPLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STAGE_UNKNOWN" => Some(Self::Unknown),
            "START" => Some(Self::Start),
            "END" => Some(Self::End),
            "SAMPLE" => Some(Self::Sample),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod click_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ClickClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClickClient<tonic::transport::Channel> {
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
    impl<T> ClickClient<T>
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
        ) -> ClickClient<InterceptedService<T, F>>
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
            ClickClient::new(InterceptedService::new(inner, interceptor))
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
    }
}
