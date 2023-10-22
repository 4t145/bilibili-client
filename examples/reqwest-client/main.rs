#![feature(async_fn_in_trait)]
use std::{path::PathBuf, sync::Arc};

use bilibili_client::{business::login::LoginByQrCode, reqwest_client::*, api::live::room_play_url::{RoomPlayUrlRequest, StreamPlatform, StreamQuality}};
use qrcode::{render::svg::Color, QrCode};
use reqwest::cookie;
use tokio::{fs, io::AsyncWriteExt};

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
        let abs_path = fs::canonicalize(&self.file)
            .await
            .expect("can't get absolute file");
        if let Ok(mut child) = tokio::process::Command::new("cmd.exe")
            .arg(format!(
                "/c start microsoft-edge:file:///{path}",
                path = abs_path.to_string_lossy()
            ))
            .spawn()
        {
            child.wait().await.unwrap_or_default();
        }
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
    let config_file = std::env::var("BILIBILI_COOKIE").unwrap_or(String::from("cookie.toml"));
    let jar = Arc::new(cookie::Jar::default());
    let client = Client::new(jar.clone());
    if let Ok(login_info) = fs::read_to_string(&config_file).await {
        let login_info = toml::from_str(&login_info).expect("cannot parse cookie");
        client.set_login_info(&login_info);
    } else {
        let loginer = FileLogin::new("qr.svg");
        let login = client
            .qr_login(loginer)
            .await
            .expect("fail to login");
        println!("login: {:?}", login);
        let login_info = client.get_login_info_from_cookie();
        let mut cookie_file = fs::File::create(config_file).await.expect("fail to save");
        cookie_file.write_all(toml::to_string(&login_info).expect("cant save cookie as toml file").as_bytes()).await.expect("fail to write cookie to file");

    }
    let cookie = client.get_login_info_from_cookie();
    println!("cookie: {:?}", cookie);
    let request = RoomPlayUrlRequest::new(851181).platform(StreamPlatform::H5).qn(StreamQuality::BlueLight);
    let play_url = client.get_room_play_url(&request).await.unwrap();
    dbg!(&play_url);
    let room_play_info = client.get_room_play_info(851181).await.unwrap();
    let danmu_info = client.get_danmu_info(room_play_info.room_id).await.unwrap();
    dbg!(&danmu_info);
}
