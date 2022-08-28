use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

use crate::consts::*;
use crate::api::passport::qrcode::*;
use crate::api::live::msg::send::*;
use crate::api::live::msg::LiveDanmaku;
use crate::api::{Api, ApiError};
use crate::logger::{Logger};
use reqwest_cookie_store::CookieStoreRwLock;

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
    cookie_store: Arc<CookieStoreRwLock>,
    cookie_file_writer: Option<BufWriter<File>>,
    pub logger: L
}

pub struct ClientConfig<'p, L: Logger> {
    pub logger: L,
    pub cookie_file: Option<&'p Path>
}

#[derive(Debug)]
pub enum ClientError {
    Api(ApiError),
    Fs(std::io::Error),
    CookieStore(cookie_store::Error),
    Offline,
    NoCookieFile
}

impl<L:Logger> Client<L> {
    pub fn new(config: ClientConfig<L>) -> Result<Self, ClientError> {
        use ClientError::*;
        use std::io::{BufReader};
        use std::fs::OpenOptions;
        let (cookie_store, writer) = match config.cookie_file {
            Some(path) => {
                let reader = OpenOptions::new().create(true).write(true).read(true).open(&path).map(BufReader::new).map_err(Fs)?;
                let s = cookie_store::CookieStore::load_json(reader).map_err(CookieStore)?;
                let writer = OpenOptions::new().write(true).open(&path).map(BufWriter::new).map_err(Fs)?;
                (s, Some(writer))
            },
            None => {
                (cookie_store::CookieStore::default(), None)
            },
        };

        let status = cookie_store.get("bilibili.com", "/", "bili_jct").map(|c|{
            ClientStatus::Online(ClientStatusOnline {
                bili_jct: c.value().into(),
                live_send_req_generator: LiveSendReqGenerator::new(),
            }) 
        }).unwrap_or(ClientStatus::Offline);

        let cookie_store = reqwest_cookie_store::CookieStoreRwLock::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        let client = Self {
            http_client: reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .user_agent(AGENT)
            .build()
            .unwrap(),
            cookie_store: cookie_store,
            status,
            logger:config.logger,
            cookie_file_writer: writer,
        };
        Ok(client)
    }


    expose_logger_method!(self, critical, error, warn, info, debug);

    pub fn is_online(&self) -> bool {
        match self.status {
            ClientStatus::Online(_) => true,
            _ => false,
        }
    }

    pub fn save_cookies(&mut self) -> Result<(), ClientError> {
        use std::io::{Write};
        if let Some(writer) = &mut self.cookie_file_writer {
            {
                let _ = &self.cookie_store.read().unwrap().save_json(writer).map_err(ClientError::CookieStore)?;
            }
            writer.flush().map_err(ClientError::Fs)?;
            return Ok(())
        }
        Err(ClientError::NoCookieFile)
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
        let resp = self.http_client.execute(request).await.map_err(ApiError::Http)?;
        resp.json().await.map_err(ApiError::Deser)
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
    bili_jct: String,
    live_send_req_generator: LiveSendReqGenerator
}

impl ClientStatusOnline {
    #[inline]
    fn make_live_send_req(&mut self, roomid: u64, msg: LiveDanmaku) -> LiveSendReq {
        match msg {
            LiveDanmaku::Emoticon(e) => {
                self.live_send_req_generator.gen_emoticon(roomid, e, self.bili_jct.clone())
            },
            LiveDanmaku::Text(msg) => {
                self.live_send_req_generator.gen(roomid, msg, self.bili_jct.clone())
            },
        }
    }
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
                            self.warn(format!("nosuch code {other}: {msg}").as_str());

                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                },
                GetLoginInfoRespData::Body { url } => {
                    let parse = reqwest::Url::parse(&url).unwrap();
                    let mut pairs = parse.query_pairs();
                    let (_, v) = pairs.find(|(k,_)|{*k == "bili_jct"}).unwrap();
                    let bili_jct = v.to_string();
                    self.status = ClientStatus::Online (
                        ClientStatusOnline {
                            bili_jct: bili_jct.clone(),
                            live_send_req_generator: LiveSendReqGenerator::new() 
                        }
                    );
                    self.info("已登录成功");
                    if self.cookie_file_writer.is_some() {
                        self.save_cookies()?;
                    }
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