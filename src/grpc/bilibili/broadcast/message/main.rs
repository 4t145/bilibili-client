///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bubble {
    #[prost(message, repeated, tag = "1")]
    pub paragraphs: ::prost::alloc::vec::Vec<
        super::super::super::app::dynamic::v2::Paragraph,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatResult {
    ///
    #[prost(int32, tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub bubble: ::prost::alloc::vec::Vec<Bubble>,
    ///
    #[prost(string, tag = "4")]
    pub rewrite_word: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
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
        pub async fn chat_result_push(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ChatResult>>,
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
                "/bilibili.broadcast.message.main.Search/ChatResultPush",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.broadcast.message.main.Search",
                        "ChatResultPush",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NativePageEvent {
    /// Native页ID
    #[prost(int64, tag = "1")]
    pub page_id: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<EventItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventItem {
    /// 组件标识
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    /// 组件类型
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// 进度条数值
    #[prost(int64, tag = "3")]
    pub num: i64,
    /// 进度条展示数值
    #[prost(string, tag = "4")]
    pub display_num: ::prost::alloc::string::String,
    /// h5的组件标识
    #[prost(string, tag = "5")]
    pub web_key: ::prost::alloc::string::String,
    /// 活动统计维度
    /// 0:用户维度 1:规则维度
    #[prost(int64, tag = "6")]
    pub dimension: i64,
}
/// Generated client implementations.
pub mod native_page_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct NativePageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NativePageClient<tonic::transport::Channel> {
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
    impl<T> NativePageClient<T>
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
        ) -> NativePageClient<InterceptedService<T, F>>
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
            NativePageClient::new(InterceptedService::new(inner, interceptor))
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
            tonic::Response<tonic::codec::Streaming<super::NativePageEvent>>,
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
                "/bilibili.broadcast.message.main.NativePage/WatchNotify",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.broadcast.message.main.NativePage",
                        "WatchNotify",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopActivityReply {
    /// 当前生效的资源
    #[prost(message, optional, tag = "1")]
    pub online: ::core::option::Option<TopOnline>,
    /// 对online内容进行hash和上次结果一样则不重新加载
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
}
/// 当前生效的资源
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopOnline {
    /// 活动类型
    /// 1:七日活动 2:后台配置
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 图标
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    /// 跳转链接
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// 资源状态标识(后台配置)
    #[prost(string, tag = "4")]
    pub unique_id: ::prost::alloc::string::String,
    /// 动画资源
    #[prost(message, optional, tag = "5")]
    pub animate: ::core::option::Option<Animate>,
    /// 红点
    #[prost(message, optional, tag = "6")]
    pub red_dot: ::core::option::Option<RedDot>,
    /// 活动名称
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// 轮询间隔 单位秒
    #[prost(int64, tag = "8")]
    pub interval: i64,
}
/// 动画资源
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Animate {
    /// 动效结束展示icon
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 7日活动动画
    #[prost(string, tag = "2")]
    pub json: ::prost::alloc::string::String,
    /// s10活动svg动画
    #[prost(string, tag = "3")]
    pub svg: ::prost::alloc::string::String,
    /// 循环次数(默认0不返回 表示无限循环)
    #[prost(int32, tag = "4")]
    pub r#loop: i32,
}
/// 红点
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedDot {
    /// 红点类型
    /// 1:纯红点 2:数字红点
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 如果是数字红点 显示的数字
    #[prost(int32, tag = "2")]
    pub number: i32,
}
/// Generated client implementations.
pub mod resource_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct ResourceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ResourceClient<tonic::transport::Channel> {
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
    impl<T> ResourceClient<T>
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
        ) -> ResourceClient<InterceptedService<T, F>>
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
            ResourceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn top_activity(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TopActivityReply>>,
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
                "/bilibili.broadcast.message.main.Resource/TopActivity",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.broadcast.message.main.Resource",
                        "TopActivity",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// 实时弹幕事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmukuEvent {
    /// 弹幕列表
    #[prost(message, repeated, tag = "1")]
    pub elems: ::prost::alloc::vec::Vec<DanmakuElem>,
}
/// 弹幕条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DanmakuElem {
    /// 弹幕dmid
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 弹幕出现位置(单位为ms)
    #[prost(int32, tag = "2")]
    pub progress: i32,
    /// 弹幕类型
    #[prost(int32, tag = "3")]
    pub mode: i32,
    /// 弹幕字号
    #[prost(int32, tag = "4")]
    pub fontsize: i32,
    /// 弹幕颜色
    #[prost(uint32, tag = "5")]
    pub color: u32,
    /// 发送着mid hash
    #[prost(string, tag = "6")]
    pub mid_hash: ::prost::alloc::string::String,
    /// 弹幕正文
    #[prost(string, tag = "7")]
    pub content: ::prost::alloc::string::String,
    /// 发送时间
    #[prost(int64, tag = "8")]
    pub ctime: i64,
    /// 弹幕动作
    #[prost(string, tag = "9")]
    pub action: ::prost::alloc::string::String,
    /// 弹幕池
    #[prost(int32, tag = "10")]
    pub pool: i32,
    /// 弹幕id str
    #[prost(string, tag = "11")]
    pub id_str: ::prost::alloc::string::String,
}
/// 互动弹幕
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDm {
    /// 弹幕id
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// 对象视频cid
    #[prost(int64, tag = "2")]
    pub oid: i64,
    /// 发送者mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
    /// 互动弹幕指令
    #[prost(string, tag = "5")]
    pub command: ::prost::alloc::string::String,
    /// 互动弹幕正文
    #[prost(string, tag = "6")]
    pub content: ::prost::alloc::string::String,
    /// 弹幕状态
    #[prost(int32, tag = "7")]
    pub state: i32,
    /// 出现时间
    #[prost(int32, tag = "8")]
    pub progress: i32,
    /// 创建时间
    #[prost(string, tag = "9")]
    pub ctime: ::prost::alloc::string::String,
    /// 发布时间
    #[prost(string, tag = "10")]
    pub mtime: ::prost::alloc::string::String,
    /// 扩展json数据
    #[prost(string, tag = "11")]
    pub extra: ::prost::alloc::string::String,
    /// 弹幕id str类型
    #[prost(string, tag = "12")]
    pub id_str: ::prost::alloc::string::String,
}
