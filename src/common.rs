use reqwest::Client;

pub fn load_version () -> &'static str {
  return option_env!("CARGO_PKG_VERSION").unwrap();
}

pub async fn fetch_subsproxy_xray_config_text (url: &str) -> Result<String, String> {
  let client = Client::new();
  let version = load_version();
  let ua = format!("{}/{}", "Subsproxy", version);
  let response = client.get(url).header("User-Agent", ua).send().await.unwrap();

  if response.status().is_success() {
    let servers = response.text().await.unwrap();

    Ok(servers)
  } else {
    Err(String::from("{}"))
  }
}
