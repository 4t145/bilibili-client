use http::HeaderValue;
use reqwest::{self, cookie::CookieStore, Error, Url};

use http_api_util::Api;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
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
    #[serde(flatten)]
    pub ext: HashMap<String, String>,
}

static BILIBILI_URL: OnceLock<Url> = OnceLock::new();
pub(crate) fn bilibili_url() -> &'static Url {
    BILIBILI_URL.get_or_init(|| Url::parse("https://bilibili.com").expect("invalid bilibli url"))
}

impl LoginInfo {
    pub fn is_login(&self) -> bool {
        self.dede_user_id.is_some()
            && self.dede_user_id_ck_md5.is_some()
            && self.sess_data.is_some()
            && self.bili_jct.is_some()
    }
    pub fn resgiter<C: CookieStore + ?Sized>(&self, cookie_store: &C) {
        let mut headers = vec![];
        fn item<'a>(key: &'a str) -> impl Fn(&'a str) -> HeaderValue {
            move |value| {
                HeaderValue::from_str(&format!("{}={}", key, value))
                    .expect("invalid header value in LoginInfo")
            }
        }
        headers.extend(self.dede_user_id.as_deref().map(item("DedeUserID")));
        headers.extend(self.sid.as_deref().map(item("sid")));
        headers.extend(
            self.dede_user_id_ck_md5
                .as_deref()
                .map(item("DedeUserID__ckMd5")),
        );
        headers.extend(self.sess_data.as_deref().map(item("SESSDATA")));
        headers.extend(self.bili_jct.as_deref().map(item("bili_jct")));
        headers.extend(self.ext.iter().map(|(k, v)| item(k)(v)));
        cookie_store.set_cookies(&mut headers.iter(), bilibili_url());
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

    pub fn set_login_info(&self, login_info: &LoginInfo) {
        login_info.resgiter(self.cookie_store.as_ref());
    }

    pub fn get_login_info(&self) -> LoginInfo {
        let url = bilibili_url();
        let mut login_info = LoginInfo::default();
        let Some(value) = self.cookie_store.cookies(url) else {
            return login_info;
        };
        for c in cookie::Cookie::split_parse(String::from_utf8_lossy(value.as_bytes())).flatten() {
            let (name, value) = c.name_value_trimmed();
            match name {
                "DedeUserID" => login_info.dede_user_id.replace(value.to_owned()),
                "DedeUserID__ckMd5" => login_info.dede_user_id_ck_md5.replace(value.to_owned()),
                "SESSDATA" => login_info.sess_data.replace(value.to_owned()),
                "bili_jct" => login_info.bili_jct.replace(value.to_owned()),
                "sid" => login_info.sid.replace(value.to_owned()),
                _ => login_info.ext.insert(name.to_string(), value.to_owned()),
            };
        }
        login_info
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

    pub async fn send<'r, R: crate::api::Request<'r>>(
        &self,
        req: &'r R,
        base: &Url,
    ) -> Result<R::Response, ClientError> {
        let resp = crate::api::send(&self.client, base, req).await?;
        Ok(resp)
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
