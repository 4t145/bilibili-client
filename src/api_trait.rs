use reqwest::{Request, Error as HttpError, multipart::Form};


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
                    form = form.text(key.clone(), val.to_string());
                }
            }
        }
        client.request(Self::METHOD, Self::URL).multipart(form).build().map_err(ApiError::ReqForm)
    }

    fn json_req<A: Api>(client:&reqwest::Client, req: A::Request) -> Result<Request, ApiError> {
        client.request(A::METHOD, A::URL).json(&req).build().map_err(ApiError::ReqForm)
    }
}