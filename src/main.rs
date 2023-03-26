use rusoto_sts::{GetCallerIdentityRequest, Sts, StsClient};

#[tokio::main]
async fn main() {
    let client = StsClient::new(rusoto_core::Region::default());
    let identity = client.get_caller_identity(GetCallerIdentityRequest{}).await;
    println!("default chain says: {:?}", identity);

    let provider = rusoto_sts::WebIdentityProvider::from_k8s_env();
    let provider = rusoto_credential::AutoRefreshingProvider::new(provider)?;
    let client = StsClient::new_with(rusoto_core::HttpClient::new().unwrap(),
    provider, rusoto_core::Region::default());
    let identity = client.get_caller_identity(GetCallerIdentityRequest{}).await;
    println!("default chain says: {:?}", identity)
}
