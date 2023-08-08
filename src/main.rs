use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("SUBSPROXY_URL").unwrap();
    let client = reqwest::Client::new();
    let version: &str = option_env!("CARGO_PKG_VERSION").unwrap();
    let ua = format!("{}/{}", "Subsproxy", version);
    let response = client.get(&url).header("User-Agent", ua).send().await?;

    if response.status().is_success() {
        let servers = response.text().await?;

        println!("{}", servers);
    }

    Ok(())
}
