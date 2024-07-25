#![cfg(feature = "clash")]

use std::path::PathBuf;

#[allow(unused_imports)]
use crate::de::deserialize_path;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Clash {
    pub user_agent: String,
    pub proxy: ClashProxy,
    pub app: ClashApp,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClashProxy {
    pub providers: Vec<ClashProxyProvider>,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClashProxyProvider {
    #[serde(default)]
    pub primary: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub interval: u32,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClashApp {
    #[cfg(all(target_os = "macos", feature = "clashx"))]
    pub clashx: ClashAppClashX,
    #[cfg(feature = "clash_verge")]
    pub clash_verge: ClashAppClashVerge,
}

#[cfg(all(target_os = "macos", feature = "clashx"))]
#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClashAppClashX {
    #[serde(default, deserialize_with = "deserialize_path")]
    pub config_dir: Option<PathBuf>,
}

#[cfg(feature = "clash_verge")]
#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ClashAppClashVerge {
    #[serde(default, deserialize_with = "deserialize_path")]
    pub config_dir: Option<PathBuf>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ClashAppName {
    /// ClashX
    #[cfg(all(target_os = "macos", feature = "clashx"))]
    ClashX,
    /// Clash.Verge
    #[cfg(feature = "clash_verge")]
    ClashVerge,
}

impl ClashAppName {
    pub fn as_str(&self) -> &'static str {
        match self {
            #[cfg(all(target_os = "macos", feature = "clashx"))]
            Self::ClashX => "ClashX",
            #[cfg(feature = "clash_verge")]
            Self::ClashVerge => "Clash.Verge",
            #[allow(unreachable_patterns)]
            _ => "",
        }
    }
}

impl std::fmt::Display for ClashAppName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

impl super::Config {
    #[allow(dead_code)]
    pub fn get_clash_user_agent(&self) -> &String {
        &self.clash.user_agent
    }

    #[allow(dead_code)]
    pub fn get_clash_primary_proxy_provider(&self) -> Option<&ClashProxyProvider> {
        self.clash.proxy.providers.iter().find(|&v| v.primary)
    }

    #[allow(dead_code)]
    pub fn get_clash_app_config_dir(&self, name: &ClashAppName) -> &Option<PathBuf> {
        match name {
            #[cfg(all(target_os = "macos", feature = "clashx"))]
            ClashAppName::ClashX => &self.clash.app.clashx.config_dir,
            #[cfg(feature = "clash_verge")]
            ClashAppName::ClashVerge => &self.clash.app.clash_verge.config_dir,
            #[allow(unreachable_patterns)]
            _ => &None,
        }
    }
}
