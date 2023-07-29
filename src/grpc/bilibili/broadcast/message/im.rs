///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyRsp {
    ///
    #[prost(uint64, tag = "1")]
    pub uid: u64,
    /// 命令id
    #[prost(uint64, tag = "2")]
    pub cmd: u64,
    ///
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    ///
    #[prost(enumeration = "PlType", tag = "4")]
    pub payload_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// 发送方uid
    #[prost(uint64, tag = "1")]
    pub sender_uid: u64,
    /// 接收方类型
    #[prost(int32, tag = "2")]
    pub receiver_type: i32,
    /// 接收方id
    #[prost(uint64, tag = "3")]
    pub receiver_id: u64,
    /// 客户端的序列id 用于服务端去重
    #[prost(uint64, tag = "4")]
    pub cli_msg_id: u64,
    /// 消息类型
    #[prost(int32, tag = "5")]
    pub msg_type: i32,
    /// 消息内容
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    /// 服务端的序列号
    #[prost(uint64, tag = "7")]
    pub msg_seqno: u64,
    /// 消息发送时间（服务端时间）
    #[prost(uint64, tag = "8")]
    pub timestamp: u64,
    /// at用户列表
    #[prost(uint64, repeated, tag = "9")]
    pub at_uids: ::prost::alloc::vec::Vec<u64>,
    /// 多人消息
    #[prost(uint64, repeated, tag = "10")]
    pub recver_ids: ::prost::alloc::vec::Vec<u64>,
    /// 消息唯一标示
    #[prost(uint64, tag = "11")]
    pub msg_key: u64,
    /// 消息状态
    #[prost(uint32, tag = "12")]
    pub msg_status: u32,
    /// 是否为系统撤销
    #[prost(bool, tag = "13")]
    pub sys_cancel: bool,
    /// 是否是多聊消息 目前群通知管理员的部分通知属于该类消息
    #[prost(uint32, tag = "14")]
    pub is_multi_chat: u32,
    /// 表示撤回的消息的session_seqno 用以后续的比较 实现未读数的正确显示
    #[prost(uint64, tag = "15")]
    pub withdraw_seqno: u64,
    /// 通知码
    #[prost(string, tag = "16")]
    pub notify_code: ::prost::alloc::string::String,
    /// 消息来源
    #[prost(uint32, tag = "17")]
    pub msg_source: u32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotifyInfo {
    ///
    #[prost(uint32, tag = "1")]
    pub msg_type: u32,
    ///
    #[prost(uint64, tag = "2")]
    pub talker_id: u64,
    ///
    #[prost(uint32, tag = "3")]
    pub session_type: u32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReqServerNotify {
    /// 最新序列号
    #[prost(uint64, tag = "1")]
    pub lastest_seqno: u64,
    /// 即时消息 该类消息主要用于系统通知 当客户端sync msg时 不会sync到此类消息
    #[prost(message, optional, tag = "2")]
    pub instant_msg: ::core::option::Option<Msg>,
    ///
    #[prost(message, optional, tag = "3")]
    pub notify_info: ::core::option::Option<NotifyInfo>,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlType {
    ///
    EnPayloadNormal = 0,
    ///
    EnPayloadBase64 = 1,
}
impl PlType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlType::EnPayloadNormal => "EN_PAYLOAD_NORMAL",
            PlType::EnPayloadBase64 => "EN_PAYLOAD_BASE64",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_PAYLOAD_NORMAL" => Some(Self::EnPayloadNormal),
            "EN_PAYLOAD_BASE64" => Some(Self::EnPayloadBase64),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CmdId {
    /// 非法cmd
    EnCmdIdInvalid = 0,
    /// 服务端主动发起
    EnCmdIdMsgNotify = 1,
    ///
    EnCmdIdKickOut = 2,
}
impl CmdId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CmdId::EnCmdIdInvalid => "EN_CMD_ID_INVALID",
            CmdId::EnCmdIdMsgNotify => "EN_CMD_ID_MSG_NOTIFY",
            CmdId::EnCmdIdKickOut => "EN_CMD_ID_KICK_OUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EN_CMD_ID_INVALID" => Some(Self::EnCmdIdInvalid),
            "EN_CMD_ID_MSG_NOTIFY" => Some(Self::EnCmdIdMsgNotify),
            "EN_CMD_ID_KICK_OUT" => Some(Self::EnCmdIdKickOut),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod notify_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct NotifyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NotifyClient<tonic::transport::Channel> {
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
    impl<T> NotifyClient<T>
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
        ) -> NotifyClient<InterceptedService<T, F>>
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
            NotifyClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn watch_notify(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NotifyRsp>>,
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
                "/bilibili.broadcast.message.im.Notify/WatchNotify",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.broadcast.message.im.Notify",
                        "WatchNotify",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
