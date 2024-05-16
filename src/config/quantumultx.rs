#![cfg(feature = "quantumultx")]

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuantumultX {
    pub remote: QuantumultXSection,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuantumultXSection {
    pub server: QuantumultXSectionServer,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuantumultXSectionServer {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub img_url: String,
    #[serde(default)]
    pub interval: u32,
}
