#![cfg(feature = "quantumultx")]

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuantumultX {
    pub remote: QuantumultXSection,
    pub mitm: QuantumultXSectionMITM,
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

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QuantumultXSectionMITM {
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub p12: String,
}
