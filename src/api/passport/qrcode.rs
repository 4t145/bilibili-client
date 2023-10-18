use crate::{
    api::{content_type::Form, passport_url, Request, RequestParts},
    reqwest_client::{ClientResult, ReqwestClient},
};

impl ReqwestClient {
    pub async fn get_login_url(&self) -> ClientResult<GetLoginUrlResp> {
        self.send(&GetLoginUrlRequest, passport_url()).await
    }
    pub async fn get_login_info(&self, oauth_key: &str) -> ClientResult<GetLoginInfoResp> {
        self.send(&GetLoginInfoRequest { oauth_key }, passport_url()).await
    }
}


#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginUrlResp {
    pub code: i64,
    pub status: bool,
    pub data: GetLoginUrlRespData,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginUrlRespData {
    pub url: String,
    pub oauth_key: String,
}

pub struct GetLoginUrlRequest;
impl Request<'static> for GetLoginUrlRequest {
    type Body = ();

    type Query = ();

    type ContentType = ();

    type Response = GetLoginUrlResp;

    const METHOD: http::Method = http::Method::GET;

    const PATH: &'static str = "qrcode/getLoginUrl";

    fn parts(&'static self) -> crate::api::RequestParts<'static, Self::Query, Self::Body> {
        Default::default()
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginInfoRequest<'r> {
    pub(crate) oauth_key: &'r str,
}

impl<'r> Request<'r> for GetLoginInfoRequest<'r> {
    type Body = &'r Self;

    type Query = ();

    type ContentType = Form;

    type Response = GetLoginInfoResp;

    const METHOD: http::Method = http::Method::POST;

    const PATH: &'static str = "qrcode/getLoginInfo";

    fn parts(&'r self) -> crate::api::RequestParts<'r, Self::Query, Self::Body> {
        RequestParts::body_from_request(self)
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginInfoResp {
    pub message: Option<String>,
    pub data: GetLoginInfoRespData,
}

#[derive(serde::Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum GetLoginInfoRespData {
    Code(i8),
    Body { url: String },
}
