#![feature(async_fn_in_trait)]
use std::{path::PathBuf, sync::Arc};

use bilibili_client::{
    business::login::LoginByQrCode, model::dynamic::DynamicCard, reqwest_client::*,
};
use image::Luma;
use qrcode::{render::svg::Color, QrCode};
use reqwest::cookie;
use tokio::io::AsyncWriteExt;

pub struct FileLogin {
    file: PathBuf,
}

impl FileLogin {
    pub fn new<P>(file: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { file: file.into() }
    }
}
impl LoginByQrCode for FileLogin {
    async fn update_code(&mut self, url: &str) {
        let code = QrCode::new(url).unwrap();
        let image = code.render::<Color>().build();
        let mut file = tokio::fs::File::create(&self.file).await.unwrap();
        file.write_all(image.as_bytes()).await.unwrap();
    }

    async fn scanned(&mut self) {
        tokio::fs::remove_file(&self.file).await.unwrap_or_default();
    }

    async fn expired(&mut self) {
        tokio::fs::remove_file(&self.file).await.unwrap_or_default();
    }

    async fn next_poll(&mut self) {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let jar = Arc::new(cookie::Jar::default());
    let reqwest_client = ReqwestClient::new(jar.clone());
    let loginer = FileLogin::new("qr.svg");
    let login = reqwest_client.qr_login(loginer).await.expect("fail to login");
    println!("login: {:?}", login);
    // // 接下来一秒测试缓存
    // let result = reqwest_client.get_live_info(7706705).await;
    // dbg!(result);
}
