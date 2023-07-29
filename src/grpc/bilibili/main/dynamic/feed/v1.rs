/// 创建动态-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDynReq {
    /// 用户创建接口meta信息
    #[prost(message, optional, tag = "1")]
    pub meta: ::core::option::Option<
        super::super::super::super::dynamic::UserCreateMeta,
    >,
    /// 发布的内容
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<
        super::super::super::super::dynamic::CreateContent,
    >,
    /// 发布类型
    #[prost(enumeration = "super::super::super::super::dynamic::CreateScene", tag = "3")]
    pub scene: i32,
    /// 图片内容
    #[prost(message, repeated, tag = "4")]
    pub pics: ::prost::alloc::vec::Vec<super::super::super::super::dynamic::CreatePic>,
    /// 转发源
    #[prost(message, optional, tag = "5")]
    pub repost_src: ::core::option::Option<
        super::super::super::super::dynamic::DynIdentity,
    >,
    /// 动态视频
    #[prost(message, optional, tag = "6")]
    pub video: ::core::option::Option<
        super::super::super::super::dynamic::CreateDynVideo,
    >,
    /// 通用模板类型：2048方图 2049竖图 其他值无效
    #[prost(int64, tag = "7")]
    pub sketch_type: i64,
    /// 通用模板的元内容（网页内容）
    #[prost(message, optional, tag = "8")]
    pub sketch: ::core::option::Option<super::super::super::super::dynamic::Sketch>,
    /// 小程序的内容
    #[prost(message, optional, tag = "9")]
    pub program: ::core::option::Option<super::super::super::super::dynamic::Program>,
    /// 动态附加小卡
    #[prost(message, optional, tag = "10")]
    pub dyn_tag: ::core::option::Option<super::super::super::super::dynamic::CreateTag>,
    /// 动态附加大卡
    #[prost(message, optional, tag = "11")]
    pub attach_card: ::core::option::Option<
        super::super::super::super::dynamic::CreateAttachCard,
    >,
    /// 特殊的创建选项
    #[prost(message, optional, tag = "12")]
    pub option: ::core::option::Option<
        super::super::super::super::dynamic::CreateOption,
    >,
    ///
    #[prost(message, optional, tag = "13")]
    pub topic: ::core::option::Option<super::super::super::super::dynamic::CreateTopic>,
    ///
    #[prost(string, tag = "14")]
    pub upload_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInitCheckReq {
    ///
    #[prost(int32, tag = "1")]
    pub scene: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub meta: ::core::option::Option<super::super::super::super::dynamic::MetaDataCtrl>,
    ///
    #[prost(message, optional, tag = "3")]
    pub repost: ::core::option::Option<
        super::super::super::super::dynamic::RepostInitCheck,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePageInfosReq {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePageInfosRsp {
    ///
    #[prost(message, optional, tag = "1")]
    pub topic: ::core::option::Option<CreatePageTopicInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePageTopicInfo {
    ///
    #[prost(int64, tag = "1")]
    pub topic_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub topic_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePermissionButtonClickReq {
    ///
    #[prost(enumeration = "DynamicButtonClickBizType", tag = "1")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePermissionButtonClickRsp {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlusButtonClickReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePlusButtonClickRsp {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicButtonClickReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicButtonClickRsp {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotSearchReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotSearchRsp {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<hot_search_rsp::Item>,
    ///
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `HotSearchRsp`.
pub mod hot_search_rsp {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        ///
        #[prost(string, tag = "1")]
        pub words: ::prost::alloc::string::String,
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveButtonClickReq {
    ///
    #[prost(int64, tag = "1")]
    pub uid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub reserve_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub reserve_total: i64,
    ///
    #[prost(int32, tag = "4")]
    pub cur_btn_status: i32,
    ///
    #[prost(string, tag = "5")]
    pub spmid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub dyn_id: i64,
    ///
    #[prost(int64, tag = "7")]
    pub dyn_type: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveButtonClickResp {
    ///
    #[prost(enumeration = "ReserveButtonStatus", tag = "1")]
    pub final_btn_status: i32,
    ///
    #[prost(enumeration = "ReserveButtonMode", tag = "2")]
    pub btn_mode: i32,
    ///
    #[prost(int64, tag = "3")]
    pub reserve_update: i64,
    ///
    #[prost(string, tag = "4")]
    pub desc_update: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "5")]
    pub has_activity: bool,
    ///
    #[prost(string, tag = "6")]
    pub activity_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub toast: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitCheckReq {
    ///
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<
        super::super::super::super::dynamic::CreateContent,
    >,
    ///
    #[prost(message, repeated, tag = "2")]
    pub pics: ::prost::alloc::vec::Vec<super::super::super::super::dynamic::CreatePic>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitCheckRsp {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestReq {
    ///
    #[prost(string, tag = "1")]
    pub s: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestRsp {
    ///
    #[prost(string, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "2")]
    pub track_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DynamicButtonClickBizType {
    ///
    None = 0,
    ///
    Live = 1,
    ///
    DynUp = 2,
}
impl DynamicButtonClickBizType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DynamicButtonClickBizType::None => "DYNAMIC_BUTTON_CLICK_BIZ_TYPE_NONE",
            DynamicButtonClickBizType::Live => "DYNAMIC_BUTTON_CLICK_BIZ_TYPE_LIVE",
            DynamicButtonClickBizType::DynUp => "DYNAMIC_BUTTON_CLICK_BIZ_TYPE_DYN_UP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DYNAMIC_BUTTON_CLICK_BIZ_TYPE_NONE" => Some(Self::None),
            "DYNAMIC_BUTTON_CLICK_BIZ_TYPE_LIVE" => Some(Self::Live),
            "DYNAMIC_BUTTON_CLICK_BIZ_TYPE_DYN_UP" => Some(Self::DynUp),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveButtonMode {
    ///
    None = 0,
    ///
    Reserve = 1,
    ///
    UpCancel = 2,
}
impl ReserveButtonMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReserveButtonMode::None => "RESERVE_BUTTON_MODE_NONE",
            ReserveButtonMode::Reserve => "RESERVE_BUTTON_MODE_RESERVE",
            ReserveButtonMode::UpCancel => "RESERVE_BUTTON_MODE_UP_CANCEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVE_BUTTON_MODE_NONE" => Some(Self::None),
            "RESERVE_BUTTON_MODE_RESERVE" => Some(Self::Reserve),
            "RESERVE_BUTTON_MODE_UP_CANCEL" => Some(Self::UpCancel),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReserveButtonStatus {
    ///
    None = 0,
    ///
    Uncheck = 1,
    ///
    Check = 2,
}
impl ReserveButtonStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReserveButtonStatus::None => "RESERVE_BUTTON_STATUS_NONE",
            ReserveButtonStatus::Uncheck => "RESERVE_BUTTON_STATUS_UNCHECK",
            ReserveButtonStatus::Check => "RESERVE_BUTTON_STATUS_CHECK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESERVE_BUTTON_STATUS_NONE" => Some(Self::None),
            "RESERVE_BUTTON_STATUS_UNCHECK" => Some(Self::Uncheck),
            "RESERVE_BUTTON_STATUS_CHECK" => Some(Self::Check),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod feed_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct FeedClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FeedClient<tonic::transport::Channel> {
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
    impl<T> FeedClient<T>
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
        ) -> FeedClient<InterceptedService<T, F>>
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
            FeedClient::new(InterceptedService::new(inner, interceptor))
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
        /// 发布页预校验
        pub async fn create_init_check(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInitCheckReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::dynamic::CreateCheckResp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/CreateInitCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.dynamic.feed.v1.Feed",
                        "CreateInitCheck",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn submit_check(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitCheckReq>,
        ) -> std::result::Result<tonic::Response<super::SubmitCheckRsp>, tonic::Status> {
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
                "/bilibili.main.dynamic.feed.v1.Feed/SubmitCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "SubmitCheck"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 创建动态
        pub async fn create_dyn(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDynReq>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::dynamic::CreateResp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/CreateDyn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "CreateDyn"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 根据name取uid
        pub async fn get_uid_by_name(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::dynamic::GetUidByNameReq,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::dynamic::GetUidByNameRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/GetUidByName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "GetUidByName"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// at用户推荐列表
        pub async fn at_list(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::dynamic::AtListReq,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::dynamic::AtListRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/AtList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "AtList"));
            self.inner.unary(req, path, codec).await
        }
        /// at用户搜索列表
        pub async fn at_search(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::dynamic::AtSearchReq,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::dynamic::AtListRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/AtSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "AtSearch"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn reserve_button_click(
            &mut self,
            request: impl tonic::IntoRequest<super::ReserveButtonClickReq>,
        ) -> std::result::Result<
            tonic::Response<super::ReserveButtonClickResp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/ReserveButtonClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.dynamic.feed.v1.Feed",
                        "ReserveButtonClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn create_plus_button_click(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePlusButtonClickReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePlusButtonClickRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/CreatePlusButtonClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.dynamic.feed.v1.Feed",
                        "CreatePlusButtonClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn hot_search(
            &mut self,
            request: impl tonic::IntoRequest<super::HotSearchReq>,
        ) -> std::result::Result<tonic::Response<super::HotSearchRsp>, tonic::Status> {
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
                "/bilibili.main.dynamic.feed.v1.Feed/HotSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "HotSearch"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn suggest(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestReq>,
        ) -> std::result::Result<tonic::Response<super::SuggestRsp>, tonic::Status> {
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
                "/bilibili.main.dynamic.feed.v1.Feed/Suggest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.main.dynamic.feed.v1.Feed", "Suggest"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn dynamic_button_click(
            &mut self,
            request: impl tonic::IntoRequest<super::DynamicButtonClickReq>,
        ) -> std::result::Result<
            tonic::Response<super::DynamicButtonClickRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/DynamicButtonClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.dynamic.feed.v1.Feed",
                        "DynamicButtonClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn create_permission_button_click(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePermissionButtonClickReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePermissionButtonClickRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/CreatePermissionButtonClick",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.dynamic.feed.v1.Feed",
                        "CreatePermissionButtonClick",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn create_page_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePageInfosReq>,
        ) -> std::result::Result<
            tonic::Response<super::CreatePageInfosRsp>,
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
                "/bilibili.main.dynamic.feed.v1.Feed/CreatePageInfos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.main.dynamic.feed.v1.Feed",
                        "CreatePageInfos",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
