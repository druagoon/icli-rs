#![cfg(feature = "quantumultx")]

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuantumultX {
    pub remote: Remote,
    pub mitm: Mitm,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Remote {
    pub server: Server,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Server {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub img_url: String,
    #[serde(default)]
    pub interval: u32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Mitm {
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub p12: String,
}
