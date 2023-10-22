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

use crate::reqwest_client::{Client, ClientError};
use std::collections::HashMap;

use http::{header::IntoHeaderName, HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Url};
use serde::Serialize;

#[derive(Default)]
pub struct RequestParts<'a, Q, B> {
    pub path: HashMap<&'static str, &'a dyn ToString>,
    pub headers: HeaderMap,
    pub query: Q,
    pub body: B,
}

impl<'a, Q, B> RequestParts<'a, Q, B> {
    pub fn new(query: Q, body: B) -> Self {
        Self {
            path: HashMap::new(),
            headers: HeaderMap::new(),
            query,
            body,
        }
    }
    pub fn with_path<V: ToString>(mut self, key: &'static str, value: &'a V) -> Self {
        self.path.insert(key, value);
        self
    }
    pub fn with_header<K: IntoHeaderName>(mut self, key: K, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    pub fn with_query(mut self, query: Q) -> Self {
        self.query = query;
        self
    }
    pub fn with_body(mut self, body: B) -> Self {
        self.body = body;
        self
    }
}

impl<'r, R: Request<'r>> RequestParts<'r, &'r R, ()> {
    pub fn query_from_request(request: &'r R) -> Self {
        Self {
            path: HashMap::new(),
            headers: HeaderMap::new(),
            query: request,
            body: (),
        }
    }
}

impl<'r, R: Request<'r>> RequestParts<'r, (), &'r R> {
    pub fn body_from_request(request: &'r R) -> Self {
        Self {
            path: HashMap::new(),
            headers: HeaderMap::new(),
            query: (),
            body: request,
        }
    }
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
        base: &Url,
    ) -> reqwest::Result<reqwest::Request> {
        let RequestParts {
            query,
            path,
            body,
            headers,
        } = self.parts();
        let path_pat = Self::PATH;
        // #[todo]
        // this can be optimized
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
        let mut url = base.clone();
        url.set_path(&path);
        let mut builder = client.request(Self::METHOD, url);
        builder = builder.query(&query).headers(headers);
        builder = Self::ContentType::set_body(body, builder);
        self.custom_modify(builder).build()
    }
}

pub(crate) async fn send<'r, R: Request<'r>>(
    client: &reqwest::Client,
    base: &Url,
    req: &'r R,
) -> reqwest::Result<R::Response> {
    let req = req.build_request(client, base)?;
    let resp = client.execute(req).await?;
    resp.json().await
}

impl Client {
    pub async fn send<'r, R: crate::api::Request<'r>>(
        &self,
        req: &'r R,
        base: &Url,
    ) -> Result<R::Response, ClientError> {
        let resp = crate::api::send(&self.client, base, req).await?;
        Ok(resp)
    }
}
macro_rules! static_url {
    {$id: ident: $url:expr} => {
        pub fn $id() -> &'static Url {
            use std::sync::OnceLock;
            static STATIC_URL: OnceLock<Url> = OnceLock::new();
            STATIC_URL.get_or_init(|| Url::parse($url).expect("invalid url"))
        }
    };
    {$($id: ident: $url:expr),*$(,)?} => {
        $(static_url!{$id: $url})*
    };
}

static_url! {
    passport_url:   "https://passport.bilibili.com/",
    api_vc_url:     "https://api.vc.bilibili.com/",
    api_url:        "https://api.bilibili.com/",
    api_live_url:   "https://api.live.bilibili.com/",
}
