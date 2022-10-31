
use bilibili_client::awc_client::*;
use http_api_util::cache::{FifoCache, ApiCache};
use std::{time::*, sync::RwLock};
use actix_web;
#[actix_web::main]
async fn main() {
    let awc_client = AwcClient::new();
    // 接下来一秒测试缓存
    println!(">>> 测试缓存");
    let start_time = Instant::now();
    let secs = 10;
    let mut repeat_count = 0;
    let room_info_cache = RwLock::new(FifoCache::new(128));
    while Instant::now() - start_time < Duration::from_secs(secs) {
        repeat_count += 1;
        let req_start_time = Instant::now();
        let resp = awc_client.get_room_info_cached(59275, &room_info_cache).await.unwrap();
        let req_end_time = Instant::now();
        let msspan = (req_end_time - req_start_time).as_millis();
        if msspan > 10 {
            println!("第{repeat_count}次请求，用时{msspan:04}ms");
            if let Some(data) = resp.data {
                let title = data.live_room.title;
                let watched = data.live_room.watched_show.num;
                println!("{title}, {watched}人看过")
            }
        }

    }
    let fetch_per_time = repeat_count/secs;
    println!("获取速度 {fetch_per_time}次/s");
    println!("<<<");    
}