///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgreePolicyReply {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgreePolicyReq {
    ///
    #[prost(enumeration = "PolicyType", tag = "1")]
    pub policy_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicInfoReply {
    ///
    #[prost(string, tag = "1")]
    pub customer_service_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub agreement_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub privacy_url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub links: ::prost::alloc::vec::Vec<Link>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicInfoReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Display {
    ///
    #[prost(string, tag = "1")]
    pub bg_theme_light: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bg_theme_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub nft_poster: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub nft_raw: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPolicyReply {
    ///
    #[prost(string, tag = "1")]
    pub short_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub detail_jump: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPolicyReq {
    ///
    #[prost(enumeration = "PolicyType", tag = "1")]
    pub policy_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoReply {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub avatar_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub help_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInfoReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub link_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub track_event_id: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNftByMidReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
    ///
    #[prost(int64, tag = "2")]
    pub anchor_id: i64,
    ///
    #[prost(bool, tag = "3")]
    pub end: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNftByMidReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub category: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub biz_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "4")]
    pub anchor_id: i64,
    ///
    #[prost(int64, tag = "5")]
    pub page_size: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrderByMidReply {
    ///
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    ///
    #[prost(int64, tag = "2")]
    pub anchor_id: i64,
    ///
    #[prost(bool, tag = "3")]
    pub end: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrderByMidReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub anchor_id: i64,
    ///
    #[prost(int64, tag = "3")]
    pub page_size: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nft {
    ///
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub item_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub serial_number: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub issuer: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub display: ::core::option::Option<Display>,
    ///
    #[prost(string, tag = "6")]
    pub detail_url: ::prost::alloc::string::String,
    ///
    #[prost(enumeration = "NftStatus", tag = "7")]
    pub nft_status: i32,
    ///
    #[prost(int64, tag = "8")]
    pub item_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    ///
    #[prost(string, tag = "1")]
    pub item_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub serial_number: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub tx_hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub tx_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub issuer: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub issue_time: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub token_id: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub display: ::core::option::Option<Display>,
    ///
    #[prost(string, tag = "9")]
    pub contract_address: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub hash_jump: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub contract_jump: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "12")]
    pub disable_browser_jump: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCheckReply {
    ///
    #[prost(int32, tag = "1")]
    pub policy_status: i32,
    ///
    #[prost(int32, tag = "2")]
    pub gt14: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserCheckReq {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int32, tag = "2")]
    pub policy_type: i32,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gt14Status {
    ///
    Lt14 = 0,
    ///
    Ge14 = 1,
    ///
    UnknownGt14 = 2,
}
impl Gt14Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Gt14Status::Lt14 => "LT14",
            Gt14Status::Ge14 => "GE14",
            Gt14Status::UnknownGt14 => "UNKNOWN_GT14",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LT14" => Some(Self::Lt14),
            "GE14" => Some(Self::Ge14),
            "UNKNOWN_GT14" => Some(Self::UnknownGt14),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NftStatus {
    ///
    Undefined = 0,
    ///
    Normal = 1,
    ///
    Doing = 2,
}
impl NftStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NftStatus::Undefined => "UNDEFINED",
            NftStatus::Normal => "NORMAL",
            NftStatus::Doing => "DOING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNDEFINED" => Some(Self::Undefined),
            "NORMAL" => Some(Self::Normal),
            "DOING" => Some(Self::Doing),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolicyAgreeStatus {
    ///
    Unsigned = 0,
    ///
    Accepted = 1,
    ///
    Expired = 2,
}
impl PolicyAgreeStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PolicyAgreeStatus::Unsigned => "UNSIGNED",
            PolicyAgreeStatus::Accepted => "ACCEPTED",
            PolicyAgreeStatus::Expired => "EXPIRED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSIGNED" => Some(Self::Unsigned),
            "ACCEPTED" => Some(Self::Accepted),
            "EXPIRED" => Some(Self::Expired),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolicyType {
    ///
    UnknownPolicy = 0,
    ///
    Wallet = 1,
    ///
    Sale = 2,
}
impl PolicyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PolicyType::UnknownPolicy => "UNKNOWN_POLICY",
            PolicyType::Wallet => "WALLET",
            PolicyType::Sale => "SALE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_POLICY" => Some(Self::UnknownPolicy),
            "WALLET" => Some(Self::Wallet),
            "SALE" => Some(Self::Sale),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod gallery_interface_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct GalleryInterfaceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GalleryInterfaceClient<tonic::transport::Channel> {
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
    impl<T> GalleryInterfaceClient<T>
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
        ) -> GalleryInterfaceClient<InterceptedService<T, F>>
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
            GalleryInterfaceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::google::protobuf::Empty,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::google::protobuf::Empty>,
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/Ping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.pangu.gallery.v1.GalleryInterface", "Ping"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn user_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInfoReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserInfoReply>,
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/UserInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "UserInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn list_nft_by_mid(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNftByMidReq>,
        ) -> std::result::Result<
            tonic::Response<super::ListNftByMidReply>,
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/ListNFTByMid",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "ListNFTByMid",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn list_order_by_mid(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrderByMidReq>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrderByMidReply>,
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/ListOrderByMid",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "ListOrderByMid",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn basic_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BasicInfoReq>,
        ) -> std::result::Result<tonic::Response<super::BasicInfoReply>, tonic::Status> {
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/BasicInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "BasicInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn user_check(
            &mut self,
            request: impl tonic::IntoRequest<super::UserCheckReq>,
        ) -> std::result::Result<tonic::Response<super::UserCheckReply>, tonic::Status> {
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/UserCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "UserCheck",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn agree_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AgreePolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::AgreePolicyReply>,
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/AgreePolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "AgreePolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_last_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastPolicyReq>,
        ) -> std::result::Result<
            tonic::Response<super::GetLastPolicyReply>,
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
                "/bilibili.pangu.gallery.v1.GalleryInterface/GetLastPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.pangu.gallery.v1.GalleryInterface",
                        "GetLastPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
