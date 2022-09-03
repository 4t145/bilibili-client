use std::{path::Path};
use bilibili_client::{
    danmaku, 
    Client,
    ClientConfig,
    ClientError,
    transaction::{send_danmaku_to_live::SendDanmakuToLive}
};


#[tokio::main]
async fn main() -> Result<(), ClientError>{
    let config = ClientConfig { 
        cookie_file: Some(Path::new("./examples/cookies.json"))
    };
    let client = Client::new(config)?;

    
    loop {
        if !client.is_online() {

        } else {
            loop {
                client.excute(SendDanmakuToLive { 
                    roomid: 5461071, 
                    danmaku: danmaku!("黑楼黑旗黑暗剑") 
                });
                // let d1 = danmaku!("黑楼黑旗黑暗剑");
                // let d2 = danmaku!(22470216=>765);
                // let d3 = danmaku!(official=>147);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

}
