use http_api_util::Api;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::str::FromStr;

use crate::api::CommonResp;

use super::info::UserInfoResponse;

pub struct UserCards;

impl Api for UserCards {
    type Request = UserCardsRequest;

    type Response = CommonResp<Vec<UserInfoResponse>>;

    const METHOD: http::Method = http::Method::GET;

    const CONST_URL: Option<&'static str> = None;

    fn dynamic_url(request: &Self::Request) -> http::Uri {
        let mut iter = request.uids.iter();
        let mut uids = String::new();
        if let Some(first) = iter.next() {
            uids.push_str(&first.to_string());
            for rest in iter {
                uids.push(',');
                uids.push_str(&rest.to_string());
            }
        }

        http::Uri::from_str(&format!(
            "https://api.vc.bilibili.com/account/v1/user/cards?uids={uids}",
        ))
        .unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct UserCardsRequest {
    #[serde(skip)]
    pub uids: Vec<u64>,
}
