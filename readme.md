# BILIBILI-CLIENT
在rust中使用blibili api，using bilibili api in rust

本项目还在快速迭代中, this project is still working in progress
## 使用 USAGE
### 例程
在`cargo.toml`加入
```toml
[dependencies.bilibili-client]
git = "https://github.com/4t145/bilibili-client"
branch = "master"
```

在 `main.rs` 中
```rust
#[tokio::main]
async fn main() -> Result<(), ClientError>{
    let config = ClientConfig { 
        cookie_file: Some(Path::new("./examples/cookies.json"))
    };
    let client = Client::new(config)?;

    if !client.is_online() {
        let mut test = client.excute(Login{});
        loop {
            test.state.changed().await.unwrap();
            let s = &*test.state.borrow();
            match s {
                LoginState::ScaningQrcode(qrcode) => {
                    let qrcode = QrCode::new(qrcode.as_bytes()).unwrap().render::<Dense1x2>().build();
                    println!("{qrcode}")
                },
                LoginState::QrcodeScanFinished => {
                    println!("已扫码");
                },
                LoginState::Success { url } => {
                    println!("已登录：{url}");
                    client.save_cookies_to_file()?;
                    break;
                }
                _ => {},
            }
        }
    } else {
        client.excute(SendDanmakuToLive { 
            roomid: 5461071, 
            danmaku: danmaku!("黑楼黑旗黑暗剑") 
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let task = client.excute(SendDanmakuToLive { 
            roomid: 5461071, 
            danmaku: danmaku!(official=>147)
        });
        task.handle.await.unwrap()?;
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; 
}
```
[这个例程的完整版](./examples//send-danmaku/main.rs)
### 从API使用
如果仅仅是想使用api，你可以引入api模块下的实现了 `Api` trait 的结构，与其对应的请求与相应，之后可以交由`Client`结构的`fd_req` / `json_req` / `urlencoded_req` 三个方法。

### 从Transaction使用
如果想使用封装过的api，你可以使用事务（Transaction），引入从transaction使用模块下的实现了 `Transaction` trait 的结构，与其对应的请求与相应，之后你可以交由`Client`结构的`excute`来完成。
```rust

let mut task = client.excute(Login{});
loop {
    // 等待任务的状态改变
    task.state.changed().await.unwrap();
    let s = &*task.state.borrow();
    // 处理根据状态做出相应的处理
    match s {
        LoginState::ScaningQrcode(qrcode) => {
            // 打印二维码
            let qrcode = QrCode::new(qrcode.as_bytes()).unwrap().render::<Dense1x2>().build();
            println!("{qrcode}")
        },
        LoginState::QrcodeScanFinished => {
            println!("已扫码");
        },
        LoginState::Success { url } => {
            println!("已登录：{url}");
            // 保存cookie到文件里
            client.save_cookies_to_file()?;
            break;
        }
        _ => {},
    }
}
```

# 目前支持
## 登录
### 二维码 QRCODE
## 直播
### 发送弹幕
