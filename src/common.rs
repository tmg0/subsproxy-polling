use reqwest::Client;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn load_subsproxy_url() -> String {
    return env::var("SUBSPROXY_URL").unwrap();
}

pub fn load_xray_config_filepath() -> String {
    return env::var("XRAY_CONFIG_FILEPATH").unwrap();
}

pub fn load_version() -> String {
    let v = option_env!("CARGO_PKG_VERSION").unwrap();
    return String::from(v);
}

pub async fn fetch_subsproxy_xray_config_text(url: &str) -> Result<String, String> {
    let client = Client::new();
    let version = load_version();
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

pub fn ensure_file(file_path: &str) {
    if fs::metadata(file_path).is_err() {
        ensure_dir(dirname(file_path));

        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(b"").unwrap();
    }
}

pub fn ensure_dir(dir_path: &str) {
    if !fs::metadata(dir_path).is_ok() {
        fs::create_dir_all(dir_path).unwrap();
    }
}

pub fn dirname(file_path: &str) -> &str {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        parent.to_str().unwrap_or("")
    } else {
        ""
    }
}
