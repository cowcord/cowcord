use serde::{Deserialize, Serialize};

pub struct GatewayRecieveEvent {}

#[derive(Serialize, Deserialize)]
pub enum GatewayEncoding {
    json,
    etf,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GatewayCompression {
    zlib_stream,
}