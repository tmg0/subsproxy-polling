use dotenv::dotenv;
use reqwest::Client;
use tokio::time::{Duration, Interval};

mod common;
use common::{env, fse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::load_subsproxy_url();
    let xray_config_filepath = env::load_xray_config_filepath();

    let interval_duration = Duration::from_secs(60 * 60);
    let mut interval: Interval = tokio::time::interval(interval_duration);

    loop {
        interval.tick().await;
        let servers = fetch_subsproxy_xray_config_text(&url).await?;
        let file = fse::ensure_file(&xray_config_filepath);
        fse::write(file, &servers);
    }
}

async fn fetch_subsproxy_xray_config_text(url: &str) -> Result<String, String> {
    let client = Client::new();
    let version = env::load_version();
    let ua = format!("{}/{}", "Subsproxy", version);
    let response = client
        .get(url)
        .header("User-Agent", ua)
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        Ok(response.text().await.unwrap())
    } else {
        Err(String::from(""))
    }
}
