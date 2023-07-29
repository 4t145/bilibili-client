/// 动画效果
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimateIcon {
    /// icon文件
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 动效json文件
    #[prost(string, tag = "2")]
    pub json: ::prost::alloc::string::String,
}
/// 活动入口-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntranceReply {
    /// 展示图标
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    /// 活动名称
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// 活动跳转链接
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// 动画效果
    #[prost(message, optional, tag = "4")]
    pub animate_icon: ::core::option::Option<AnimateIcon>,
}
/// 活动入口-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntranceReq {}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyReply {
    ///
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivacyReq {
    ///
    #[prost(string, tag = "1")]
    pub activity_uid: ::prost::alloc::string::String,
}
/// 首页弹窗-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowReply {
    /// 弹窗类型
    /// 0:弹窗 1:普通页面
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 跳转链接
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// 上报数据字段
    #[prost(string, tag = "3")]
    pub report_data: ::prost::alloc::string::String,
}
/// 首页弹窗-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowReq {}
/// Generated client implementations.
pub mod fission_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Fission裂变
    #[derive(Debug, Clone)]
    pub struct FissionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FissionClient<tonic::transport::Channel> {
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
    impl<T> FissionClient<T>
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
        ) -> FissionClient<InterceptedService<T, F>>
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
            FissionClient::new(InterceptedService::new(inner, interceptor))
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
        /// 活动入口
        pub async fn entrance(
            &mut self,
            request: impl tonic::IntoRequest<super::EntranceReq>,
        ) -> std::result::Result<tonic::Response<super::EntranceReply>, tonic::Status> {
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
                "/bilibili.account.fission.v1.Fission/Entrance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.account.fission.v1.Fission", "Entrance"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 首页弹窗
        pub async fn window(
            &mut self,
            request: impl tonic::IntoRequest<super::WindowReq>,
        ) -> std::result::Result<tonic::Response<super::WindowReply>, tonic::Status> {
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
                "/bilibili.account.fission.v1.Fission/Window",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.account.fission.v1.Fission", "Window"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn privacy(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivacyReq>,
        ) -> std::result::Result<tonic::Response<super::PrivacyReply>, tonic::Status> {
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
                "/bilibili.account.fission.v1.Fission/Privacy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("bilibili.account.fission.v1.Fission", "Privacy"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
