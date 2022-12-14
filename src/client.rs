use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::consts::*;
use crate::api::{Api, ApiError};
use crate::transaction::{Transaction, Task};
use reqwest_cookie_store::CookieStoreRwLock;

pub struct Client {
    http_client: reqwest::Client,
    cookie_store: Arc<CookieStoreRwLock>,
    cookie_file_path: Option<PathBuf>,
    // pub loggers: Vec<Box<dyn Logger>>
}

pub struct ClientConfig<'p> {
    pub cookie_file: Option<&'p Path>
}

#[derive(Debug)]
pub enum ClientError {
    Api(ApiError),
    Fs(std::io::Error),
    CookieStore(cookie_store::Error),
    Offline,
    NoCookieFile
}

impl Client {
    pub fn new(config: ClientConfig) -> Result<Arc<Self>, ClientError> {
        use ClientError::*;
        use std::io::{BufReader};
        use std::fs::OpenOptions;
        let cookie_store = match config.cookie_file {
            Some(path) => {
                let reader = OpenOptions::new().create(true).write(true).read(true).open(&path).map(BufReader::new).map_err(Fs)?;
                let store = cookie_store::CookieStore::load_json(reader).map_err(CookieStore)?;
                store
            },
            None => {
                cookie_store::CookieStore::default()
            },
        };

        let cookie_store = reqwest_cookie_store::CookieStoreRwLock::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        let client = Self {
            http_client: reqwest::Client::builder()
            .cookie_provider(cookie_store.clone())
            .user_agent(AGENT)
            .build()
            .unwrap(),
            cookie_store: cookie_store,
            cookie_file_path: config.cookie_file.map(|x|x.to_path_buf()),
        };
        Ok(Arc::new(client))
    }


    pub fn save_cookies_to_file(&self) -> Result<(), ClientError> {
        use std::io::Write;
        use std::io::BufWriter;
        use std::fs::OpenOptions;
        use ClientError::*;
        if let Some(path) = &self.cookie_file_path {
            let mut writer = OpenOptions::new().write(true).open(path).map(BufWriter::new).map_err(Fs)?;
            {
                let _ = &self.cookie_store.read().unwrap().save_json(&mut writer).map_err(CookieStore)?;
            }
            writer.flush().map_err(Fs)?;
            return Ok(())
        }
        Err(ClientError::NoCookieFile)
    }

    pub fn get_cookie(&self, key: &str) -> Option<String> {
        let store = self.cookie_store.read().unwrap();
        let cookie = store.get("bilibili.com", "/", key);
        cookie.map(|c|c.value().to_string())
    }

    #[inline]
    pub fn get_cookie_jct(&self) -> Option<String> {
        self.get_cookie("bili_jct")
    }

    #[inline]
    pub fn is_online(&self) -> bool {
        self.get_cookie_jct().is_some()
    }

    /// ?????????????????????
    #[inline]
    pub async fn query_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        let request = A::query(&self.http_client, req)?;
        self.http_client.execute(request).await
            .map_err(ApiError::Http)?
            .json().await.map_err(ApiError::Deser)
    }

    /// ???`form data`????????????
    #[inline]
    pub async fn fd_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        let request = A::form_data_req(&self.http_client, req)?;
        self.http_client.execute(request).await
            .map_err(ApiError::Http)?
            .json().await.map_err(ApiError::Deser)
    }

    /// ???`json`????????????
    #[inline]
    pub async fn json_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        let request = A::json_req(&self.http_client, req)?;
        self.http_client.execute(request).await
            .map_err(ApiError::Http)?
            .json().await.map_err(ApiError::Deser)
    }

    /// ???`urlencoded`????????????
    #[inline]
    pub async fn urlencoded_req<A: Api>(&self, req: A::Request) -> Result<A::Response, ApiError> {
        let request = A::urlencoded_req(&self.http_client, req)?;
        let resp = self.http_client.execute(request).await.map_err(ApiError::Http)?;
        resp.json().await.map_err(ApiError::Deser)
    }
}

impl Client {
    pub fn excute<T:Transaction>(self: &Arc<Self>, transaction: T) -> Task<T> {
        transaction.excute_on(self.clone())
    }
}