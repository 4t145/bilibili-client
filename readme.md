# BILIBILI-CLIENT
在rust中使用blibili api，using bilibili api in rust

本项目还在快速迭代中, this project is still working in progress
## 使用 USAGE
## 使用nightly版本(将会在rust 1.75版本后稳定)
```bash
rustup override set nightly
```
### 例程
在`cargo.toml`加入
```toml
[dependencies.bilibili-client]
git = "https://github.com/4t145/bilibili-client"
branch = "master"
```

在 `main.rs` 中
```rust
#![feature(async_fn_in_trait)]

#[tokio::main]
async fn main() {
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
}

```
[这个例程的完整版](./examples/reqwest-client/main.rs)