use bilibili_client::{grpc::*, model::dynamic::DynamicCard, tonic_client::TonicClient};
#[tokio::main]
async fn main() {
    use bilibili::app::space::v1::{space_client::SpaceClient, ArchiveReply, ArchiveReq};
    let mut tonic_client = TonicClient::new();
    let client = SpaceClient::connect(TonicClient::GRPC_RAW_HOST_URL)
        .await
        .unwrap();
    let mut request = tonic_client.gen_request(ArchiveReq {
        vmid: todo!(),
        pn: todo!(),
        ps: todo!(),
        order: todo!(),
    });
    let resp = client.archive(request).await.unwrap();
}
