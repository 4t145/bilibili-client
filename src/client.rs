use std::collections::HashMap;

use crate::consts::*;
use crate::passport::qrcode::*;
use crate::api_trait::{fetch, JsonApi};


pub struct Client {
    http_client: reqwest::Client,
    cookies: HashMap<String, String>
}

impl Client {
    fn new() -> Self {
        Self {
            http_client: reqwest::Client::builder().user_agent(AGENT).build().unwrap(),
            cookies: HashMap::new()
        }
    }

    #[inline]
    async fn fetch<Api: JsonApi>(&self, req: Api::Request) -> Api::Response {
        fetch::<Api>(&self.http_client, req).await
    }
}

pub enum ClientStatus {
    Offline,

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
                    let (k, v) = pairs.find(|(k,_)|{*k == "bili_jct"}).unwrap();
                    self.cookies.insert(k.to_string(), v.to_string());
                },
            }
        }
    }
}
// pub fn request_passport() {

//     let url = PASSPORT_HOST_URL;
//     url.
//     let request = self.agent
//             .request(method, format!("{}{}", PASSPORT_HOST_URL, path).as_str());
// }