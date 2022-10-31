use reqwest::{
    self,
    Error
};
use expire::MaybeExpired;
use http_api_util::{
    cache::{ApiCache, FifoCache},
    Api,
};

use std::{
    hash::Hash,
    sync::{RwLock},
    time,
};



use crate::{api::{
    user::info::{UserInfo, UserInfoRequest, UserInfoResponse},
    CommonResp,
}, consts::AGENT};

pub struct ReqwestClient {
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum AwcClientError {
    SerJson(serde_json::Error),
    Reqwest(Error),
}

impl ReqwestClient {
    pub fn new() -> Self {
        let mut default_hreaders = http::HeaderMap::new();
        default_hreaders.insert(http::header::USER_AGENT, AGENT.parse().unwrap());
        let client = reqwest::Client::builder().default_headers(default_hreaders).build().unwrap();
        return ReqwestClient {
            client
        };
    }

    pub async fn send_json<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, AwcClientError> {
        use AwcClientError::*;
        let resp = self.client
            .request(A::METHOD, A::url(&request).to_string()).json(&request).send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }

    pub async fn send_form<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, AwcClientError> {
        use AwcClientError::*;
        let resp = self.client
            .request(A::METHOD, A::url(&request).to_string())
            .form(&request)
            .send()
            .await
            .map_err(Reqwest)?;
        resp.json::<A::Response>().await.map_err(Reqwest)
    }

    pub async fn send_form_cached<A: Api>(
        &self,
        request: &A::Request,
        rwl_cache: &RwLock<FifoCache<A::Request, MaybeExpired<A::Response>>>,
        expire: time::Duration,
    ) -> Result<A::Response, AwcClientError>
    where
        A::Request: Hash + Eq + Clone,
        A::Response: Clone,
    {
        const EXPIRE: time::Duration = time::Duration::from_secs(1);
        {
            let cache = rwl_cache.read().unwrap();
            if let Some(maybe_expired) = cache.get(request) {
                if let Some(response) = maybe_expired.get() {
                    return Ok(response.clone());
                }
            }
            // here cache dropped
        }
        dbg!("缓存过期");
        let mut cache = rwl_cache.write().unwrap();
        let response = self.send_json::<A>(&request).await?;
        let mut maybe_expired = MaybeExpired::new();
        maybe_expired.set(response.clone(), expire);
        cache.put(request.clone(), maybe_expired);
        return Ok(response);
    }

    pub async fn get_room_info_cached(
        &self,
        uid: u64,
        rwl_cache: &RwLock<FifoCache<UserInfoRequest, MaybeExpired<CommonResp<UserInfoResponse>>>>,
    ) -> Result<CommonResp<UserInfoResponse>, AwcClientError> {
        let request = UserInfoRequest { mid: uid };
        const EXPIRE: time::Duration = time::Duration::from_secs(1);
        self.send_form_cached::<UserInfo>(&request, rwl_cache, EXPIRE).await
    }

    pub async fn get_live_info(&self, uid: u64) -> Result<CommonResp<UserInfoResponse>, AwcClientError> {
        let request = UserInfoRequest { mid: uid };
        self.send_form::<UserInfo>(&request).await
    }
}