
use bilibili_client::{reqwest_client::*, model::dynamic::DynamicCard};
#[tokio::main]
async fn main() {
    let reqwest_client = ReqwestClient::new();
    // 接下来一秒测试缓存
    println!(">>> 测试话题");

    let resp = reqwest_client.get_dynamic_by_topic("鲨画".into(), 0).await;
    match resp {
        Ok(resp) => {
            if let Some(data) = resp.data {
                for item in data.cards {
                    let card = serde_json::from_str::<DynamicCard>(&item.card);
                    match card {
                        Ok(card) => {
                            println!("{card:?}")
                        },
                        Err(e) => {
                            println!(">>> 错误");
                            println!("{}", e.to_string());
                            println!("{}", item.card);
                            println!("<<<");
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("错误");    
            dbg!(e);
        },
    }
    println!("<<<");    
}