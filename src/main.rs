use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("SUBSPROXY_URL").unwrap();
    let servers = reqwest::get(&url).await?.text().await?;

    println!("{}", servers);

    Ok(())
}
