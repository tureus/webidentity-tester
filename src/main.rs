use rusoto_sts::{GetCallerIdentityRequest, Sts, StsClient};

#[tokio::main]
async fn main() {
    let client = StsClient::new(rusoto_core::Region::default());
    let identity = client.get_caller_identity(GetCallerIdentityRequest{}).await;
    println!("{:?}", identity);
}
