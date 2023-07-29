/// 播放页信息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReq {
    /// 课程epid(与番剧不互通)
    #[prost(int64, tag = "1")]
    pub ep_id: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 青少年模式
    #[prost(int32, tag = "11")]
    pub teenagers_mode: i32,
    /// 视频编码
    #[prost(
        enumeration = "super::super::super::super::app::playurl::v1::CodeType",
        tag = "12"
    )]
    pub prefer_codec_type: i32,
    /// 是否强制请求预览视频
    #[prost(bool, tag = "13")]
    pub is_preview: bool,
}
/// 投屏地址-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectReq {
    /// 课程epid(与番剧不互通)
    #[prost(int64, tag = "1")]
    pub ep_id: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 投屏协议
    /// 0:默认乐播 1:自建协议 2:云投屏
    #[prost(int32, tag = "11")]
    pub protocol: i32,
    /// 投屏设备
    /// 0:默认其他 1:OTT设备
    #[prost(int32, tag = "12")]
    pub device_type: i32,
    /// 是否flv格式
    #[prost(bool, tag = "13")]
    pub flv_proj: bool,
}
/// 播放页信息-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReply {
    /// 视频url信息
    #[prost(message, optional, tag = "1")]
    pub video_info: ::core::option::Option<
        super::super::super::super::app::playurl::v1::VideoInfo,
    >,
    /// 禁用功能配置
    #[prost(message, optional, tag = "2")]
    pub play_conf: ::core::option::Option<PlayAbilityConf>,
}
/// 禁用功能配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAbilityConf {
    /// 后台播放
    #[prost(bool, tag = "1")]
    pub background_play_disable: bool,
    /// 镜像反转
    #[prost(bool, tag = "2")]
    pub flip_disable: bool,
    /// 支持投屏
    #[prost(bool, tag = "3")]
    pub cast_disable: bool,
    /// 反馈
    #[prost(bool, tag = "4")]
    pub feedback_disable: bool,
    /// 字幕
    #[prost(bool, tag = "5")]
    pub subtitle_disable: bool,
    /// 播放速度
    #[prost(bool, tag = "6")]
    pub playback_rate_disable: bool,
    /// 定时停止播放
    #[prost(bool, tag = "7")]
    pub time_up_disable: bool,
    /// 播放方式
    #[prost(bool, tag = "8")]
    pub playback_mode_disable: bool,
    /// 画面尺寸
    #[prost(bool, tag = "9")]
    pub scale_mode_disable: bool,
    /// 顶
    #[prost(bool, tag = "10")]
    pub like_disable: bool,
    /// 踩
    #[prost(bool, tag = "11")]
    pub dislike_disable: bool,
    /// 投币
    #[prost(bool, tag = "12")]
    pub coin_disable: bool,
    /// 充电
    #[prost(bool, tag = "13")]
    pub elec_disable: bool,
    /// 分享
    #[prost(bool, tag = "14")]
    pub share_disable: bool,
    /// 截图
    #[prost(bool, tag = "15")]
    pub screen_shot_disable: bool,
    /// 锁屏
    #[prost(bool, tag = "16")]
    pub lock_screen_disable: bool,
    /// 相关推荐
    #[prost(bool, tag = "17")]
    pub recommend_disable: bool,
    /// 倍速
    #[prost(bool, tag = "18")]
    pub playback_speed_disable: bool,
    /// 清晰度
    #[prost(bool, tag = "19")]
    pub definition_disable: bool,
    /// 选集
    #[prost(bool, tag = "20")]
    pub selections_disable: bool,
    /// 下一集
    #[prost(bool, tag = "21")]
    pub next_disable: bool,
    /// 编辑弹幕
    #[prost(bool, tag = "22")]
    pub edit_dm_disable: bool,
    /// 外层面板弹幕设置
    #[prost(bool, tag = "25")]
    pub outer_dm_disable: bool,
    /// 三点内弹幕设置
    #[prost(bool, tag = "26")]
    pub inner_dm_disable: bool,
    /// 画中画
    #[prost(bool, tag = "27")]
    pub small_window_disable: bool,
}
/// 投屏地址-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectReply {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<
        super::super::super::super::app::playurl::v1::PlayUrlReply,
    >,
}
/// Generated client implementations.
pub mod play_url_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 课程视频url
    #[derive(Debug, Clone)]
    pub struct PlayUrlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayUrlClient<tonic::transport::Channel> {
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
    impl<T> PlayUrlClient<T>
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
        ) -> PlayUrlClient<InterceptedService<T, F>>
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
            PlayUrlClient::new(InterceptedService::new(inner, interceptor))
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
        /// 播放页信息
        pub async fn play_view(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewReq>,
        ) -> std::result::Result<tonic::Response<super::PlayViewReply>, tonic::Status> {
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
                "/bilibili.cheese.gateway.player.v1.PlayURL/PlayView",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.cheese.gateway.player.v1.PlayURL",
                        "PlayView",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// 投屏地址
        pub async fn project(
            &mut self,
            request: impl tonic::IntoRequest<super::ProjectReq>,
        ) -> std::result::Result<tonic::Response<super::ProjectReply>, tonic::Status> {
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
                "/bilibili.cheese.gateway.player.v1.PlayURL/Project",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "bilibili.cheese.gateway.player.v1.PlayURL",
                        "Project",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
