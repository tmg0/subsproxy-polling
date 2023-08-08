use dotenv::dotenv;

mod common;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = common::load_subsproxy_url();

    let servers = common::fetch_subsproxy_xray_config_text(&url)
        .await
        .unwrap();

    println!("{}", servers);

    let xray_config_filepath = common::load_xray_config_filepath();

    common::ensure_file(&xray_config_filepath);

    Ok(())
}
