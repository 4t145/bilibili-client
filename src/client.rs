use std::sync::Arc;

use crate::consts::*;
use crate::api::passport::qrcode::*;
use crate::api::live::msg::send::*;
use crate::api::live::msg::LiveDanmaku;
use crate::api::{Api, ApiError};
use crate::logger::{Logger};
use reqwest::cookie::Jar;


macro_rules! expose_logger_method {
    ($self:ident, $($method:ident),*) => {
        $(
            #[inline]
            pub fn $method(&mut $self, msg:impl Into<String>) {
                $self.logger.$method(msg.into());
            }
        )*
    };
}

pub struct Client<L: Logger> {
    http_client: reqwest::Client,
    status: ClientStatus,
    // cookie_store: Arc<Jar>,
    pub logger: L
}

impl<L:Logger> Client<L> {
    pub fn new(logger: L) -> Self {
        let cookie_store = Arc::new(Jar::default());
        let mut client = Self {
            http_client: reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .cookie_store(true)
            .user_agent(AGENT)
            .build()
            .unwrap(),
            // cookie_store,
            status: ClientStatus::Offline,
            logger,
        };
        client.info("client created");
        client.info(format!("log level {:?}", client.logger.level()).as_str());
        client
    }

    expose_logger_method!(self, critical, error, warn, info, debug);

    pub fn is_online(&self) -> bool {
        match self.status {
            ClientStatus::Online(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub async fn fd_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        
        let request = A::form_data_req(&self.http_client, req)?;
        self.http_client.execute(request).await
            .map_err(ApiError::Http)?
            .json().await.map_err(ApiError::Deser)
    }

    #[inline]
    pub async fn json_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        let request = A::form_data_req(&self.http_client, req)?;
        self.http_client.execute(request).await
            .map_err(ApiError::Http)?
            .json().await.map_err(ApiError::Deser)
    }

    #[inline]
    pub async fn urlencoded_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        let request = A::urlencoded_req(&self.http_client, req)?;

        self.http_client.execute(request).await
            .map_err(ApiError::Http)?
            .json().await.map_err(ApiError::Deser)
    }
}

enum ClientStatus {
    Offline,
    Online(ClientStatusOnline)
}

impl ClientStatus {
    // fn map_online(&self) -> Result<> {

    // }
    fn map_online_mut(&mut self) -> Option<&mut ClientStatusOnline> {
        match self {
            ClientStatus::Online(s) => Some(s),
            _ => None,
        }
    }
}

struct ClientStatusOnline {
    cookies: Cookies,
    live_send_req_generator: LiveSendReqGenerator
}

impl ClientStatusOnline {
    #[inline]
    fn make_live_send_req(&mut self, roomid: u64, msg: LiveDanmaku) -> LiveSendReq {
        match msg {
            LiveDanmaku::Emoticon(e) => {
                self.live_send_req_generator.gen_emoticon(roomid, e, self.cookies.bili_jct.clone())
            },
            LiveDanmaku::Text(msg) => {
                self.live_send_req_generator.gen(roomid, msg, self.cookies.bili_jct.clone())
            },
        }
    }
}

struct Cookies {
    bili_jct: String,
    /* may impl later */
    // sess_data: String,
    // dede_user_id: String,
    // dede_user_id_ckmd5: String
}

#[derive(Debug)]
pub enum ClientError {
    Api(ApiError),
    Offline,
}

impl<L:Logger> Client<L> {
    pub async fn login(&mut self) -> Result<bool, ClientError> {
        self.debug("START LOGIN");
        let resp = self.urlencoded_req::<GetLoginUrl>(()).await.map_err(ClientError::Api)?;
        self.debug("GENERATED RESP");
        let oauth_key = resp.data.oauth_key;
        self.debug(format!("AUTHKEY = {oauth_key}").as_str());
        self.logger.qrcode(resp.data.url.as_bytes());
        // scan qrcode...
        loop {
            let resp = self.urlencoded_req::<GetLoginInfo>(
                GetLoginInfoReq {oauth_key:oauth_key.clone()}
            ).await.map_err(ClientError::Api)?;
            match resp.data {
                GetLoginInfoRespData::Code(code) => {
                    match code {
                        -1 => {
                            self.critical("缺少oauth_key");
                        }
                        -2 => {
                            self.warn("二维码已过期");
                            return Ok(false);
                        }
                        -4 => self.debug("二维码尚未扫描"),
                        -5 => self.info("二维码已扫描"),
                        other => {
                            let msg = resp.message.unwrap_or_default();
                            self.debug(format!("nosuch code {other}: {msg}").as_str());

                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                },
                GetLoginInfoRespData::Body { url } => {
                    let parse = reqwest::Url::parse(&url).unwrap();
                    let mut pairs = parse.query_pairs();
                    let (_, v) = pairs.find(|(k,_)|{*k == "bili_jct"}).unwrap();
                    // self.cookie_store.add_cookie_str(cookie, url)
                    let bili_jct = v.to_string();
                    self.status = ClientStatus::Online (
                        ClientStatusOnline {
                            cookies: Cookies { bili_jct: bili_jct.clone() },
                            live_send_req_generator: LiveSendReqGenerator::new() 
                        }
                    );
                    self.info("已登录成功");
                    return Ok(true);
                },
            }
        }
    }
}

/** Live Api */
impl<L:Logger> Client<L> {
    /// send danmaku
    pub async fn send_danmaku_to_live(&mut self, roomid: u64, danmaku: LiveDanmaku) -> Result<<LiveSend as Api>::Response, ClientError> {
        let online = self.status.map_online_mut().ok_or(ClientError::Offline)?;
        let req = online.make_live_send_req(roomid, danmaku);
        self.debug(serde_json::json!(req).to_string().as_str());
        let resp = self.urlencoded_req::<LiveSend>(req).await.map_err(ClientError::Api)?;
        return Ok(resp)
    }
}