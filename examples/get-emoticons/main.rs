use std::{path::Path};
use bilibili_client::{
    danmaku, 
    Client,
    ClientConfig,
    ClientError,
    api::live::emoticon::get_emoticons::{GetEmoticons, GetEmoticonsRequest},
    api::Api,
    transaction::{
        send_danmaku_to_live::*,
        login::*
    }
};

#[tokio::main]
async fn main() -> Result<(), ClientError>{
    let config = ClientConfig { 
        cookie_file: Some(Path::new("./examples/cookies.json"))
    };
    let client = Client::new(config)?;
    let resp = client.as_ref().form_req::<GetEmoticons>(GetEmoticonsRequest::new_pc(21452505)).await.unwrap();
    for group in resp.data.data {
        println!("[{}]:", group.pkg_name);
        for emoticon in group.emoticons {
            println!("{:?}", emoticon)
        }
    }
    Ok(())
}
