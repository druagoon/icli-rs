#![cfg(all(target_os = "macos", feature = "clashx"))]

use std::fs;
use std::io::Write;
use std::path::Path;

use regex::Regex;
use reqwest::blocking::Client;

use super::{ClashProfile, VpnClashConfigGenerator};
use crate::commands::vpn::makeconfig::VpnMakeConfigCmd;
use crate::commands::vpn::VpnConfigGenerator;
use crate::config::clash::{ClashAppName, ClashProxyProvider};

pub struct ClashXConfig<'a> {
    cmd: &'a VpnMakeConfigCmd,
}

impl<'a> ClashXConfig<'a> {
    pub fn new(cmd: &'a VpnMakeConfigCmd) -> Self {
        Self { cmd }
    }
}

impl<'a> VpnConfigGenerator for ClashXConfig<'a> {
    fn make(&self) -> anyhow::Result<()> {
        self.make_profile()?;
        Ok(())
    }
}

impl<'a> VpnClashConfigGenerator for ClashXConfig<'a> {
    fn get_cmd(&self) -> &VpnMakeConfigCmd {
        self.cmd
    }

    fn get_app_name(&self) -> &'static ClashAppName {
        &ClashAppName::ClashX
    }

    fn get_profile(&self, primary: &ClashProxyProvider) -> anyhow::Result<ClashProfile> {
        let name = self.get_profile_name(primary);
        let cfg_dir = self.get_app_config_dir()?;
        let mut path = cfg_dir.join(&name);
        path.set_extension("yaml");
        Ok(ClashProfile::new(name, path))
    }

    fn fetch_rule<P: AsRef<Path>>(
        &self,
        client: &Client,
        url: &str,
        filepath: P,
    ) -> anyhow::Result<()> {
        let response = client.get(url).send()?;
        let bytes = response.bytes()?;
        let re = Regex::new(r"(?ms)^\s+-\s+IP-ASN,.+")?;
        let buf = re.replace_all(std::str::from_utf8(&bytes)?, "");
        let file_dir = filepath.as_ref().parent().unwrap();
        if !file_dir.is_dir() {
            fs::create_dir_all(file_dir)?;
        }
        let mut fp = fs::File::create(&filepath)?;
        fp.write_all(buf.as_bytes())?;
        Ok(())
    }
}
