use std::future::{Future};
use reqwest::{Request, Url, RequestBuilder, Method, Body, Response};
pub trait TraitApi {
    const BASE_URL:&'static str;
    fn path(path: &[&'static str]) -> String {
        let size = path.iter().fold(0, |len, s| 1+len+s.len());
        let mut url = String::with_capacity(size + Self::BASE_URL.len());
        url.push_str(Self::BASE_URL);
        for s in path {
            url.push('/');
            url.push_str(s);
        }
        return url;
    }
}


pub trait JsonApi {
    type Request: serde::Serialize;
    const METHOD: reqwest::Method;
    const URL: &'static str;
    type Response: for<'de> serde::Deserialize<'de>;
}

fn make_request<Api: JsonApi>(req: &Api::Request) -> Request {
    let url = Url::parse(Api::URL).unwrap();
    let mut r = Request::new(Api::METHOD, url);
    let json = serde_json::to_vec(req).unwrap();
    *r.body_mut() = Some(Body::from(json));
    r
}

pub async fn fetch<Api: JsonApi>(client:&reqwest::Client, req: Api::Request) -> Api::Response {
    let resp = client.execute(make_request::<Api>(&req)).await;
    resp.unwrap().json::<Api::Response>().await.unwrap()
}