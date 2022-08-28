pub mod live;
pub mod passport;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct CommonResp<T> {
    pub code: i32,
    pub message: Option<String>,
    pub data: T
}

use reqwest::{Request, Error as HttpError, multipart::Form};

#[derive(Debug)]
pub enum ApiError {
    Deser(HttpError),
    Http(HttpError),
    ReqForm(HttpError)
}

pub trait Api {
    type Request: serde::Serialize;
    type Response: for<'de> serde::Deserialize<'de>;
    const METHOD: reqwest::Method;
    const URL: &'static str;

    /// !!! **Reqbody will be EMPTY while req is not an object** !!!
    fn form_data_req(client:&reqwest::Client, req: Self::Request) -> Result<Request, ApiError> {
        let json = serde_json::json!(req);
        let mut form = Form::new();
        if let Some(obj) = json.as_object() {
            for (key, val) in obj {
                if !val.is_null() {
                    if val.is_string() {
                        form = form.text(key.clone(), val.to_string());
                    } else {
                        form = form.text(key.clone(), val.to_string());
                    }
                }
            }
        }
        client.request(Self::METHOD, Self::URL).multipart(form).build().map_err(ApiError::ReqForm)
    }

    fn json_req(client:&reqwest::Client, req: Self::Request) -> Result<Request, ApiError> {
        client.request(Self::METHOD, Self::URL).json(&req).build().map_err(ApiError::ReqForm)
    }

    fn urlencoded_req(client:&reqwest::Client, req: Self::Request) -> Result<Request, ApiError> {
        client.request(Self::METHOD, Self::URL).form(&req).build().map_err(ApiError::ReqForm)
    }
}