use reqwest::Method;
use crate::api::Api;

pub struct GetLoginUrl;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginUrlResp {
    pub code: i64,
    pub status: bool,
    pub data: GetLoginUrlRespData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginUrlRespData {
    pub url: String,
    pub oauth_key: String
}

impl Api for GetLoginUrl {
    type Request = ();
    type Response = GetLoginUrlResp;
    const METHOD: Method = Method::GET;
    const URL: &'static str = "https://passport.bilibili.com/qrcode/getLoginUrl";

}



pub struct GetLoginInfo;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginInfoReq {
    pub(crate) oauth_key: String
}


#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoginInfoResp {
    pub message: Option<String>,
    pub data: GetLoginInfoRespData
}

#[derive(serde::Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum GetLoginInfoRespData {
    Code(i8),
    Body {
        url: String
    }
}

impl Api for GetLoginInfo {
    type Request = GetLoginInfoReq;
    type Response = GetLoginInfoResp;
    const METHOD: Method = Method::POST;
    const URL: &'static str = "https://passport.bilibili.com/qrcode/getLoginInfo";
}