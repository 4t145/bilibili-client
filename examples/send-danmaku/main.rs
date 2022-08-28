use bilibili_client::{
    Client,
    logger::{
        LogLevel,
        stdout_logger::StdoutLogger
    },
    api::live::msg::LiveDanmaku,
};


#[tokio::main]
async fn main() {
    println!("[BILIBILI CLIENT]");
    let logger = StdoutLogger::new();
    let mut client = Client::new(logger);
    client.logger.set_level(LogLevel::Info);
    loop {
        match client.login().await {
            Ok(result) => {
                if result == true {
                    loop {
                        let danmaku = LiveDanmaku::Text("黑楼黑旗黑暗剑".to_owned());
                        match client.send_danmaku_to_live(5461071, danmaku).await {
                            Ok(resp) => {
                                client.info(resp.to_string())
                            }
                            Err(e) => {
                                client.warn(format!("解析响应失败{e:?}"))
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
                    }
                } else {
                    client.warn("fail to get login info")
                }
            }
            Err(e) => {
                client.warn(format!("{e:?}"))
            }
        } 
        client.warn("sleep 1 sec");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
