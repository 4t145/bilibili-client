use std::{path::Path};
use bilibili_client::{
    danmaku, 
    Client,
    ClientConfig,
    ClientError,
    transaction::{
        send_danmaku_to_live::*,
        login::*
    }
};
use qrcode::{QrCode, render::unicode::Dense1x2};

#[tokio::main]
async fn main() -> Result<(), ClientError>{
    let config = ClientConfig { 
        cookie_file: Some(Path::new("./examples/cookies.json"))
    };
    let client = Client::new(config)?;

    
    loop {
        if !client.is_online() {
            let mut task = client.excute(Login{});
            loop {
                task.state.changed().await.unwrap();
                let s = &*task.state.borrow();
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
            // let d1 = danmaku!("黑楼黑旗黑暗剑");
            // let d2 = danmaku!(22470216=>765);
            // let d3 = danmaku!(official=>147);
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            let task = client.excute(SendDanmakuToLive { 
                roomid: 5461071, 
                danmaku: danmaku!(official=>147)
            });
            task.handle.await.unwrap()?;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
