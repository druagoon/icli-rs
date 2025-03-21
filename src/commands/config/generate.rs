use std::fs;
use std::path::PathBuf;

use crate::config::{Config, DEFAULT_CONFIG};
use crate::prelude::*;

/// Generate the configuration files.
#[derive(clap::Parser, Debug)]
pub struct ConfigGenerateCmd {
    /// Force overwrite even if the configuration file already exists.
    #[arg(long, default_value_t)]
    force: bool,
    /// Generate configuration file in the current directory.
    #[arg(long, default_value_t)]
    local: bool,
}

impl ConfigGenerateCmd {
    fn get_config_file_path(&self) -> PathBuf {
        if self.local {
            Config::local_config_path()
        } else {
            Config::user_config_path()
        }
    }
}

impl CliCommand for ConfigGenerateCmd {
    fn run(&self) -> CliResult {
        let cf = self.get_config_file_path();
        if self.force || !cf.exists() {
            if cf.exists() {
                let to = cf.with_extension("bak");
                fs::copy(&cf, &to)?;
                println!("backup config file: {} => {}", cf.display(), to.display());
            }
            let cf_dir = cf.parent().unwrap();
            if !cf_dir.exists() {
                fs::create_dir_all(cf_dir)?;
            }
            fs::write(cf, DEFAULT_CONFIG)?;
        } else {
            println!("config file already exists: {}", cf.display())
        }
        Ok(())
    }
}
