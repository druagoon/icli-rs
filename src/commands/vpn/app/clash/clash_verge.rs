#![cfg(feature = "clash_verge")]

use std::path::Path;

use super::{ClashProfile, VpnClashConfigGenerator};
use crate::commands::vpn::makeconfig::VpnMakeConfigCmd;
use crate::commands::vpn::VpnConfigGenerator;
use crate::config::clash::{ClashAppName, ClashProxyProvider};

#[allow(dead_code)]
pub struct ClashVergeConfig<'a> {
    cmd: &'a VpnMakeConfigCmd,
}

impl<'a> ClashVergeConfig<'a> {
    pub fn new(cmd: &'a VpnMakeConfigCmd) -> Self {
        Self { cmd }
    }
}

impl<'a> VpnConfigGenerator for ClashVergeConfig<'a> {
    fn make(&self) -> anyhow::Result<()> {
        self.make_profile()?;
        Ok(())
    }
}

impl<'a> VpnClashConfigGenerator for ClashVergeConfig<'a> {
    fn get_cmd(&self) -> &VpnMakeConfigCmd {
        self.cmd
    }

    fn get_app_name() -> &'static ClashAppName {
        &ClashAppName::ClashVerge
    }

    fn get_profile(primary: &ClashProxyProvider) -> anyhow::Result<ClashProfile> {
        let name = Self::get_profile_name(primary);
        let cfg_dir = Self::get_app_config_dir()?;
        let profiles = cfg_dir.join("profiles.yaml");
        let prof = Profiles::new(profiles);
        // let item = prof.get_item_by_name(&name);
        // let utc_now = chrono::Utc::now();
        // let ts = utc_now.timestamp().to_string();
        // let filename = item.map_or(ts.as_str(), |v| &v.file);
        let filename = prof.get_filename_by_name(&name).expect("profile filename not found");
        let mut path = cfg_dir.join("profiles").join(filename);
        path.set_extension("yaml");
        Ok(ClashProfile { name, path })
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Profiles {
    #[serde(default)]
    current: String,
    items: Vec<ProfileItem>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ProfileItem {
    #[serde(default)]
    uid: String,
    #[serde(default, alias = "type")]
    type_: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    file: String,
    #[serde(default)]
    desc: String,
}

impl Profiles {
    fn new<T: AsRef<Path>>(path: T) -> Self {
        let cb = ::config::Config::builder()
            .add_source(::config::File::with_name(path.as_ref().to_str().unwrap()).required(false))
            .build()
            .expect("config build error");
        cb.try_deserialize::<Self>().expect("profiles deserialize error")
    }

    fn get_item_by_name(&self, name: &str) -> Option<&ProfileItem> {
        self.items.iter().find(|&item| item.name.as_ref().is_some_and(|x| x == name))
    }

    fn get_filename_by_name(&self, name: &str) -> Option<&String> {
        self.get_item_by_name(name).map(|v| &v.file)
    }
}
