use reqwest::{
    self,
    cookie::{self, CookieStore},
    Error, Url,
};

use expire::MaybeExpired;
use http_api_util::{
    cache::{ApiCache, FifoCache},
    Api,
};
use serde::{Deserialize, Deserializer, Serialize};

use std::{
    hash::Hash,
    str::FromStr,
    sync::{Arc, OnceLock, RwLock},
    time, convert::Infallible,
};

use crate::{
    api::{
        dynamic::topic::{DynamicTopic, DynamicTopicRequest, DynamicTopicResponse},
        user::{
            cards::{UserCards, UserCardsRequest},
            info::{UserInfo, UserInfoRequest, UserInfoResponse},
        },
        CommonResp,
    },
    consts::AGENT,
};

pub struct ReqwestClient {
    client: reqwest::Client,
    cookie_store: Arc<dyn CookieStore + 'static>,
}

#[derive(Debug)]
pub enum ClientError {
    SerJson(serde_json::Error),
    Reqwest(Error),
    Offline,
    Fail { code: i32, message: String },
}

impl From<serde_json::Error> for ClientError {
    fn from(e: serde_json::Error) -> Self {
        ClientError::SerJson(e)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        ClientError::Reqwest(e)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LoginInfo {
    #[serde(rename = "DedeUserID")]
    pub dede_user_id: Option<String>,
    pub sid: Option<String>,
    #[serde(rename = "DedeUserID__ckMd5")]
    pub dede_user_id_ck_md5: Option<String>,
    #[serde(rename = "SESSDATA")]
    pub sess_data: Option<String>,
    #[serde(rename = "bili_jct")]
    pub bili_jct: Option<String>,
}

impl FromStr for LoginInfo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut login_info = LoginInfo::default();
        s.split(';').for_each(|s| {
            if let Some((key, val)) = s.split_once('=') {
                match key.trim() {
                    "DedeUserID" => login_info.dede_user_id = Some(val.to_string()),
                    "DedeUserID__ckMd5" => login_info.dede_user_id_ck_md5 = Some(val.to_string()),
                    "SESSDATA" => login_info.sess_data = Some(val.to_string()),
                    "bili_jct" => login_info.bili_jct = Some(val.to_string()),
                    "sid" => login_info.sid = Some(val.to_string()),
                    _ => {}
                }
            }
        });
        Ok(login_info)
    }
}

impl ReqwestClient {
    pub fn new<C: CookieStore + 'static>(cookie_store: Arc<C>) -> Self {
        let mut default_hreaders = http::HeaderMap::new();
        default_hreaders.insert(http::header::USER_AGENT, AGENT.parse().unwrap());
        let mut client = reqwest::Client::builder().default_headers(default_hreaders);
        client = client.cookie_provider(cookie_store.clone());
        let cookie_store = cookie_store as Arc<dyn CookieStore>;
        let client = client.build().unwrap();
        ReqwestClient {
            client,
            cookie_store,
        }
    }

    pub fn get_login_info(&self) -> LoginInfo {
        static BILIBILI_URL: OnceLock<Url> = OnceLock::new();
        let url = BILIBILI_URL
            .get_or_init(|| Url::parse("https://bilibili.com").expect("invalid bilibli url"));
        let Some(value) = self.cookie_store.cookies(url) else {
            return LoginInfo::default();
        };
        String::from_utf8_lossy(value.as_bytes()).parse::<LoginInfo>().expect("Infallible")
    }

    pub async fn send_json<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, ClientError> {
        use ClientError::*;
        let resp = self
            .client
            .request(A::METHOD, A::url(request).to_string())
            .json(&request)
            .send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }

    pub async fn send_form<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, ClientError> {
        use ClientError::*;
        let resp = self
            .client
            .request(A::METHOD, A::url(request).to_string())
            .form(&request)
            .send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }

    pub async fn send_query<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, ClientError> {
        use ClientError::*;
        let resp = self
            .client
            .request(A::METHOD, A::url(request).to_string())
            .send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }

    pub async fn get_live_info(
        &self,
        uid: u64,
    ) -> Result<CommonResp<UserInfoResponse>, ClientError> {
        let request = UserInfoRequest { mid: uid };
        self.send_form::<UserInfo>(&request).await
    }

    pub async fn get_user_info_list(
        &self,
        uids: Vec<u64>,
    ) -> Result<CommonResp<Vec<UserInfoResponse>>, ClientError> {
        let request = UserCardsRequest { uids };
        self.send_query::<UserCards>(&request).await
    }

    pub async fn get_dynamic_by_topic(
        &self,
        topic_name: String,
        offset_dynamic_id: u64,
    ) -> Result<CommonResp<DynamicTopicResponse>, ClientError> {
        let request = DynamicTopicRequest {
            topic_name,
            offset_dynamic_id,
        };
        self.send_query::<DynamicTopic>(&request).await
    }
}
