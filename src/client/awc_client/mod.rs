use awc::{
    self,
    error::{JsonPayloadError, SendRequestError},
};
use expire::MaybeExpired;
use http_api_util::{
    cache::{ApiCache, FifoCache},
    Api,
};

use std::{hash::Hash, sync::RwLock, time};

mod api_cache;

use crate::{
    api::{
        user::info::{UserInfo, UserInfoRequest, UserInfoResponse},
        CommonResp,
    },
    consts::AGENT,
};

pub struct AwcClient {
    client: awc::Client,
}

#[derive(Debug)]
pub enum AwcClientError {
    SerJson(serde_json::Error),
    Json(JsonPayloadError),
    Send(SendRequestError),
}

impl Default for AwcClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AwcClient {
    pub fn new() -> Self {
        let client = awc::Client::builder()
            .add_default_header((http::header::USER_AGENT, AGENT))
            .finish();
        AwcClient { client }
    }

    pub async fn send_json<A: Api>(
        &self,
        request: &A::Request,
    ) -> Result<A::Response, AwcClientError> {
        use AwcClientError::*;
        let mut resp = self
            .client
            .request(A::METHOD, A::url(request))
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
            .request(A::METHOD, A::url(request))
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
        {
            let cache = rwl_cache.read().unwrap();
            if let Some(maybe_expired) = cache.get(request) {
                if let Some(response) = maybe_expired.get() {
                    return Ok(response.clone());
                }
            }
        }
        let mut cache = rwl_cache.write().unwrap();
        let response = self.send_json::<A>(request).await?;
        let mut maybe_expired = MaybeExpired::new();
        maybe_expired.set(response.clone(), expire);
        cache.put(request.clone(), maybe_expired);
        Ok(response)
    }

    pub async fn get_room_info_cached(
        &self,
        uid: u64,
        rwl_cache: &RwLock<FifoCache<UserInfoRequest, MaybeExpired<CommonResp<UserInfoResponse>>>>,
        expire: Option<time::Duration>,
    ) -> Result<CommonResp<UserInfoResponse>, AwcClientError> {
        const EXPIRE: time::Duration = time::Duration::from_secs(1);
        let request = UserInfoRequest { mid: uid };
        self.send_form_cached::<UserInfo>(&request, rwl_cache, expire.unwrap_or(EXPIRE))
            .await
    }

    pub async fn get_live_info(
        &self,
        uid: u64,
    ) -> Result<CommonResp<UserInfoResponse>, AwcClientError> {
        let request = UserInfoRequest { mid: uid };
        self.send_form::<UserInfo>(&request).await
    }
}
