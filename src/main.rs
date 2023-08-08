mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = common::load_subsproxy_url();
    let servers = common::fetch_subsproxy_xray_config_text(&url).await?;
    println!("{}", servers);

    Ok(())
}
