
use bilibili_client::{reqwest_client::*, model::dynamic::DynamicCard};
#[tokio::main]
async fn main() {
    let reqwest_client = ReqwestClient::new();
    // 接下来一秒测试缓存
    let result = reqwest_client.get_live_info(7706705).await;
    dbg!(result);
}