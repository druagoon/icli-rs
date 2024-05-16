pub mod clash;
pub mod proxy;
pub mod quantumultx;

use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::include_template;

const DEFAULT_CONFIG: &str = include_template!("config/default.toml");
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
        let mut cb = ::config::Config::builder()
            .add_source(::config::File::from_str(DEFAULT_CONFIG, ::config::FileFormat::Toml))
            .add_source(::config::File::from_str(OS_DEFAULT_CONFIG, ::config::FileFormat::Toml));
        let mut cfg_files = Self::locate_config_files();
        cfg_files.reverse();
        for p in &cfg_files {
            cb = cb.add_source(::config::File::with_name(p.to_str().unwrap()).required(false));
        }
        let cfg = cb.build()?;
        Ok(cfg.try_deserialize::<Self>()?)
    }

    fn locate_config_files() -> Vec<PathBuf> {
        let mut files = vec![];
        let mut path = std::env::current_dir().unwrap();
        files.push(Self::get_config_file(&path));
        while let Some(parent) = path.parent() {
            let cf = Self::get_config_file(parent);
            files.push(cf);
            path.pop();
        }
        let default = Self::get_user_config_file();
        files.push(Path::new(&default).to_path_buf());
        files
    }

    pub fn get_config_file<T: AsRef<Path>>(p: T) -> PathBuf {
        let suffix = PathBuf::from_iter(Self::CONFIG_PATH_SUFFIX);
        p.as_ref().join(suffix)
    }

    pub fn get_user_config_file() -> String {
        let file = shellexpand::tilde(Self::USER_CONFIG_FILE);
        file.to_string()
    }
}

#[allow(dead_code)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("load config failed"));
