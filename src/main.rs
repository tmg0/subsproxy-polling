use dotenv::dotenv;
use std::env;

mod common;
use common::fetch_subsproxy_xray_config_text;

mod client;
use client::v2ray::parse_v2ray_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("SUBSPROXY_URL").unwrap();
    let servers = fetch_subsproxy_xray_config_text(&url).await?;
    let v2ray = parse_v2ray_config(servers);
    println!("{:?}", v2ray);

    Ok(())
}
