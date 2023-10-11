use reqwest::{
    self,
    Error, cookie::{CookieStore, self}, Url
};

use expire::MaybeExpired;
use http_api_util::{
    cache::{ApiCache, FifoCache},
    Api,
};


use std::{
    hash::Hash,
    sync::{RwLock, Arc, OnceLock},
    time,
};



use crate::{api::{
    user::{info::{UserInfo, UserInfoRequest, UserInfoResponse}, cards::{UserCards, UserCardsRequest}},
    CommonResp, dynamic::topic::{DynamicTopicRequest, DynamicTopic, DynamicTopicResponse},
}, consts::AGENT};

pub type FifoRwlCache<A> = RwLock<FifoCache<<A as Api>::Request, MaybeExpired<Arc<<A as Api>::Response>>>>;

pub struct ReqwestClient {
    client: reqwest::Client,
    cookie_store: Arc<dyn CookieStore + 'static>,
}

#[derive(Debug)]
pub enum ClientError {
    SerJson(serde_json::Error),
    Reqwest(Error),
    Offline,
    Fail {
        code: i32,
        message: String,
    },
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

impl ReqwestClient {
    pub fn new(cookie_store: Option<Arc<impl CookieStore + 'static>>) -> Self {
        let mut default_hreaders = http::HeaderMap::new();
        default_hreaders.insert(http::header::USER_AGENT, AGENT.parse().unwrap());
        let mut client = reqwest::Client::builder()
        .default_headers(default_hreaders);
        let cookie_store = if let Some(cookie_store) = cookie_store {
            client = client.cookie_provider(cookie_store.clone());
            cookie_store as Arc<dyn CookieStore>
        } else {
            let cookie_store = Arc::new(cookie::Jar::default());
            client = client.cookie_provider(cookie_store.clone());
            cookie_store as Arc<dyn CookieStore>
        };
        let client = client.build().unwrap();
        ReqwestClient {
            client,
            cookie_store
        }
    }
    
    pub fn get_jct(&self) -> Option<&str> {
        static BILIBILI_URL: OnceLock<Url> = OnceLock::new();
        let url = BILIBILI_URL.get_or_init(||Url::parse("bilibili.com").expect("invalid bilibli url"));
        todo!();
        // self.cookie_store.cookies(url)
        Some("")
    }

    pub async fn send_json<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, ClientError> {
        use ClientError::*;
        let resp = self.client
            .request(A::METHOD, A::url(request).to_string()).json(&request).send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }

    pub async fn send_form<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, ClientError> {
        use ClientError::*;
        let resp = self.client
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
        let resp = self.client
            .request(A::METHOD, A::url(request).to_string())
            .send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }
    
    pub async fn get_live_info(&self, uid: u64) -> Result<CommonResp<UserInfoResponse>, ClientError> {
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

    pub async fn get_dynamic_by_topic(&self, topic_name: String, offset_dynamic_id: u64) -> Result<CommonResp<DynamicTopicResponse>, ClientError> {
        let request = DynamicTopicRequest {topic_name, offset_dynamic_id};
        self.send_query::<DynamicTopic>(&request).await
    }

}
