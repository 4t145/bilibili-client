use std::collections::HashMap;

use crate::consts::*;
use crate::passport::qrcode::*;
use crate::api::live::send::*;

use crate::api_trait::{fetch, JsonApi};


pub struct Client {
    http_client: reqwest::Client,
    // cookies: HashMap<String, String>,
    status: ClientStatus
}

impl Client {
    fn new() -> Self {
        Self {
            http_client: reqwest::Client::builder().user_agent(AGENT).build().unwrap(),
            // cookies: HashMap::new(),
            status: ClientStatus::Offline
        }
    }

    #[inline]
    async fn fetch<Api: JsonApi>(&self, req: Api::Request) -> Api::Response {
        fetch::<Api>(&self.http_client, req).await
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
    fn make_live_send_req(&mut self, roomid: u64, msg: impl Into<String>) -> LiveSendReq {
        self.live_send_req_generator.gen(roomid, msg.into(), self.cookies.bili_jct.clone())
    }
}

pub struct Cookies {
    bili_jct: String,
    /* may impl later */
    // sess_data: String,
    // dede_user_id: String,
    // dede_user_id_ckmd5: String
}
pub struct Offline {

}

impl Client {
    pub async fn login(&mut self) {
        let client = reqwest::Client::builder().user_agent(AGENT).build().unwrap();
        let resp = self.fetch::<GetLoginUrl>(()).await;
        let oauth_key = resp.data.oauth_key;
        // scan qrcode...
        loop {
            let resp = self.fetch::<GetLoginInfo>(GetLoginInfoReq {oauth_key:oauth_key.clone()}).await;
            match resp.data {
                GetLoginInfoRespData::Code(code) => {
                    match code {
                        -4 => {

                        }
                        -5 => {

                        }
                        _ => {}
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
                },
                GetLoginInfoRespData::Body { url } => {
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
                    break;
                },
            }
        }
    }
}

/** Live Api */
impl Client {
    /// send danmaku
    pub async fn send_danmaku_to_live(&mut self, roomid: u64, msg: impl Into<String>) -> Result<(), ()> {
        let online = self.status.map_online_mut().ok_or(())?;
        let req = online.make_live_send_req(roomid, msg);
        self.fetch::<LiveSend>(req).await;
        return Ok(())
    }
    
}
// pub fn request_passport() {

//     let url = PASSPORT_HOST_URL;
//     url.
//     let request = self.agent
//             .request(method, format!("{}{}", PASSPORT_HOST_URL, path).as_str());
// }