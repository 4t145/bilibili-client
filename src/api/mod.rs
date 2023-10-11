pub mod dynamic;
pub mod live;
pub mod passport;
pub mod user;
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct CommonResp<T> {
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> From<CommonResp<T>> for Result<T, ClientError> {
    fn from(resp: CommonResp<T>) -> Self {
        if let Some(data) = resp.data {
            debug_assert!(resp.code == 0);
            Ok(data)
        } else {
            Err(ClientError::Fail {
                code: resp.code,
                message: resp.message.unwrap_or_default(),
            })
        }
    }
}
// use reqwest::{Error as HttpError};

pub use http_api_util::Api;

use crate::reqwest_client::ClientError;
use std::collections::HashMap;

use http::HeaderMap;
use reqwest::RequestBuilder;
use serde::Serialize;

#[derive(Default)]
pub struct RequestParts<'a, Q, B> {
    pub path: HashMap<&'static str, &'a dyn ToString>,
    pub headers: HeaderMap,
    pub query: Q,
    pub body: B,
}

pub trait ContentType {
    fn set_body<B: Serialize>(body: B, builder: RequestBuilder) -> RequestBuilder;
}

pub mod content_type {
    use super::ContentType;
    use reqwest::RequestBuilder;
    use serde::Serialize;

    impl ContentType for () {
        fn set_body<B: Serialize>(_body: B, builder: RequestBuilder) -> RequestBuilder {
            builder
        }
    }
    pub struct Json;

    impl ContentType for Json {
        fn set_body<B: Serialize>(body: B, builder: RequestBuilder) -> RequestBuilder {
            builder.json(&body)
        }
    }

    pub struct Form;

    impl ContentType for Form {
        fn set_body<B: Serialize>(body: B, builder: RequestBuilder) -> RequestBuilder {
            builder.form(&body)
        }
    }
}

pub trait Request<'r> {
    type Body: Serialize + 'r;
    type Query: Serialize + 'r;
    type ContentType: ContentType;
    type Response: for<'de> Deserialize<'de>;

    const METHOD: http::Method;
    const PATH: &'static str;

    fn parts(&'r self) -> RequestParts<'r, Self::Query, Self::Body>;

    fn custom_modify(&self, builder: RequestBuilder) -> RequestBuilder {
        builder
    }

    fn build_request(
        &'r self,
        client: &reqwest::Client,
        base: &str,
    ) -> reqwest::Result<reqwest::Request> {
        let RequestParts {
            query,
            path,
            body,
            headers,
        } = self.parts();
        let path_pat = Self::PATH;
        let path = path_pat
            .split('/')
            .map(|x| {
                if let Some(key) = x.strip_prefix(':') {
                    path.get(key).expect("miss key").to_string()
                } else {
                    x.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("/");
        let url = format!("{}/{}", base.trim_end_matches('/'), path);
        let mut builder = client.request(Self::METHOD, url);
        builder = builder.query(&query).headers(headers);
        builder = Self::ContentType::set_body(body, builder);
        self.custom_modify(builder).build()
    }
}

pub async fn send<'r, R: Request<'r>>(
    client: &reqwest::Client,
    base: &str,
    req: &'r R,
) -> reqwest::Result<R::Response> {
    let req = req.build_request(client, base)?;
    let resp = client.execute(req).await?;
    resp.json().await
}
