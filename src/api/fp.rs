// https://api.bilibili.com/x/frontend/finger/spi

use serde::Deserialize;

use crate::reqwest_client::{Client, ClientResult};

use super::{Request, RequestParts, api_url, CommonResp};

#[derive(Debug, Clone, Copy)]
pub struct FingerSpiRequest;

#[derive(Debug, Clone, Deserialize)]
pub struct FingerSpiResponse {
    pub b_3: String,
    pub b_4: String,
}

impl Request<'static> for FingerSpiRequest {
    type Body = ();

    type Query = ();

    type ContentType = ();

    type Response = CommonResp<FingerSpiResponse>;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "x/frontend/finger/spi";

    fn parts(&'static self) -> RequestParts<'static, Self::Query, Self::Body> {
        RequestParts::new((), ())
    }
}

impl Client {
    pub async fn get_finger(&self) -> ClientResult<FingerSpiResponse> {
        self.send(&FingerSpiRequest, api_url()).await?.into()
    }
}