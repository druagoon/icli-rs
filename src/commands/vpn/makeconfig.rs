use std::path::PathBuf;

#[cfg(feature = "clash_verge")]
use super::app::clash::clash_verge::ClashVergeConfig;
#[cfg(all(target_os = "macos", feature = "clashx"))]
use super::app::clash::clashx::ClashXConfig;
#[cfg(feature = "quantumultx")]
use super::app::quantumultx::QuantumultXConfig;
use super::VpnConfigGenerator;
use crate::prelude::*;
use crate::utils::path::expand_tilde;

/// Make config for VPN app.
#[derive(clap::Parser, Debug)]
pub struct VpnMakeConfigCmd {
    /// VPN client app.
    #[arg(long, value_enum)]
    app: VpnApp,
    /// Path to the template file for generating config files.
    #[arg(short = 't', long)]
    pub template: Option<String>,
    /// Download clash provider rules to local.
    #[cfg(feature = "clash")]
    #[arg(long)]
    download_rules: bool,
    /// Write QuantumultX.conf to DIR.
    #[cfg(feature = "quantumultx")]
    #[arg(long, value_name = "DIR", default_value = "./output")]
    output_dir: String,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum VpnApp {
    /// ClashX
    #[cfg(all(target_os = "macos", feature = "clashx"))]
    #[value(name = "clashx")]
    ClashX,
    /// Clash.Verge
    #[cfg(feature = "clash_verge")]
    #[value(name = "clash.verge")]
    ClashVerge,
    /// QuantumultX
    #[cfg(feature = "quantumultx")]
    #[value(name = "quantumultx")]
    QuantumultX,
}

impl VpnMakeConfigCmd {
    #[allow(dead_code)]
    pub fn get_template(&self) -> Option<PathBuf> {
        self.template.as_ref().map(|x| expand_tilde(x))
    }

    #[allow(dead_code)]
    pub fn is_download_rules(&self) -> bool {
        self.download_rules
    }

    #[allow(dead_code)]
    pub fn get_output_dir(&self) -> PathBuf {
        expand_tilde(&self.output_dir)
    }
}

impl CliCommand for VpnMakeConfigCmd {
    fn run(&self) -> CliResult {
        match self.app {
            #[cfg(all(target_os = "macos", feature = "clashx"))]
            VpnApp::ClashX => ClashXConfig::new(self).make(),
            #[cfg(feature = "clash_verge")]
            VpnApp::ClashVerge => ClashVergeConfig::new(self).make(),
            #[cfg(feature = "quantumultx")]
            VpnApp::QuantumultX => QuantumultXConfig::new(self).make(),
            // #[allow(unreachable_patterns)]
            // _ => {
            //     println!("the client app is not yet supported");
            //     Ok(())
            // }
        }
    }
}
