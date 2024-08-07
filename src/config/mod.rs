pub mod clash;
pub mod proxy;
pub mod quantumultx;

use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::include_template;

pub const DEFAULT_CONFIG: &str = include_template!("config/default.toml");
#[cfg(target_os = "linux")]
const OS_DEFAULT_CONFIG: &str = include_template!("config/linux.toml");
#[cfg(target_os = "macos")]
const OS_DEFAULT_CONFIG: &str = include_template!("config/macos.toml");
#[cfg(target_os = "windows")]
const OS_DEFAULT_CONFIG: &str = include_template!("config/windows.toml");

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub proxy: self::proxy::Proxy,
    #[cfg(feature = "clash")]
    pub clash: self::clash::Clash,
    #[cfg(feature = "quantumultx")]
    pub quantumultx: self::quantumultx::QuantumultX,
}

impl Config {
    const CONFIG_PATH_SUFFIX: [&'static str; 2] = [".icli", "config.toml"];
    const USER_CONFIG_FILE: &'static str = "~/.config/icli/config.toml";

    pub fn new() -> anyhow::Result<Self> {
        let sources = vec![
            ::config::File::from_str(DEFAULT_CONFIG, ::config::FileFormat::Toml),
            ::config::File::from_str(OS_DEFAULT_CONFIG, ::config::FileFormat::Toml),
        ];
        let files: Vec<_> = Self::locate_config_files()
            .iter()
            .rev()
            .filter(|&x| x.exists())
            .map(|x| ::config::File::from(x.to_owned()).required(false))
            .collect();
        let cfg = ::config::Config::builder()
            .add_source(sources)
            .add_source(files)
            .build()?
            .try_deserialize::<Self>()?;
        Ok(cfg)
    }

    pub fn locate_config_files() -> Vec<PathBuf> {
        vec![Self::get_local_config_file(), Self::get_user_config_file()]
    }

    pub fn get_path_config_file<T: AsRef<Path>>(p: T) -> PathBuf {
        let suffix = PathBuf::from_iter(Self::CONFIG_PATH_SUFFIX);
        p.as_ref().join(suffix)
    }

    pub fn get_local_config_file() -> PathBuf {
        let cwd = std::env::current_dir().unwrap();
        Self::get_path_config_file(cwd)
    }

    pub fn get_user_config_file() -> PathBuf {
        let file = shellexpand::tilde(Self::USER_CONFIG_FILE);
        PathBuf::from(file.to_string())
    }
}

#[allow(dead_code)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("load config failed"));
