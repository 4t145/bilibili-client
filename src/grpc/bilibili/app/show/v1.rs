/// 气泡信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bubble {
    /// 文案
    #[prost(string, tag = "1")]
    pub bubble_content: ::prost::alloc::string::String,
    /// 版本
    #[prost(int32, tag = "2")]
    pub version: i32,
    /// 起始时间
    #[prost(int64, tag = "3")]
    pub stime: i64,
}
/// 配置信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// 标题
    #[prost(string, tag = "1")]
    pub item_title: ::prost::alloc::string::String,
    /// 底部文案
    #[prost(string, tag = "2")]
    pub bottom_text: ::prost::alloc::string::String,
    /// 底部图片url
    #[prost(string, tag = "3")]
    pub bottom_text_cover: ::prost::alloc::string::String,
    /// 底部跳转页url
    #[prost(string, tag = "4")]
    pub bottom_text_url: ::prost::alloc::string::String,
    /// 顶部按钮信息列表
    #[prost(message, repeated, tag = "5")]
    pub top_items: ::prost::alloc::vec::Vec<EntranceShow>,
    /// 头图url
    #[prost(string, tag = "6")]
    pub head_image: ::prost::alloc::string::String,
    /// 当前页按钮信息
    #[prost(message, repeated, tag = "7")]
    pub page_items: ::prost::alloc::vec::Vec<EntranceShow>,
    ///
    #[prost(int32, tag = "8")]
    pub hit: i32,
}
/// 按钮信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntranceShow {
    /// 按钮图标url
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 按钮名
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 入口模块id
    #[prost(string, tag = "3")]
    pub module_id: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    /// 气泡信息
    #[prost(message, optional, tag = "5")]
    pub bubble: ::core::option::Option<Bubble>,
    /// 入口id
    #[prost(int64, tag = "6")]
    pub entrance_id: i64,
    /// 头图url
    #[prost(string, tag = "7")]
    pub top_photo: ::prost::alloc::string::String,
    /// 入口类型
    #[prost(int32, tag = "8")]
    pub entrance_type: i32,
}
/// 热门列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopularReply {
    /// 卡片列表
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<super::super::card::v1::Card>,
    /// 配置信息
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<Config>,
    /// 版本
    #[prost(string, tag = "3")]
    pub ver: ::prost::alloc::string::String,
}
/// 热门列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopularResultReq {
    /// 排位索引id，为上此请求末尾项的idx
    #[prost(int64, tag = "1")]
    pub idx: i64,
    /// 登录标识
    /// 1:未登陆用户第一页 2:登陆用户第一页
    #[prost(int32, tag = "2")]
    pub login_event: i32,
    /// 清晰度(旧版)
    #[prost(int32, tag = "3")]
    pub qn: i32,
    /// 视频流版本(旧版)
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流功能(旧版)
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 是否强制使用域名(旧版)
    #[prost(int32, tag = "6")]
    pub force_host: i32,
    /// 是否4K(旧版)
    #[prost(int32, tag = "7")]
    pub fourk: i32,
    /// 当前页面spm
    #[prost(string, tag = "8")]
    pub spmid: ::prost::alloc::string::String,
    /// 上此请求末尾项的param
    #[prost(string, tag = "9")]
    pub last_param: ::prost::alloc::string::String,
    /// 上此请求的ver
    #[prost(string, tag = "10")]
    pub ver: ::prost::alloc::string::String,
    /// 分品类热门的入口ID
    #[prost(int64, tag = "11")]
    pub entrance_id: i64,
    /// 热门定位id集合
    #[prost(string, tag = "12")]
    pub location_ids: ::prost::alloc::string::String,
    /// 0:tag页 1:中间页
    #[prost(int32, tag = "13")]
    pub source_id: i32,
    /// 数据埋点上报
    /// 0:代表手动刷新 1:代表自动刷新
    #[prost(int32, tag = "14")]
    pub flush: i32,
    /// 秒开参数
    #[prost(message, optional, tag = "15")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
/// Generated client implementations.
pub mod popular_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 热门
    #[derive(Debug, Clone)]
    pub struct PopularClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PopularClient<tonic::transport::Channel> {
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
    impl<T> PopularClient<T>
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
        ) -> PopularClient<InterceptedService<T, F>>
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
            PopularClient::new(InterceptedService::new(inner, interceptor))
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
        /// 热门列表
        pub async fn index(
            &mut self,
            request: impl tonic::IntoRequest<super::PopularResultReq>,
        ) -> std::result::Result<tonic::Response<super::PopularReply>, tonic::Status> {
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
                "/bilibili.app.show.v1.Popular/Index",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.show.v1.Popular", "Index"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// 排行榜列表项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 封面url
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    /// 参数(稿件avid)
    #[prost(string, tag = "3")]
    pub param: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
    /// 重定向url
    #[prost(string, tag = "5")]
    pub redirect_url: ::prost::alloc::string::String,
    /// 跳转类型
    /// av:视频稿件
    #[prost(string, tag = "6")]
    pub goto: ::prost::alloc::string::String,
    /// 播放数
    #[prost(int32, tag = "7")]
    pub play: i32,
    /// 弹幕数
    #[prost(int32, tag = "8")]
    pub danmaku: i32,
    /// UP主mid
    #[prost(int64, tag = "9")]
    pub mid: i64,
    /// UP主昵称
    #[prost(string, tag = "10")]
    pub name: ::prost::alloc::string::String,
    /// UP主头像url
    #[prost(string, tag = "11")]
    pub face: ::prost::alloc::string::String,
    /// 评论数
    #[prost(int32, tag = "12")]
    pub reply: i32,
    /// 收藏数
    #[prost(int32, tag = "13")]
    pub favourite: i32,
    /// 发布时间
    #[prost(int64, tag = "14")]
    pub pub_date: i64,
    /// 分区tid
    #[prost(int32, tag = "15")]
    pub rid: i32,
    /// 子分区名
    #[prost(string, tag = "16")]
    pub rname: ::prost::alloc::string::String,
    /// 视频总时长
    #[prost(int64, tag = "17")]
    pub duration: i64,
    /// 点赞数
    #[prost(int32, tag = "18")]
    pub like: i32,
    /// 1P cid
    #[prost(int64, tag = "19")]
    pub cid: i64,
    /// 综合评分
    #[prost(int64, tag = "20")]
    pub pts: i64,
    /// 合作视频文案
    #[prost(string, tag = "21")]
    pub cooperation: ::prost::alloc::string::String,
    /// 属性位
    /// 0:未关注 1:已关注
    #[prost(int32, tag = "22")]
    pub attribute: i32,
    /// UP主粉丝数
    #[prost(int64, tag = "23")]
    pub follower: i64,
    /// UP主认证信息
    #[prost(message, optional, tag = "24")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    /// 同一UP收起子项列表
    #[prost(message, repeated, tag = "25")]
    pub children: ::prost::alloc::vec::Vec<Item>,
    /// 关系信息
    #[prost(message, optional, tag = "26")]
    pub relation: ::core::option::Option<Relation>,
}
/// 认证信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    /// 认证类型
    /// -1:无认证 0:个人认证 1:机构认证
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 认证描述
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
/// 全站排行榜-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankAllResultReq {
    /// 必须为"all"
    #[prost(string, tag = "1")]
    pub order: ::prost::alloc::string::String,
    /// 页码
    /// 默认1页
    #[prost(int32, tag = "2")]
    pub pn: i32,
    /// 每页项数
    /// 默认100项，最大100
    #[prost(int32, tag = "3")]
    pub ps: i32,
}
/// 排行榜信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankListReply {
    /// 排行榜列表
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
/// 分区排行榜-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankRegionResultReq {
    /// 一级分区tid(二级分区不可用)
    /// 0:全站
    #[prost(int32, tag = "1")]
    pub rid: i32,
    /// 页码
    /// 默认1页
    #[prost(int32, tag = "2")]
    pub pn: i32,
    /// 每页项数
    /// 默认100项，最大100
    #[prost(int32, tag = "3")]
    pub ps: i32,
}
/// 关系信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    /// 关系状态id
    /// 1:未关注 2:已关注 3:被关注 4:互相关注
    #[prost(int32, tag = "1")]
    pub status: i32,
    /// 是否关注
    #[prost(int32, tag = "2")]
    pub is_follow: i32,
    /// 是否粉丝
    #[prost(int32, tag = "3")]
    pub is_followed: i32,
}
/// Generated client implementations.
pub mod rank_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 排行榜
    #[derive(Debug, Clone)]
    pub struct RankClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RankClient<tonic::transport::Channel> {
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
    impl<T> RankClient<T>
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
        ) -> RankClient<InterceptedService<T, F>>
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
            RankClient::new(InterceptedService::new(inner, interceptor))
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
        /// 全站排行榜
        pub async fn rank_all(
            &mut self,
            request: impl tonic::IntoRequest<super::RankAllResultReq>,
        ) -> std::result::Result<tonic::Response<super::RankListReply>, tonic::Status> {
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
                "/bilibili.app.show.v1.Rank/RankAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.show.v1.Rank", "RankAll"));
            self.inner.unary(req, path, codec).await
        }
        /// 分区排行榜
        pub async fn rank_region(
            &mut self,
            request: impl tonic::IntoRequest<super::RankRegionResultReq>,
        ) -> std::result::Result<tonic::Response<super::RankListReply>, tonic::Status> {
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
                "/bilibili.app.show.v1.Rank/RankRegion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.show.v1.Rank", "RankRegion"));
            self.inner.unary(req, path, codec).await
        }
    }
}
