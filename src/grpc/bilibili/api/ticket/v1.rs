///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTicketRequest {
    /// 包含:
    /// + x-fingerprint(包含手机各类信息, 使用 datacenter.hakase.protobuf.AndroidDeviceInfo 生成)
    /// + x-exbadbasket(?)
    #[prost(map = "string, bytes", tag = "1")]
    pub context: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// 暂时固定为 ec01
    #[prost(string, tag = "2")]
    pub key_id: ::prost::alloc::string::String,
    ///
    #[prost(bytes = "vec", tag = "3")]
    pub sign: ::prost::alloc::vec::Vec<u8>,
    /// 暂时留空
    #[prost(string, tag = "4")]
    pub token: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTicketResponse {
    /// x-bili-ticket
    #[prost(string, tag = "1")]
    pub ticket: ::prost::alloc::string::String,
    /// 有效期起
    #[prost(int64, tag = "2")]
    pub created_at: i64,
    /// 有效期
    #[prost(int64, tag = "3")]
    pub ttl: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub context: ::core::option::Option<get_ticket_response::Context>,
}
/// Nested message and enum types in `GetTicketResponse`.
pub mod get_ticket_response {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Context {
        ///
        #[prost(string, tag = "1")]
        pub v_voucher: ::prost::alloc::string::String,
    }
}
/// Generated client implementations.
pub mod ticket_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TicketClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TicketClient<tonic::transport::Channel> {
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
    impl<T> TicketClient<T>
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
        ) -> TicketClient<InterceptedService<T, F>>
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
            TicketClient::new(InterceptedService::new(inner, interceptor))
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
        /// 获取鉴权用 Ticket
        pub async fn get_ticket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTicketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTicketResponse>,
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
                "/bilibili.api.ticket.v1.Ticket/GetTicket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("bilibili.api.ticket.v1.Ticket", "GetTicket"));
            self.inner.unary(req, path, codec).await
        }
    }
}
