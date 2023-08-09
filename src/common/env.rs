use std::env;

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
