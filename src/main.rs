use reqwest::Client;
use tokio::time::{Duration, Interval};

mod common;
use common::{conf, fse, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = conf::load_config();
    let url = conf::load_subsproxy_url(&conf);
    let xray_config_filepath = conf::load_xray_config_filepath(&conf);
    let cmd = conf::load_command(&conf);

    let interval_duration = Duration::from_secs(60 * 60);
    let mut interval: Interval = tokio::time::interval(interval_duration);

    loop {
        interval.tick().await;
        let servers = fetch_subsproxy_xray_config_text(url).await?;
        let file = fse::ensure_file(xray_config_filepath);
        fse::write(file, &servers);
        
        if !cmd.is_empty() {
            process::exec(cmd).unwrap();
        }   
    }
}

async fn fetch_subsproxy_xray_config_text(url: &str) -> Result<String, String> {
    let client = Client::new();
    let version = conf::load_version();
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
