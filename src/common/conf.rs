use serde_derive::Deserialize;

use crate::fse::read_toml;

#[derive(Debug, Deserialize)]
pub struct Conf {
    subsproxy_url: String,
    xray_config_filepath: String,
}

pub fn load_config() -> Conf {
    let conf: Conf = read_toml("config.toml").unwrap();
    return conf;
}

pub fn load_subsproxy_url(conf: &Conf) -> &str {
    return &conf.subsproxy_url;
}

pub fn load_xray_config_filepath(conf: &Conf) -> &str {
    return &conf.xray_config_filepath;
}

pub fn load_version() -> String {
    let v = option_env!("CARGO_PKG_VERSION").unwrap();
    return String::from(v);
}
