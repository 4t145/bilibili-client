use crate::consts::*;
use crate::passport::qrcode::*;
use crate::api::live::send::*;

use crate::api_trait::{Api, ApiError};
use crate::logger::{Logger, Log, LogLevel};

// macro_rules! debug {
//     ($client:expr, $msg: expr) => {
//         $client.logger.text(Log{
//             content: $msg.to_owned().into(),
//             level: LogLevel::Debug,
//         })
//     };
// }


macro_rules! info {
    ($client:expr, $msg: expr) => {
        $client.logger.text(Log{
            content: $msg.to_owned().into(),
            level: LogLevel::Info,
        })
    };
}

macro_rules! warn {
    ($client:expr, $msg: expr) => {
        $client.logger.text(Log{
            content: $msg.to_owned().into(),
            level: LogLevel::Warn,
        })
    };
}


pub struct Client<L: Logger> {
    http_client: reqwest::Client,
    // cookies: HashMap<String, String>,
    status: ClientStatus,
    logger: L
}

impl<L:Logger> Client<L> {
    pub fn new(logger: L) -> Self {
        Self {
            http_client: reqwest::Client::builder().user_agent(AGENT).build().unwrap(),
            // cookies: HashMap::new(),
            status: ClientStatus::Offline,
            logger,
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
}

pub enum ClientStatus {
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

pub struct ClientStatusOnline {
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

pub struct Cookies {
    bili_jct: String,
    /* may impl later */
    // sess_data: String,
    // dede_user_id: String,
    // dede_user_id_ckmd5: String
}

pub enum ClientError {
    Api(ApiError),
    Offline,
}

impl<L:Logger> Client<L> {
    pub async fn login(&mut self) -> Result<bool, ClientError> {
        let resp = self.fd_req::<GetLoginUrl>(()).await.map_err(ClientError::Api)?;
        let oauth_key = resp.data.oauth_key;
        // scan qrcode...
        loop {
            let resp = self.fd_req::<GetLoginInfo>(
                GetLoginInfoReq {oauth_key:oauth_key.clone()}
            ).await.map_err(ClientError::Api)?;
            match resp.data {
                GetLoginInfoRespData::Code(code) => {
                    match code {
                        -2 => {
                            warn!(self, "二维码已过期");
                            return Ok(false);
                        }
                        -4 => info!(self, "二维码尚未扫描"),
                        -5 => info!(self, "二维码已扫描"),
                        _ => {}
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
                },
                GetLoginInfoRespData::Body { url } => {
                    self.logger.qrcode(url.as_bytes());
                    let parse = reqwest::Url::parse(&url).unwrap();
                    let mut pairs = parse.query_pairs();
                    let (_, v) = pairs.find(|(k,_)|{*k == "bili_jct"}).unwrap();
                    let bili_jct = v.to_string();
                    // self.cookies.insert(k.to_string(), v.to_string());
                    self.status = ClientStatus::Online (
                        ClientStatusOnline {
                            cookies: Cookies { bili_jct: bili_jct.clone() },
                            live_send_req_generator: LiveSendReqGenerator::new() 
                        }
                    );
                    info!(self, "已登录成功");
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
        let resp = self.fd_req::<LiveSend>(req).await.map_err(ClientError::Api)?;
        return Ok(resp)
    }
}