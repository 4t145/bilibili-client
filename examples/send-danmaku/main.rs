use bilibili_client::{
    client::Client,
    logger::{
        LogLevel,
        stdout_logger::StdoutLogger
    },
    api::live::msg::send::LiveDanmaku
};


#[tokio::main]
async fn main() {
    println!("[BILIBILI CLIENT]");
    let logger = StdoutLogger::new();
    let mut client = Client::new(logger);
    client.logger.set_level(LogLevel::Debug);
    loop {
        match client.login().await {
            Ok(result) => {
                if result == true {
                    loop {
                        let danmaku = LiveDanmaku::Emoticon("room_21452505_191".to_owned());
                        match client.send_danmaku_to_live(5461071, danmaku).await {
                            Ok(resp) => {
                                client.info(resp.to_string().as_str())
                            }
                            Err(e) => {
                                client.warn(format!("解析响应失败{e:?}").as_str())
                            }
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await
                    }
                } else {
                    client.warn("fail to get login info")
                }
            }
            Err(e) => {
                client.warn(format!("{e:?}").as_str())
            }
        } 
        client.warn("sleep 1 sec");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
