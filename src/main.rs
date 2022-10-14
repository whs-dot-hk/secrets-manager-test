use aws_config::meta::region::RegionProviderChain;
use aws_sdk_secretsmanager as secretsmanager;

#[tokio::main]
async fn main() -> Result<(), secretsmanager::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = secretsmanager::Client::new(&config);

    let response = client.get_secret_value().secret_id("test").send().await?;

    println!("{:?}", response.secret_string);

    Ok(())
}
