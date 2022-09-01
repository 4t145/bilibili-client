use std::path::Path;
use bilibili_client::{
    danmaku, 
    Client,
    ClientConfig,
    ClientError,
    logger::{
        LogLevel,
        stdout_logger::StdoutLogger
    },
};


#[tokio::main]
async fn main() -> Result<(), ClientError>{
    let config = ClientConfig { 
        logger: StdoutLogger::new(), 
        cookie_file: Some(Path::new("./examples/cookies.json"))
    };
    let mut client = Client::new(config)?;
    client.logger.set_level(LogLevel::Info);
    
    loop {
        if !client.is_online() {
            client.login().await?;
        } else {
            loop {
                // let d1 = danmaku!("黑楼黑旗黑暗剑");
                // let d2 = danmaku!(22470216=>765);
                // let d3 = danmaku!(official=>147);
                match client.send_danmaku_to_live(5461071, danmaku!("黑楼黑旗黑暗剑")).await {
                    Ok(resp) => {
                        client.info(resp.to_string())
                    }
                    Err(e) => {
                        client.warn(format!("解析响应失败{e:?}"))
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
            }
        }
        client.warn("sleep 1 sec");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
