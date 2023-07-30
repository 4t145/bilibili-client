use bilibili_client::{
    grpc::*,
    model::dynamic::DynamicCard,
    tonic_client::{LoginInfo, TonicClient},
};
#[tokio::main]
async fn main() {
    use bilibili::app::dynamic::v2::{dynamic_client::DynamicClient, DynSpaceReq, DynSpaceRsp};
    let mut tonic_client = TonicClient::new();
    tonic_client.set_login_info(LoginInfo::new(
        std::env::var("uid").unwrap().parse().unwrap(),
        std::env::var("ak").unwrap(),
    ));
    let mut client = DynamicClient::connect(TonicClient::GRPC_RAW_HOST_URL)
        .await
        .unwrap();
    let mut request = tonic_client.gen_request(DynSpaceReq {
        host_uid: 85181,
        local_time: 8,
        page: 1,
        from: "space".into(),
        ..Default::default()
    });
    let resp = client.dyn_space(request).await.unwrap();
    dbg!(&resp);
}
