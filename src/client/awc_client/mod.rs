use awc::{
    self,
    error::{JsonPayloadError, SendRequestError},
};
use expire::MaybeExpired;
use http_api_util::{
    cache::{ApiCache, FifoCache},
    Api,
};

use std::{
    hash::Hash,
    sync::RwLock,
    time,
};

mod api_cache;
use api_cache::Cache;

use crate::{api::{
    user::info::{UserInfo, UserInfoRequest, UserInfoResponse},
    CommonResp,
}, consts::AGENT};

pub struct AwcClient {
    client: awc::Client,
    cache: Cache,
}

#[derive(Debug)]
pub enum AwcClientError {
    SerJson(serde_json::Error),
    Json(JsonPayloadError),
    Send(SendRequestError),
}

impl AwcClient {
    pub fn new() -> Self {
        let client = awc::Client::builder().add_default_header((http::header::USER_AGENT, AGENT)).finish();
        return AwcClient {
            client,
            cache: Cache {
                user_info: RwLock::new(FifoCache::new(128)),
            },
        };
    }

    pub async fn send_json<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, AwcClientError> {
        use AwcClientError::*;
        let mut resp = self
            .client
            .request(A::METHOD, A::url(&request))
            .send_json(&request)
            .await
            .map_err(Send)?;
        resp.json::<A::Response>().await.map_err(Json)
    }

    pub async fn send_form<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, AwcClientError> {
        use AwcClientError::*;
        let mut resp = self
            .client
            .request(A::METHOD, A::url(&request))
            .send_form(&request)
            .await
            .map_err(Send)?;
        resp.json::<A::Response>().await.map_err(Json)
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
    ) -> Result<CommonResp<UserInfoResponse>, AwcClientError> {
        let request = UserInfoRequest { mid: uid };
        const EXPIRE: time::Duration = time::Duration::from_secs(1);
        self.send_form_cached::<UserInfo>(&request, &self.cache.user_info, EXPIRE).await
    }

    pub async fn get_live_info(&self, uid: u64) -> Result<CommonResp<UserInfoResponse>, AwcClientError> {
        let request = UserInfoRequest { mid: uid };
        self.send_form::<UserInfo>(&request).await

    }
}
