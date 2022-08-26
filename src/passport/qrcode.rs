pub(crate) const BASE_URL: &'static str = "https://passport.bilibili.com/qrcode";
use reqwest::{Request, Url, RequestBuilder, Method, Body};
use crate::api_trait::JsonApi;

pub struct GetLoginUrl;

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLoginUrlResp {
    pub code: i64,
    pub status: bool,
    pub data: GetLoginUrlRespData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLoginUrlRespData {
    pub url: String,
    pub oauth_key: String
}

impl JsonApi for GetLoginUrl {
    type Request = ();
    type Response = GetLoginUrlResp;
    const METHOD: Method = Method::GET;
    const URL: &'static str = "https://passport.bilibili.com/qrcode/getLoginUrl";

}



pub struct GetLoginInfo;

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLoginInfoReq {
    pub oauth_key: String
}


#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetLoginInfoResp {
    pub data: GetLoginInfoRespData
}

#[derive(serde::Deserialize)]
#[serde(untagged, rename_all = "PascalCase")]
pub enum GetLoginInfoRespData {
    Code(i8),
    Body {
        url: String
    }
}
impl JsonApi for GetLoginInfo {
    type Request = GetLoginInfoReq;
    type Response = GetLoginInfoResp;
    const METHOD: Method = Method::POST;
    const URL: &'static str = "https://passport.bilibili.com/qrcode/getLoginInfo";
}