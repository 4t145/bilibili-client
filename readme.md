# BILIBILI-CLIENT
在rust中使用blibili api，using bilibili api in rust

本项目还在快速迭代中, this project is still working in progress
## 使用 USAGE
在`cargo.toml`加入
```toml
[dependencies.bilibili-client]
git = "https://github.com/4t145/bilibili-client"
branch = "master"

[dependencies.tokio]
version = "^1.19"
features = ["full"]
```

在 `main.rs` 中
```rust
#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let config = ClientConfig { 
        // 你也可以自己实现StdoutLogger之外的logger
        logger: StdoutLogger::new(), 
        cookie_file: Some(Path::new("./cookies.json"))
    };
    let mut client = Client::new(config)?;
    // 如果设置cookie_file，客户端创建完成后可能就处于登录状态
    while !client.is_online() {
        client.login().await?;
    }
    client.send_danmaku_to_live(851181, LiveDanmaku::text("关注851181")).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    client.send_danmaku_to_live(851181, LiveDanmaku::text("关注梦魇tsuki谢谢喵")).await?;
    Ok(())
}
```

# 目前支持
## 登录
### 二维码 QRCODE
## 直播
### 发送弹幕
