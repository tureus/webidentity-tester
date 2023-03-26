use rusoto_sts::{GetCallerIdentityRequest, Sts, StsClient};


#[tokio::main]
async fn main() {
    rusoto().await;

    aws_sdk().await;
}

async fn aws_sdk() {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_sts::Client::new(&shared_config);
    let req = client.get_caller_identity();
    let result = req.send().await;
    println!("identity: {:?}", result)
}

async fn rusoto() {
    let client = StsClient::new(rusoto_core::Region::default());
    let identity = client.get_caller_identity(GetCallerIdentityRequest{}).await;
    println!("default chain says: {:?}", identity);

    let provider = rusoto_sts::WebIdentityProvider::from_k8s_env();
    let provider = rusoto_credential::AutoRefreshingProvider::new(provider).unwrap();
    let client = StsClient::new_with(rusoto_core::HttpClient::new().unwrap(),
                                     provider, rusoto_core::Region::default());
    let identity = client.get_caller_identity(GetCallerIdentityRequest{}).await;
    println!("default chain says: {:?}", identity)
}