/// 专栏卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardArticle {
    /// 封面url
    #[prost(string, repeated, tag = "1")]
    pub covers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// UP主昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// UP主mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    /// 是否展示关注按钮
    #[prost(bool, tag = "4")]
    pub display_attention: bool,
    /// 角标
    #[prost(string, tag = "5")]
    pub badge: ::prost::alloc::string::String,
    /// 关系信息
    #[prost(message, optional, tag = "6")]
    pub relation: ::core::option::Option<Relation>,
}
/// 课程卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardCheese {
    /// 封面url
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    /// 观看进度
    #[prost(int64, tag = "2")]
    pub progress: i64,
    /// 总计时长
    #[prost(int64, tag = "3")]
    pub duration: i64,
    /// 单集标题
    #[prost(string, tag = "4")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub state: i64,
}
/// 直播卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardLive {
    /// 封面url
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    /// 主播昵称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 主播mid
    #[prost(int64, tag = "3")]
    pub mid: i64,
    /// 直播分区名
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
    /// 直播状态
    #[prost(int32, tag = "5")]
    pub ststus: i32,
    /// 是否展示关注按钮
    #[prost(bool, tag = "6")]
    pub display_attention: bool,
    /// 关系信息
    #[prost(message, optional, tag = "7")]
    pub relation: ::core::option::Option<Relation>,
}
/// pgc稿件卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardOgv {
    /// 封面url
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    /// 观看进度
    #[prost(int64, tag = "2")]
    pub progress: i64,
    /// 总计时长
    #[prost(int64, tag = "3")]
    pub duration: i64,
    /// 单集标题
    #[prost(string, tag = "4")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub state: i64,
}
/// ugc稿件卡片
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardUgc {
    /// 封面url
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    /// 观看进度
    #[prost(int64, tag = "2")]
    pub progress: i64,
    /// 视频长度
    #[prost(int64, tag = "3")]
    pub duration: i64,
    /// UP主昵称
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// UP主mid
    #[prost(int64, tag = "5")]
    pub mid: i64,
    /// 是否展示关注按钮
    #[prost(bool, tag = "6")]
    pub display_attention: bool,
    /// 历史观看视频cid
    #[prost(int64, tag = "7")]
    pub cid: i64,
    /// 历史观看视频分P
    #[prost(int32, tag = "8")]
    pub page: i32,
    /// 历史观看视频分P的标题
    #[prost(string, tag = "9")]
    pub subtitle: ::prost::alloc::string::String,
    /// 关系信息
    #[prost(message, optional, tag = "10")]
    pub relation: ::core::option::Option<Relation>,
    /// 稿件bvid
    #[prost(string, tag = "11")]
    pub bvid: ::prost::alloc::string::String,
    /// 总分P数
    #[prost(int64, tag = "12")]
    pub videos: i64,
    /// 短链接
    #[prost(string, tag = "13")]
    pub short_link: ::prost::alloc::string::String,
    /// 分享副标题
    #[prost(string, tag = "14")]
    pub share_subtitle: ::prost::alloc::string::String,
    /// 播放数
    #[prost(int64, tag = "15")]
    pub view: i64,
    ///
    #[prost(int64, tag = "16")]
    pub state: i64,
}
/// 清空历史记录-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearReq {
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "1")]
    pub business: ::prost::alloc::string::String,
}
/// 游标信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cursor {
    /// 本页最大值游标值
    #[prost(int64, tag = "1")]
    pub max: i64,
    /// 本页最大值游标类型
    #[prost(int32, tag = "2")]
    pub max_tp: i32,
}
/// 历史记录卡片信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorItem {
    /// 标题
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// 目标uri/url
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    /// 观看时间
    #[prost(int64, tag = "8")]
    pub view_at: i64,
    /// 历史记录id
    #[prost(int64, tag = "9")]
    pub kid: i64,
    /// 业务id
    #[prost(int64, tag = "10")]
    pub oid: i64,
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "11")]
    pub business: ::prost::alloc::string::String,
    /// 业务类型代码
    #[prost(int32, tag = "12")]
    pub tp: i32,
    /// 设备标识
    #[prost(message, optional, tag = "13")]
    pub dt: ::core::option::Option<DeviceType>,
    /// 是否有分享按钮
    #[prost(bool, tag = "14")]
    pub has_share: bool,
    /// 主体数据
    #[prost(oneof = "cursor_item::CardItem", tags = "1, 2, 3, 4, 5")]
    pub card_item: ::core::option::Option<cursor_item::CardItem>,
}
/// Nested message and enum types in `CursorItem`.
pub mod cursor_item {
    /// 主体数据
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CardItem {
        /// ugc稿件
        #[prost(message, tag = "1")]
        CardUgc(super::CardUgc),
        /// pgc稿件
        #[prost(message, tag = "2")]
        CardOgv(super::CardOgv),
        /// 专栏
        #[prost(message, tag = "3")]
        CardArticle(super::CardArticle),
        /// 直播
        #[prost(message, tag = "4")]
        CardLive(super::CardLive),
        /// 课程
        #[prost(message, tag = "5")]
        CardCheese(super::CardCheese),
    }
}
/// 获取历史记录列表(旧版)-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorReply {
    /// 卡片内容
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CursorItem>,
    /// 顶部tab
    #[prost(message, repeated, tag = "2")]
    pub tab: ::prost::alloc::vec::Vec<CursorTab>,
    /// 游标信息
    #[prost(message, optional, tag = "3")]
    pub cursor: ::core::option::Option<Cursor>,
    /// 是否未拉取完
    #[prost(bool, tag = "4")]
    pub has_more: bool,
}
/// 获取历史记录列表(旧版)-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorReq {
    /// 游标信息
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<Cursor>,
    /// 业务类型
    /// all:全部 archive:视频 live:直播 article:专栏
    #[prost(string, tag = "2")]
    pub business: ::prost::alloc::string::String,
    /// 秒开参数(旧版)
    #[prost(message, optional, tag = "3")]
    pub player_preload: ::core::option::Option<PlayerPreloadParams>,
    /// 秒开参数
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
/// 业务分类表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorTab {
    /// 业务类型
    #[prost(string, tag = "1")]
    pub business: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 路由uri
    #[prost(string, tag = "3")]
    pub router: ::prost::alloc::string::String,
    /// tab定位
    #[prost(bool, tag = "4")]
    pub focus: bool,
}
/// 获取历史记录列表-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorV2Reply {
    /// 卡片内容
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CursorItem>,
    /// 游标信息
    #[prost(message, optional, tag = "2")]
    pub cursor: ::core::option::Option<Cursor>,
    /// 是否未拉取完
    #[prost(bool, tag = "3")]
    pub has_more: bool,
    ///
    #[prost(string, tag = "4")]
    pub empty_link: ::prost::alloc::string::String,
}
/// 获取历史记录列表-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CursorV2Req {
    /// 游标信息
    #[prost(message, optional, tag = "1")]
    pub cursor: ::core::option::Option<Cursor>,
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "2")]
    pub business: ::prost::alloc::string::String,
    /// 秒开参数(旧版)
    #[prost(message, optional, tag = "3")]
    pub player_preload: ::core::option::Option<PlayerPreloadParams>,
    /// 秒开参数
    #[prost(message, optional, tag = "4")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
    /// 是否选择本机的播放历史
    #[prost(bool, tag = "5")]
    pub is_local: bool,
}
/// 删除历史记录-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReq {
    /// 历史记录信息
    #[prost(message, optional, tag = "1")]
    pub his_info: ::core::option::Option<HisInfo>,
}
/// 设备类型
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceType {
    /// 设备标识代码
    #[prost(enumeration = "Dt", tag = "1")]
    pub r#type: i32,
    /// 图标url
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
}
/// 历史记录信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HisInfo {
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "1")]
    pub business: ::prost::alloc::string::String,
    /// 历史记录id
    #[prost(int64, tag = "2")]
    pub kid: i64,
}
/// 获取历史记录tab-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoryTabReply {
    /// tab列表
    #[prost(message, repeated, tag = "1")]
    pub tab: ::prost::alloc::vec::Vec<CursorTab>,
}
/// 获取历史记录tab-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoryTabReq {
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "1")]
    pub business: ::prost::alloc::string::String,
    /// 查询请求来源
    #[prost(enumeration = "HistorySource", tag = "2")]
    pub source: i32,
    /// 搜索关键词
    #[prost(string, tag = "3")]
    pub keyword: ::prost::alloc::string::String,
}
/// 获取最新的历史记录-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatestHistoryReply {
    /// 卡片内容
    #[prost(message, optional, tag = "1")]
    pub items: ::core::option::Option<CursorItem>,
    /// 场景
    #[prost(string, tag = "2")]
    pub scene: ::prost::alloc::string::String,
    /// 弹窗停留时间
    #[prost(int64, tag = "3")]
    pub rtime: i64,
    /// 分组的标志(客户端埋点上报)
    #[prost(string, tag = "4")]
    pub flag: ::prost::alloc::string::String,
}
/// 获取最新的历史记录-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatestHistoryReq {
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "1")]
    pub business: ::prost::alloc::string::String,
    /// 秒开参数
    #[prost(message, optional, tag = "2")]
    pub player_preload: ::core::option::Option<PlayerPreloadParams>,
}
/// 空响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoReply {}
/// 页面信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// 当前页码
    #[prost(int64, tag = "1")]
    pub pn: i64,
    /// 总计条目数
    #[prost(int64, tag = "2")]
    pub total: i64,
}
/// 秒开参数
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreloadParams {
    /// 清晰度
    #[prost(int64, tag = "1")]
    pub qn: i64,
    /// 流版本
    #[prost(int64, tag = "2")]
    pub fnver: i64,
    /// 流类型
    #[prost(int64, tag = "3")]
    pub fnval: i64,
    /// 是否强制域名
    #[prost(int64, tag = "4")]
    pub force_host: i64,
    /// 是否4K
    #[prost(int64, tag = "5")]
    pub fourk: i64,
}
/// 关系信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    /// 关系状态
    /// 1:未关注 2:已关注 3:被关注 4:互关
    #[prost(int32, tag = "1")]
    pub status: i32,
    /// 用户关注UP主
    #[prost(int32, tag = "2")]
    pub is_follow: i32,
    /// UP主关注用户
    #[prost(int32, tag = "3")]
    pub is_followed: i32,
}
/// 搜索历史记录-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReply {
    /// 卡片内容
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<CursorItem>,
    /// 是否未拉取完
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    /// 页面信息
    #[prost(message, optional, tag = "3")]
    pub page: ::core::option::Option<Page>,
}
/// 搜索历史记录-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchReq {
    /// 关键词
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    /// 页码
    #[prost(int64, tag = "2")]
    pub pn: i64,
    /// 业务类型
    /// archive:视频 live:直播 article:专栏 goods:商品 show:展演
    #[prost(string, tag = "3")]
    pub business: ::prost::alloc::string::String,
}
/// 设备标识代码
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Dt {
    /// 未知
    Unknown = 0,
    /// 手机端
    Phone = 1,
    /// ipad端
    Pad = 2,
    /// web端
    Pc = 3,
    /// TV端
    Tv = 4,
    /// 车机端
    Car = 5,
    /// 物联设备
    Iot = 6,
    /// apad端
    AndPad = 7,
}
impl Dt {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Dt::Unknown => "Unknown",
            Dt::Phone => "Phone",
            Dt::Pad => "Pad",
            Dt::Pc => "PC",
            Dt::Tv => "TV",
            Dt::Car => "Car",
            Dt::Iot => "Iot",
            Dt::AndPad => "AndPad",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Phone" => Some(Self::Phone),
            "Pad" => Some(Self::Pad),
            "PC" => Some(Self::Pc),
            "TV" => Some(Self::Tv),
            "Car" => Some(Self::Car),
            "Iot" => Some(Self::Iot),
            "AndPad" => Some(Self::AndPad),
            _ => None,
        }
    }
}
/// 搜索历史记录来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HistorySource {
    /// 主站历史记录页
    HistoryValue = 0,
    /// 会员购浏览记录
    ShoppingValue = 1,
}
impl HistorySource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HistorySource::HistoryValue => "history_VALUE",
            HistorySource::ShoppingValue => "shopping_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "history_VALUE" => Some(Self::HistoryValue),
            "shopping_VALUE" => Some(Self::ShoppingValue),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod history_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 历史记录
    #[derive(Debug, Clone)]
    pub struct HistoryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HistoryClient<tonic::transport::Channel> {
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
    impl<T> HistoryClient<T>
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
        ) -> HistoryClient<InterceptedService<T, F>>
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
            HistoryClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取历史记录tab
        pub async fn history_tab(
            &mut self,
            request: impl tonic::IntoRequest<super::HistoryTabReq>,
        ) -> std::result::Result<
            tonic::Response<super::HistoryTabReply>,
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
                "/bilibili.app.interface.v1.History/HistoryTab",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.History", "HistoryTab"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 获取历史记录列表(旧版)
        pub async fn cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::CursorReq>,
        ) -> std::result::Result<tonic::Response<super::CursorReply>, tonic::Status> {
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
                "/bilibili.app.interface.v1.History/Cursor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.History", "Cursor"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取历史记录列表
        pub async fn cursor_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::CursorV2Req>,
        ) -> std::result::Result<tonic::Response<super::CursorV2Reply>, tonic::Status> {
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
                "/bilibili.app.interface.v1.History/CursorV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.History", "CursorV2"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 删除历史记录
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReq>,
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
                "/bilibili.app.interface.v1.History/Delete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.History", "Delete"));
            self.inner.unary(req, path, codec).await
        }
        /// 搜索历史记录
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchReq>,
        ) -> std::result::Result<tonic::Response<super::SearchReply>, tonic::Status> {
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
                "/bilibili.app.interface.v1.History/Search",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.History", "Search"));
            self.inner.unary(req, path, codec).await
        }
        /// 清空历史记录
        pub async fn clear(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearReq>,
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
                "/bilibili.app.interface.v1.History/Clear",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.History", "Clear"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取最新的历史记录
        pub async fn latest_history(
            &mut self,
            request: impl tonic::IntoRequest<super::LatestHistoryReq>,
        ) -> std::result::Result<
            tonic::Response<super::LatestHistoryReply>,
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
                "/bilibili.app.interface.v1.History/LatestHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.History", "LatestHistory"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arc {
    ///
    #[prost(message, optional, tag = "1")]
    pub archive: ::core::option::Option<super::super::archive::v1::Arc>,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dynamic {
    ///
    #[prost(message, optional, tag = "1")]
    pub dynamic: ::core::option::Option<super::super::dynamic::v2::DynamicItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTabReply {
    ///
    #[prost(int64, tag = "1")]
    pub focus: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub tabs: ::prost::alloc::vec::Vec<Tab>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTabReq {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "3")]
    pub from: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchArchiveReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub archives: ::prost::alloc::vec::Vec<Arc>,
    ///
    #[prost(int64, tag = "2")]
    pub total: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchArchiveReq {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub pn: i64,
    ///
    #[prost(int64, tag = "4")]
    pub ps: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDynamicReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub dynamics: ::prost::alloc::vec::Vec<Dynamic>,
    ///
    #[prost(int64, tag = "2")]
    pub total: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDynamicReq {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub pn: i64,
    ///
    #[prost(int64, tag = "4")]
    pub ps: i64,
    ///
    #[prost(message, optional, tag = "5")]
    pub player_args: ::core::option::Option<
        super::super::archive::middleware::v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tab {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum From {
    ///
    ArchiveTab = 0,
    ///
    DynamicTab = 1,
}
impl From {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            From::ArchiveTab => "ArchiveTab",
            From::DynamicTab => "DynamicTab",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ArchiveTab" => Some(Self::ArchiveTab),
            "DynamicTab" => Some(Self::DynamicTab),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod space_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct SpaceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SpaceClient<tonic::transport::Channel> {
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
    impl<T> SpaceClient<T>
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
        ) -> SpaceClient<InterceptedService<T, F>>
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
            SpaceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn search_tab(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTabReq>,
        ) -> std::result::Result<tonic::Response<super::SearchTabReply>, tonic::Status> {
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
                "/bilibili.app.interface.v1.Space/SearchTab",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.Space", "SearchTab"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn search_archive(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchArchiveReq>,
        ) -> std::result::Result<
            tonic::Response<super::SearchArchiveReply>,
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
                "/bilibili.app.interface.v1.Space/SearchArchive",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Space", "SearchArchive"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn search_dynamic(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchDynamicReq>,
        ) -> std::result::Result<
            tonic::Response<super::SearchDynamicReply>,
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
                "/bilibili.app.interface.v1.Space/SearchDynamic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Space", "SearchDynamic"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover_image_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub cover_left_icon1: i64,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub cover_left_icon2: i64,
    ///
    #[prost(message, optional, tag = "9")]
    pub user_card: ::core::option::Option<UserCard>,
    ///
    #[prost(message, optional, tag = "10")]
    pub like_button: ::core::option::Option<LikeButton>,
    ///
    #[prost(int64, tag = "11")]
    pub param: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub icon: i64,
    ///
    #[prost(enumeration = "ButType", tag = "5")]
    pub but_type: i32,
    ///
    #[prost(int32, tag = "6")]
    pub follow_state: i32,
    ///
    #[prost(string, tag = "7")]
    pub has_title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cast {
    ///
    #[prost(message, repeated, tag = "1")]
    pub person: ::prost::alloc::vec::Vec<MediaPerson>,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelInfo {
    ///
    #[prost(int64, tag = "1")]
    pub channel_id: i64,
    ///
    #[prost(bool, tag = "2")]
    pub subscribed: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeButton {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub count: i32,
    ///
    #[prost(bool, tag = "3")]
    pub show_count: bool,
    ///
    #[prost(string, tag = "4")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub selected: i32,
    ///
    #[prost(string, tag = "6")]
    pub event_v2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub like_resource: ::core::option::Option<LikeButtonResource>,
    ///
    #[prost(message, optional, tag = "8")]
    pub dis_like_resource: ::core::option::Option<LikeButtonResource>,
    ///
    #[prost(message, optional, tag = "9")]
    pub like_night_resource: ::core::option::Option<LikeButtonResource>,
    ///
    #[prost(message, optional, tag = "10")]
    pub dis_like_night_resource: ::core::option::Option<LikeButtonResource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeButtonResource {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeCard {
    ///
    #[prost(int64, tag = "1")]
    pub like: i64,
    ///
    #[prost(bool, tag = "2")]
    pub is_follow: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaCard {
    ///
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cur_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub style: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub label: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub but_first: ::core::option::Option<Button>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaDetailReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub cast: ::core::option::Option<Cast>,
    ///
    #[prost(message, optional, tag = "2")]
    pub staff: ::core::option::Option<Staff>,
    ///
    #[prost(message, optional, tag = "3")]
    pub overview: ::core::option::Option<Overview>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaDetailReq {
    ///
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub biz_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFollowReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFollowReq {
    ///
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaPerson {
    ///
    #[prost(string, tag = "1")]
    pub real_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub square_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub character: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub person_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaRelationReply {
    ///
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<SmallItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaRelationReq {
    ///
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub biz_type: i64,
    ///
    #[prost(int64, tag = "3")]
    pub feed_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub ps: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaTabReply {
    ///
    #[prost(message, optional, tag = "1")]
    pub media_card: ::core::option::Option<MediaCard>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub tab: ::prost::alloc::vec::Vec<ShowTab>,
    ///
    #[prost(int64, tag = "3")]
    pub default_tab_index: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub channel_info: ::core::option::Option<ChannelInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaTabReq {
    ///
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub biz_type: i64,
    ///
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "5")]
    pub args: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaVideoReply {
    ///
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "2")]
    pub has_more: bool,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<BigItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaVideoReq {
    ///
    #[prost(int64, tag = "1")]
    pub biz_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub biz_type: i64,
    ///
    #[prost(int64, tag = "3")]
    pub feed_id: i64,
    ///
    #[prost(string, tag = "5")]
    pub offset: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub ps: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Overview {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowTab {
    ///
    #[prost(enumeration = "TabType", tag = "1")]
    pub tab_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmallItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover_image_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub cover_left_icon1: i64,
    ///
    #[prost(string, tag = "7")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub cover_left_icon2: i64,
    ///
    #[prost(int64, tag = "9")]
    pub param: i64,
    ///
    #[prost(int64, tag = "10")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Staff {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCard {
    ///
    #[prost(string, tag = "1")]
    pub user_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub user_face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub user_url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub mid: i64,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ButType {
    ///
    ButInvalid = 0,
    ///
    ButRedirect = 1,
    ///
    ButLike = 2,
}
impl ButType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ButType::ButInvalid => "BUT_INVALID",
            ButType::ButRedirect => "BUT_REDIRECT",
            ButType::ButLike => "BUT_LIKE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUT_INVALID" => Some(Self::ButInvalid),
            "BUT_REDIRECT" => Some(Self::ButRedirect),
            "BUT_LIKE" => Some(Self::ButLike),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TabType {
    ///
    TabInvalid = 0,
    ///
    TabOgvDetail = 6,
    ///
    TabOgvReply = 7,
    ///
    TabFeedBid = 8,
    ///
    TabFeedSmall = 9,
}
impl TabType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TabType::TabInvalid => "TAB_INVALID",
            TabType::TabOgvDetail => "TAB_OGV_DETAIL",
            TabType::TabOgvReply => "TAB_OGV_REPLY",
            TabType::TabFeedBid => "TAB_FEED_BID",
            TabType::TabFeedSmall => "TAB_FEED_SMALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TAB_INVALID" => Some(Self::TabInvalid),
            "TAB_OGV_DETAIL" => Some(Self::TabOgvDetail),
            "TAB_OGV_REPLY" => Some(Self::TabOgvReply),
            "TAB_FEED_BID" => Some(Self::TabFeedBid),
            "TAB_FEED_SMALL" => Some(Self::TabFeedSmall),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod media_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct MediaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MediaClient<tonic::transport::Channel> {
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
    impl<T> MediaClient<T>
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
        ) -> MediaClient<InterceptedService<T, F>>
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
            MediaClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn media_tab(
            &mut self,
            request: impl tonic::IntoRequest<super::MediaTabReq>,
        ) -> std::result::Result<tonic::Response<super::MediaTabReply>, tonic::Status> {
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
                "/bilibili.app.interface.v1.Media/MediaTab",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.Media", "MediaTab"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn media_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::MediaDetailReq>,
        ) -> std::result::Result<
            tonic::Response<super::MediaDetailReply>,
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
                "/bilibili.app.interface.v1.Media/MediaDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Media", "MediaDetail"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn media_video(
            &mut self,
            request: impl tonic::IntoRequest<super::MediaVideoReq>,
        ) -> std::result::Result<
            tonic::Response<super::MediaVideoReply>,
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
                "/bilibili.app.interface.v1.Media/MediaVideo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Media", "MediaVideo"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn media_relation(
            &mut self,
            request: impl tonic::IntoRequest<super::MediaRelationReq>,
        ) -> std::result::Result<
            tonic::Response<super::MediaRelationReply>,
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
                "/bilibili.app.interface.v1.Media/MediaRelation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Media", "MediaRelation"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn media_follow(
            &mut self,
            request: impl tonic::IntoRequest<super::MediaFollowReq>,
        ) -> std::result::Result<
            tonic::Response<super::MediaFollowReply>,
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
                "/bilibili.app.interface.v1.Media/MediaFollow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Media", "MediaFollow"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultWordsReply {
    ///
    #[prost(string, tag = "1")]
    pub trackid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub show: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub word: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub show_front: i64,
    ///
    #[prost(string, tag = "6")]
    pub exp_str: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub value: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftFaceIcon {
    ///
    #[prost(int32, tag = "1")]
    pub region_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub show_status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultWordsReq {
    ///
    #[prost(int64, tag = "1")]
    pub from: i64,
    ///
    #[prost(int64, tag = "2")]
    pub login_event: i64,
    ///
    #[prost(int32, tag = "3")]
    pub teenagers_mode: i32,
    ///
    #[prost(int32, tag = "4")]
    pub lessons_mode: i32,
    ///
    #[prost(string, tag = "5")]
    pub tab: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub event_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub avid: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub query: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub an: i64,
    ///
    #[prost(int64, tag = "10")]
    pub is_fresh: i64,
}
/// 获取搜索建议-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestionResult3Reply {
    /// 搜索追踪id
    #[prost(string, tag = "1")]
    pub trackid: ::prost::alloc::string::String,
    /// 搜索建议条目列表
    #[prost(message, repeated, tag = "2")]
    pub list: ::prost::alloc::vec::Vec<ResultItem>,
    /// 搜索的abtest 实验信息
    #[prost(string, tag = "3")]
    pub exp_str: ::prost::alloc::string::String,
}
/// 获取搜索建议-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestionResult3Req {
    /// 关键字
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    /// 是否语法高亮
    /// 0:不显示 1:显示
    #[prost(int32, tag = "2")]
    pub highlight: i32,
    /// 是否青少年模式
    /// 1:开启青少年模式
    #[prost(int32, tag = "3")]
    pub teenagers_mode: i32,
}
/// 搜索建议条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultItem {
    /// 来源
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    /// 显示结果(语法高亮)
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// 搜索关键字
    #[prost(string, tag = "3")]
    pub keyword: ::prost::alloc::string::String,
    /// 序号
    #[prost(int32, tag = "4")]
    pub position: i32,
    /// 图片
    #[prost(string, tag = "5")]
    pub cover: ::prost::alloc::string::String,
    /// 图片尺寸
    #[prost(double, tag = "6")]
    pub cover_size: f64,
    /// sug词类型
    #[prost(string, tag = "7")]
    pub sug_type: ::prost::alloc::string::String,
    /// 词条大类型
    #[prost(int32, tag = "8")]
    pub term_type: i32,
    /// 跳转类型
    #[prost(string, tag = "9")]
    pub goto: ::prost::alloc::string::String,
    /// 跳转uri
    #[prost(string, tag = "10")]
    pub uri: ::prost::alloc::string::String,
    /// 认证信息
    #[prost(message, optional, tag = "11")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    /// 跳转参数
    #[prost(string, tag = "12")]
    pub param: ::prost::alloc::string::String,
    /// up主mid
    #[prost(int64, tag = "13")]
    pub mid: i64,
    /// 粉丝数
    #[prost(int32, tag = "14")]
    pub fans: i32,
    /// up主等级
    #[prost(int32, tag = "15")]
    pub level: i32,
    /// up主稿件数
    #[prost(int32, tag = "16")]
    pub archives: i32,
    /// 投稿时间
    #[prost(int64, tag = "17")]
    pub ptime: i64,
    /// season类型名称
    #[prost(string, tag = "18")]
    pub season_type_name: ::prost::alloc::string::String,
    /// 地区
    #[prost(string, tag = "19")]
    pub area: ::prost::alloc::string::String,
    /// 作品风格
    #[prost(string, tag = "20")]
    pub style: ::prost::alloc::string::String,
    /// 描述信息
    #[prost(string, tag = "21")]
    pub label: ::prost::alloc::string::String,
    /// 评分
    #[prost(double, tag = "22")]
    pub rating: f64,
    /// 投票数
    #[prost(int32, tag = "23")]
    pub vote: i32,
    /// 角标
    #[prost(message, repeated, tag = "24")]
    pub badges: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(string, tag = "25")]
    pub styles: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "26")]
    pub module_id: i64,
    ///
    #[prost(string, tag = "27")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "28")]
    pub face_nft_new: i32,
    ///
    #[prost(message, optional, tag = "29")]
    pub nft_face_icon: ::core::option::Option<NftFaceIcon>,
}
/// 认证信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    /// 认证类型
    /// 127:未认证 0:个人 1:机构
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 认证描述
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
/// 角标
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonStyle {
    /// 角标文案
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// 文案日间色值
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    /// 文案夜间色值
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    /// 背景日间色值
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    /// 背景夜间色值
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    /// 边框日间色值
    #[prost(string, tag = "6")]
    pub border_color: ::prost::alloc::string::String,
    /// 边框夜间色值
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    /// 角标样式
    /// 1:填充模式 2:镂空模式
    #[prost(int32, tag = "8")]
    pub bg_style: i32,
}
/// Generated client implementations.
pub mod search_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 搜索
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
        /// 获取搜索建议
        pub async fn suggest3(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestionResult3Req>,
        ) -> std::result::Result<
            tonic::Response<super::SuggestionResult3Reply>,
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
                "/bilibili.app.interface.v1.Search/Suggest3",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.app.interface.v1.Search", "Suggest3"));
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn default_words(
            &mut self,
            request: impl tonic::IntoRequest<super::DefaultWordsReq>,
        ) -> std::result::Result<
            tonic::Response<super::DefaultWordsReply>,
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
                "/bilibili.app.interface.v1.Search/DefaultWords",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.Search", "DefaultWords"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod search_test_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct SearchTestClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchTestClient<tonic::transport::Channel> {
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
    impl<T> SearchTestClient<T>
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
        ) -> SearchTestClient<InterceptedService<T, F>>
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
            SearchTestClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn not_exist(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestionResult3Req>,
        ) -> std::result::Result<
            tonic::Response<super::SuggestionResult3Reply>,
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
                "/bilibili.app.interface.v1.SearchTest/NotExist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.app.interface.v1.SearchTest", "NotExist"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
