#![cfg(feature = "clash")]

pub mod clash_verge;
pub mod clashx;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use reqwest::blocking::Client;
use tera::Tera;

use crate::commands::vpn::makeconfig::VpnMakeConfigCmd;
use crate::commands::vpn::VpnConfigGenerator;
use crate::config::clash::{ClashAppName, ClashProxyProvider};
use crate::prelude::*;

const CLASH_PROVIDER_PROFILE_TEMPLATE_NAME: &str = "profile.provider.yaml";
const CLASH_PROVIDER_PROFILE_TEMPLATE: &str = include_template!("vpn/clash/profile.provider.yaml");

#[allow(dead_code)]
#[derive(Debug)]
pub struct ClashProfile {
    name: String,
    path: PathBuf,
}

#[allow(dead_code)]
trait VpnClashConfigGenerator: VpnConfigGenerator {
    fn get_cmd(&self) -> &VpnMakeConfigCmd;

    fn get_request_client() -> reqwest::Result<Client> {
        let mut cb = Client::builder();
        let ua = CONFIG.get_clash_user_agent();
        if !ua.is_empty() {
            cb = cb.user_agent(ua);
        }
        let all_proxy = CONFIG.get_all_proxy();
        if !all_proxy.is_empty() {
            cb = cb.proxy(reqwest::Proxy::all(all_proxy)?);
        }
        cb.build()
    }

    fn get_template_engine() -> tera::Result<Tera> {
        let mut engine = Tera::default();
        engine.add_raw_templates(vec![(
            CLASH_PROVIDER_PROFILE_TEMPLATE_NAME,
            CLASH_PROVIDER_PROFILE_TEMPLATE,
        )])?;
        Ok(engine)
    }

    fn get_app_name() -> &'static ClashAppName;

    fn get_app_cfg_dir() -> anyhow::Result<PathBuf> {
        let app_name = Self::get_app_name();
        CONFIG
            .get_clash_app_cfg_dir(app_name)
            .to_owned()
            .ok_or(anyhow::format_err!("clash app config directory is not set"))
    }

    fn get_primary_proxy() -> anyhow::Result<&'static ClashProxyProvider> {
        CONFIG
            .get_clash_primary_proxy_provider()
            .ok_or(anyhow::format_err!("clash primary proxy provider not found"))
    }

    fn get_profile_name(primary: &ClashProxyProvider) -> String {
        format!("{}X", primary.name)
    }

    fn get_profile(primary: &ClashProxyProvider) -> anyhow::Result<ClashProfile>;

    fn make_profile(&self) -> anyhow::Result<()> {
        let primary = Self::get_primary_proxy()?;
        let profile = Self::get_profile(primary)?;
        log::info!("{:?}", &profile);
        let pd = profile.path.parent().unwrap();
        if !pd.is_dir() {
            fs::create_dir_all(pd)?;
        }
        let fp = fs::File::create(profile.path)?;
        let remote = self.make_remote_profile(primary)?;
        serde_yaml::to_writer(&fp, &remote)?;
        let local = self.make_local_profile(primary)?;
        serde_yaml::to_writer(&fp, &local)?;
        Ok(())
    }

    fn make_remote_profile(
        &self,
        primary: &ClashProxyProvider,
    ) -> anyhow::Result<serde_yaml::Mapping> {
        let client = Self::get_request_client()?;
        let resp = client.get(&primary.url).send()?;
        let mut block: serde_yaml::Mapping = serde_yaml::from_reader(resp)?;

        // Remove yaml unneeded keys
        let yaml_unneeded_keys = vec!["proxies", "proxy-groups", "rules"];
        remove_yaml_block_keys(&mut block, &yaml_unneeded_keys);
        Ok(block)
    }

    fn make_local_profile(
        &self,
        primary: &ClashProxyProvider,
    ) -> anyhow::Result<serde_yaml::Mapping> {
        let engine = Self::get_template_engine()?;
        let mut ctx = tera::Context::new();
        ctx.insert("clash", &CONFIG.clash);
        ctx.insert("primary", primary);
        let text = engine.render(CLASH_PROVIDER_PROFILE_TEMPLATE_NAME, &ctx)?;
        let mut block: serde_yaml::Mapping = serde_yaml::from_str(&text)?;
        let rule_providers = block.get_mut("rule-providers").unwrap();
        let rule_providers_kv: HashMap<String, ClashRuleProviderItem> =
            serde_yaml::from_value(rule_providers.clone()).unwrap();

        let cmd = self.get_cmd();
        if cmd.is_download_rules() {
            Self::make_rules(rule_providers_kv)?;
        }

        // Remove rule providers unneeded keys
        // let rule_provider_unneeded_keys = vec!["url", "interval"];
        // let rule_providers_map = rule_providers.as_mapping_mut().unwrap();
        // for (_, v) in rule_providers_map {
        //     let vm = v.as_mapping_mut().unwrap();
        //     remove_yaml_block_keys(vm, &rule_provider_unneeded_keys);
        // }

        // Remove yaml unneeded keys
        let yaml_unneeded_keys = vec!["proxy-providers-ref"];
        remove_yaml_block_keys(&mut block, &yaml_unneeded_keys);
        Ok(block)
    }

    fn make_rules(rule_providers_kv: HashMap<String, ClashRuleProviderItem>) -> anyhow::Result<()> {
        let cfg_dir = Self::get_app_cfg_dir()?;
        let client = Self::get_request_client()?;
        log::info!("starting download rules ...");
        for item in rule_providers_kv
            .values()
            .filter(|&item| !(item.url.is_empty() || item.path.is_empty()))
        {
            let target = std::path::absolute(cfg_dir.join(&item.path))?;
            println!("{} ==> {}", item.url, item.path);
            Self::fetch_rule(&client, &item.url, &target)?;
        }
        Ok(())
    }

    fn fetch_rule<P: AsRef<Path>>(client: &Client, url: &str, filepath: P) -> anyhow::Result<()> {
        let mut response = client.get(url).send()?;
        let file_dir = filepath.as_ref().parent().unwrap();
        if !file_dir.is_dir() {
            fs::create_dir_all(file_dir)?;
        }
        let mut fp = std::fs::File::create(&filepath)?;
        response.copy_to(&mut fp)?;
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ClashRuleProviderItem {
    #[serde(default, alias = "type")]
    type_: String,
    #[serde(default)]
    behavior: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    interval: u32,
}

fn remove_yaml_block_keys(block: &mut serde_yaml::Mapping, keys: &Vec<&str>) {
    for &k in keys {
        block.shift_remove(k);
    }
}
