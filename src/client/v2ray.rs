#[derive(Debug)]
pub struct V2rayConfigInbound {
  port: i32,
  protocol: String
}

#[derive(Debug)]
pub struct V2rayConfig {
  inbounds: Vec<V2rayConfigInbound>
}

pub fn parse_v2ray_config (text: String) -> V2rayConfig {
  return V2rayConfig {
    inbounds: Vec::from([
      V2rayConfigInbound { port: 1080, protocol: String::from("vmess") }
    ])
  }
}
