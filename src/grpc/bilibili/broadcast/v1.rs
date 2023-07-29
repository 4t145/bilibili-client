///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageBlackList {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageView {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushMessageResp {
    /// Deprecated: 推送任务id，使用string
    #[prost(int64, tag = "1")]
    pub old_taskid: i64,
    /// 业务
    /// 1:是视频 2:是直播 3:是活动
    #[prost(enumeration = "push_message_resp::Biz", tag = "2")]
    pub biz: i32,
    /// 类型
    /// 1:是默认 2:是热门 3:是实时 4:是推荐
    #[prost(enumeration = "push_message_resp::Type", tag = "3")]
    pub r#type: i32,
    /// 主标题
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    /// 副标题
    #[prost(string, tag = "5")]
    pub summary: ::prost::alloc::string::String,
    /// 图片地址
    #[prost(string, tag = "6")]
    pub img: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "7")]
    pub link: ::prost::alloc::string::String,
    /// 展示位置，1是顶部
    #[prost(enumeration = "push_message_resp::Position", tag = "8")]
    pub position: i32,
    /// 展示时长（单位：秒），默认3秒
    #[prost(int32, tag = "9")]
    pub duration: i32,
    /// 失效时间
    #[prost(int64, tag = "10")]
    pub expire: i64,
    /// 推送任务id
    #[prost(string, tag = "11")]
    pub taskid: ::prost::alloc::string::String,
    /// 应用内推送黑名单
    /// UGC:     ugc-video-detail
    /// PGC:     pgc-video-detail
    /// 一起看:   pgc-video-detail-theater
    /// 直播:     live-room-detail
    /// Story:    ugc-video-detail-vertical
    /// 播单黑名单 playlist-video-detail
    #[prost(message, repeated, tag = "12")]
    pub page_black_list: ::prost::alloc::vec::Vec<PageBlackList>,
    /// 预留pvid
    #[prost(message, repeated, tag = "13")]
    pub page_view: ::prost::alloc::vec::Vec<PageView>,
    /// 跳转资源
    #[prost(message, optional, tag = "14")]
    pub target_resource: ::core::option::Option<TargetResource>,
    ///
    #[prost(int32, tag = "15")]
    pub image_frame: i32,
    ///
    #[prost(int32, tag = "16")]
    pub image_marker: i32,
    ///
    #[prost(int32, tag = "17")]
    pub image_position: i32,
    ///
    #[prost(int64, tag = "18")]
    pub job: i64,
}
/// Nested message and enum types in `PushMessageResp`.
pub mod push_message_resp {
    /// 业务类型
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Biz {
        /// 未知
        Unknown = 0,
        /// 视频
        Video = 1,
        /// 直播
        Live = 2,
        /// 活动
        Activity = 3,
    }
    impl Biz {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Biz::Unknown => "BIZ_UNKNOWN",
                Biz::Video => "BIZ_VIDEO",
                Biz::Live => "BIZ_LIVE",
                Biz::Activity => "BIZ_ACTIVITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BIZ_UNKNOWN" => Some(Self::Unknown),
                "BIZ_VIDEO" => Some(Self::Video),
                "BIZ_LIVE" => Some(Self::Live),
                "BIZ_ACTIVITY" => Some(Self::Activity),
                _ => None,
            }
        }
    }
    /// 消息类型
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// 未知
        Unknown = 0,
        /// 默认
        Default = 1,
        /// 热门
        Hot = 2,
        /// 实时
        Realtime = 3,
        /// 推荐
        Recommend = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "TYPE_UNKNOWN",
                Type::Default => "TYPE_DEFAULT",
                Type::Hot => "TYPE_HOT",
                Type::Realtime => "TYPE_REALTIME",
                Type::Recommend => "TYPE_RECOMMEND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNKNOWN" => Some(Self::Unknown),
                "TYPE_DEFAULT" => Some(Self::Default),
                "TYPE_HOT" => Some(Self::Hot),
                "TYPE_REALTIME" => Some(Self::Realtime),
                "TYPE_RECOMMEND" => Some(Self::Recommend),
                _ => None,
            }
        }
    }
    /// 展示未知
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Position {
        /// 未知
        PosUnknown = 0,
        /// 顶部
        PosTop = 1,
    }
    impl Position {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Position::PosUnknown => "POS_UNKNOWN",
                Position::PosTop => "POS_TOP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POS_UNKNOWN" => Some(Self::PosUnknown),
                "POS_TOP" => Some(Self::PosTop),
                _ => None,
            }
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetResource {
    /// 直播:   roomid
    /// UGC:   avid
    /// PGC:   seasonid
    /// Story: avid
    /// 举个例子
    /// Type: LINK_TYPE_BANGUMI (番剧)
    /// Resource: {"seasonid":"123"}
    ///
    /// Type: LINK_TYPE_VIDEO (视频)
    /// Resource: {"avid":"123"}
    ///
    /// Type: LINK_TYPE_LIVE (直播)
    /// Resource: {"roomid":"123"}
    ///
    #[prost(enumeration = "LinkType", tag = "1")]
    pub r#type: i32,
    ///
    #[prost(map = "string, string", tag = "2")]
    pub resource: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LinkType {
    /// 未知
    Unknown = 0,
    /// 番剧
    Bangumi = 1,
    /// 视频
    Video = 2,
    /// 直播
    Live = 3,
}
impl LinkType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LinkType::Unknown => "LINK_TYPE_UNKNOWN",
            LinkType::Bangumi => "LINK_TYPE_BANGUMI",
            LinkType::Video => "LINK_TYPE_VIDEO",
            LinkType::Live => "LINK_TYPE_LIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINK_TYPE_UNKNOWN" => Some(Self::Unknown),
            "LINK_TYPE_BANGUMI" => Some(Self::Bangumi),
            "LINK_TYPE_VIDEO" => Some(Self::Video),
            "LINK_TYPE_LIVE" => Some(Self::Live),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod push_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Push
    #[derive(Debug, Clone)]
    pub struct PushClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PushClient<tonic::transport::Channel> {
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
    impl<T> PushClient<T>
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
        ) -> PushClient<InterceptedService<T, F>>
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
            PushClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn watch_message(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PushMessageResp>>,
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
                "/bilibili.broadcast.v1.Push/WatchMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.Push", "WatchMessage"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// 服务端下发日志上报事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LaserLogUploadResp {
    /// 任务id
    #[prost(int64, tag = "1")]
    pub taskid: i64,
    /// 下发时间
    #[prost(string, tag = "2")]
    pub date: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod laser_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Laser
    #[derive(Debug, Clone)]
    pub struct LaserClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LaserClient<tonic::transport::Channel> {
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
    impl<T> LaserClient<T>
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
        ) -> LaserClient<InterceptedService<T, F>>
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
            LaserClient::new(InterceptedService::new(inner, interceptor))
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
        /// 监听上报事件
        pub async fn watch_log_upload_event(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::LaserLogUploadResp>>,
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
                "/bilibili.broadcast.v1.Laser/WatchLogUploadEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.broadcast.v1.Laser", "WatchLogUploadEvent"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomErrorEvent {
    ///
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomJoinEvent {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomLeaveEvent {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomMessageEvent {
    ///
    #[prost(string, tag = "1")]
    pub target_path: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub body: ::core::option::Option<super::super::super::google::protobuf::Any>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomOnlineEvent {
    ///
    #[prost(int32, tag = "1")]
    pub online: i32,
    ///
    #[prost(int32, tag = "2")]
    pub all_online: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomReq {
    /// {type}://{room_id}
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "room_req::Event", tags = "2, 3, 4, 5")]
    pub event: ::core::option::Option<room_req::Event>,
}
/// Nested message and enum types in `RoomReq`.
pub mod room_req {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        ///
        #[prost(message, tag = "2")]
        Join(super::RoomJoinEvent),
        ///
        #[prost(message, tag = "3")]
        Leave(super::RoomLeaveEvent),
        ///
        #[prost(message, tag = "4")]
        Online(super::RoomOnlineEvent),
        ///
        #[prost(message, tag = "5")]
        Msg(super::RoomMessageEvent),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoomResp {
    /// {type}://{room_id}
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "room_resp::Event", tags = "2, 3, 4, 5, 6")]
    pub event: ::core::option::Option<room_resp::Event>,
}
/// Nested message and enum types in `RoomResp`.
pub mod room_resp {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        ///
        #[prost(message, tag = "2")]
        Join(super::RoomJoinEvent),
        ///
        #[prost(message, tag = "3")]
        Leave(super::RoomLeaveEvent),
        ///
        #[prost(message, tag = "4")]
        Online(super::RoomOnlineEvent),
        ///
        #[prost(message, tag = "5")]
        Msg(super::RoomMessageEvent),
        ///
        #[prost(message, tag = "6")]
        Err(super::RoomErrorEvent),
    }
}
/// Generated client implementations.
pub mod broadcast_room_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct BroadcastRoomClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BroadcastRoomClient<tonic::transport::Channel> {
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
    impl<T> BroadcastRoomClient<T>
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
        ) -> BroadcastRoomClient<InterceptedService<T, F>>
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
            BroadcastRoomClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn enter(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::RoomReq>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::RoomResp>>,
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
                "/bilibili.broadcast.v1.BroadcastRoom/Enter",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.BroadcastRoom", "Enter"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// 鉴权请求，通过authorization验证绑定用户mid
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthReq {
    /// 冷启动id，算法uuid，重新起启会变
    #[prost(string, tag = "1")]
    pub guid: ::prost::alloc::string::String,
    /// 连接id，算法uuid，重连会变
    #[prost(string, tag = "2")]
    pub conn_id: ::prost::alloc::string::String,
    /// 最后收到的消息id，用于过虑重连后获取未读的消息
    #[prost(int64, tag = "3")]
    pub last_msg_id: i64,
}
/// 鉴权返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthResp {}
/// target_path:
///    "/" Service-Name "/" {method name} 参考 gRPC Request Path
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BroadcastFrame {
    /// 请求消息信息
    #[prost(message, optional, tag = "1")]
    pub options: ::core::option::Option<FrameOption>,
    /// 业务target_path
    #[prost(string, tag = "2")]
    pub target_path: ::prost::alloc::string::String,
    /// 业务pb内容
    #[prost(message, optional, tag = "3")]
    pub body: ::core::option::Option<super::super::super::google::protobuf::Any>,
}
/// message_id:
///    client: 本次连接唯一的消息id，可用于回执
///    server: 唯一消息id，可用于上报或者回执
/// sequence:
///    client: 客户端应该每次请求时frame seq++，会返回对应的对称req/resp
///    server: 服务端下行消息，只会返回默认值：0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrameOption {
    /// 消息id
    #[prost(int64, tag = "1")]
    pub message_id: i64,
    /// frame序号
    #[prost(int64, tag = "2")]
    pub sequence: i64,
    /// 是否进行消息回执(发出MessageAckReq)
    /// downstream 上只有服务端设置为true，客户端响应
    /// upstream   上只有客户端设置为true，服务端响应
    /// 响应帧禁止设置is_ack，协议上禁止循环
    /// 通常只有业务帧才可能设置is_ack, 因为协议栈(例如心跳、鉴权)另有响应约定
    #[prost(bool, tag = "3")]
    pub is_ack: bool,
    /// 业务状态码
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
    /// 业务ack来源, 仅downstream时候由服务端填写.
    #[prost(string, tag = "5")]
    pub ack_origin: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub timestamp: i64,
}
/// 心跳请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatReq {}
/// 心跳返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatResp {}
/// 消息回执
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAckReq {
    /// 消息id
    #[prost(int64, tag = "1")]
    pub ack_id: i64,
    /// ack来源，由业务指定用于埋点跟踪
    #[prost(string, tag = "2")]
    pub ack_origin: ::prost::alloc::string::String,
    /// 消息对应的target_path，方便业务区分和监控统计
    #[prost(string, tag = "3")]
    pub target_path: ::prost::alloc::string::String,
}
/// target_path
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetPath {
    /// 需要订阅的target_paths
    #[prost(string, repeated, tag = "1")]
    pub target_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    ///
    Unknown = 0,
    ///
    Update = 1,
    ///
    Delete = 2,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unknown => "UNKNOWN",
            Action::Update => "UPDATE",
            Action::Delete => "DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "UPDATE" => Some(Self::Update),
            "DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod broadcast_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// broadcast操作，对应每个target_path
    #[derive(Debug, Clone)]
    pub struct BroadcastClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BroadcastClient<tonic::transport::Channel> {
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
    impl<T> BroadcastClient<T>
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
        ) -> BroadcastClient<InterceptedService<T, F>>
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
            BroadcastClient::new(InterceptedService::new(inner, interceptor))
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
        /// 用户鉴权
        pub async fn auth(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthReq>,
        ) -> std::result::Result<tonic::Response<super::AuthResp>, tonic::Status> {
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
                "/bilibili.broadcast.v1.Broadcast/Auth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.Broadcast", "Auth"));
            self.inner.unary(req, path, codec).await
        }
        /// 心跳保活：成功心跳为4分45秒，重试心跳为30s，三次收不到进行重连（不超过5分45）
        pub async fn heartbeat(
            &mut self,
            request: impl tonic::IntoRequest<super::HeartbeatReq>,
        ) -> std::result::Result<tonic::Response<super::HeartbeatResp>, tonic::Status> {
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
                "/bilibili.broadcast.v1.Broadcast/Heartbeat",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.Broadcast", "Heartbeat"));
            self.inner.unary(req, path, codec).await
        }
        /// 订阅target_path
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::TargetPath>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.broadcast.v1.Broadcast/Subscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.Broadcast", "Subscribe"));
            self.inner.unary(req, path, codec).await
        }
        /// 取消订阅target_path
        pub async fn unsubscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::TargetPath>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.broadcast.v1.Broadcast/Unsubscribe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.broadcast.v1.Broadcast", "Unsubscribe"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 消息回执
        pub async fn message_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::MessageAckReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.broadcast.v1.Broadcast/MessageAck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.broadcast.v1.Broadcast", "MessageAck"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod broadcast_tunnel_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// broadcast连接隧道
    #[derive(Debug, Clone)]
    pub struct BroadcastTunnelClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BroadcastTunnelClient<tonic::transport::Channel> {
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
    impl<T> BroadcastTunnelClient<T>
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
        ) -> BroadcastTunnelClient<InterceptedService<T, F>>
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
            BroadcastTunnelClient::new(InterceptedService::new(inner, interceptor))
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
        /// 创建双向stream连接隧道
        pub async fn create_tunnel(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::BroadcastFrame>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BroadcastFrame>>,
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
                "/bilibili.broadcast.v1.BroadcastTunnel/CreateTunnel",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.broadcast.v1.BroadcastTunnel",
                        "CreateTunnel",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModResourceResp {
    ///
    #[prost(int32, tag = "1")]
    pub atcion: i32,
    ///
    #[prost(string, tag = "2")]
    pub app_key: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub pool_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub module_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub module_version: i64,
    ///
    #[prost(int64, tag = "6")]
    pub list_version: i64,
}
/// Generated client implementations.
pub mod mod_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ModManager
    #[derive(Debug, Clone)]
    pub struct ModManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModManagerClient<tonic::transport::Channel> {
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
    impl<T> ModManagerClient<T>
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
        ) -> ModManagerClient<InterceptedService<T, F>>
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
            ModManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn watch_resource(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ModResourceResp>>,
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
                "/bilibili.broadcast.v1.ModManager/WatchResource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.broadcast.v1.ModManager", "WatchResource"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddParams {
    ///
    #[prost(int32, tag = "1")]
    pub a: i32,
    ///
    #[prost(int32, tag = "2")]
    pub b: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddResult {
    ///
    #[prost(int32, tag = "1")]
    pub r: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestResp {
    /// 任务id
    #[prost(int64, tag = "1")]
    pub taskid: i64,
    /// 时间戳
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
    /// 消息
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// 扩展
    #[prost(message, optional, tag = "4")]
    pub extra: ::core::option::Option<super::super::super::google::protobuf::Any>,
}
/// Generated client implementations.
pub mod test_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Test
    #[derive(Debug, Clone)]
    pub struct TestClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TestClient<tonic::transport::Channel> {
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
    impl<T> TestClient<T>
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
        ) -> TestClient<InterceptedService<T, F>>
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
            TestClient::new(InterceptedService::new(inner, interceptor))
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
        /// 监听上报事件
        pub async fn watch_test_event(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TestResp>>,
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
                "/bilibili.broadcast.v1.Test/WatchTestEvent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.Test", "WatchTestEvent"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod test2_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct Test2Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl Test2Client<tonic::transport::Channel> {
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
    impl<T> Test2Client<T>
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
        ) -> Test2Client<InterceptedService<T, F>>
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
            Test2Client::new(InterceptedService::new(inner, interceptor))
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
        pub async fn test(
            &mut self,
            request: impl tonic::IntoRequest<super::AddParams>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.broadcast.v1.Test2/Test",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.broadcast.v1.Test2", "Test"));
            self.inner.unary(req, path, codec).await
        }
    }
}
