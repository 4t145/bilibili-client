use serde::Serialize;

use crate::api::CommonResp;
use crate::api::Request;
use crate::api::RequestParts;
use crate::api::api_vc_url;
use crate::reqwest_client::ClientError;
use crate::reqwest_client::ReqwestClient;

pub struct UserCards;

impl<'r> Request<'r> for UserCardsRequest<'r> {
    type Body = ();
    type ContentType = ();
    type Query = UserCardsRequestQuery;
    type Response = CommonResp<()>;

    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "account/v1/user/cards";

    fn parts(&'r self) -> RequestParts<'r, Self::Query, Self::Body> {
        let mut uids = String::new();
        self.uids.iter().for_each(|x| {
            uids.push_str(&x.to_string());
            uids.push(',');
        });
        uids.pop();
        RequestParts {
            query: UserCardsRequestQuery { uids },
            ..Default::default()
        }
    }
}

impl ReqwestClient {
    pub async fn user_cards<'r>(&self, uids: &'r [u64]) -> Result<(), ClientError> {
        self.send(&UserCardsRequest { uids }, api_vc_url()).await?.into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserCardsRequest<'r> {
    pub uids: &'r [u64],
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UserCardsRequestQuery {
    pub uids: String,
}
