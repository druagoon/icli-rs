#![cfg(feature = "clash")]

pub mod clash_verge;
pub mod clashx;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use reqwest::blocking::Client;

use crate::commands::vpn::makeconfig::VpnMakeConfigCmd;
use crate::commands::vpn::VpnConfigGenerator;
use crate::config::clash::{ClashAppName, ClashProxyProvider};
use crate::config::Config;
use crate::prelude::*;

const CLASH_PROVIDER_PROFILE_TEMPLATE: &str = "vpn/clash/profile.provider.yaml";

#[allow(dead_code)]
#[derive(serde::Serialize, Debug)]
pub struct ClashContext<'a> {
    app: ClashContextApp,
    profile: ClashProfile,
    primary: &'a ClashProxyProvider,
}

impl<'a> ClashContext<'a> {
    #[allow(dead_code)]
    pub fn new(
        app: ClashContextApp,
        profile: ClashProfile,
        primary: &'a ClashProxyProvider,
    ) -> Self {
        Self { app, profile, primary }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize, Debug)]
pub struct ClashContextApp {
    name: String,
}

impl ClashContextApp {
    #[allow(dead_code)]
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize, Debug)]
pub struct ClashProfile {
    name: String,
    path: PathBuf,
}

impl ClashProfile {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }
}

#[allow(dead_code)]
trait VpnClashConfigGenerator: VpnConfigGenerator {
    fn get_cmd(&self) -> &VpnMakeConfigCmd;

    fn get_default_template(&self) -> Option<PathBuf> {
        let files = Config::locate_template_files(CLASH_PROVIDER_PROFILE_TEMPLATE);
        files.into_iter().find(|x| x.exists())
    }

    fn get_template(&self) -> Option<PathBuf> {
        self.get_cmd().get_template().or(self.get_default_template())
    }

    fn get_template_engine(&self) -> anyhow::Result<tera::Tera> {
        let path = self
            .get_template()
            .ok_or(anyhow::format_err!("could not find the config template file for clash"))?;
        let mut engine = tera::Tera::default();
        engine.add_template_file(path, Some(CLASH_PROVIDER_PROFILE_TEMPLATE))?;
        Ok(engine)
    }

    fn get_request_client(&self) -> reqwest::Result<Client> {
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

    fn get_app_name(&self) -> &'static ClashAppName;

    fn get_app_config_dir(&self) -> anyhow::Result<PathBuf> {
        let app_name = self.get_app_name();
        CONFIG
            .get_clash_app_config_dir(app_name)
            .to_owned()
            .ok_or(anyhow::format_err!("clash app config directory is not set"))
    }

    fn get_primary_proxy(&self) -> anyhow::Result<&'static ClashProxyProvider> {
        CONFIG
            .get_clash_primary_proxy_provider()
            .ok_or(anyhow::format_err!("clash primary proxy provider not found"))
    }

    fn get_profile_name(&self, primary: &ClashProxyProvider) -> String {
        format!("{}X", primary.name)
    }

    fn get_profile(&self, primary: &ClashProxyProvider) -> anyhow::Result<ClashProfile>;

    fn get_context(&self) -> anyhow::Result<ClashContext> {
        let primary = self.get_primary_proxy()?;
        let app = ClashContextApp::new(self.get_app_name().to_string());
        let profile = self.get_profile(primary)?;
        Ok(ClashContext::new(app, profile, primary))
    }

    fn make_profile(&self) -> anyhow::Result<()> {
        let ctx = self.get_context()?;
        log::info!("clash context: {}", serde_json::to_string(&ctx)?);
        let profile_dir = ctx.profile.path.parent().unwrap();
        if !profile_dir.is_dir() {
            fs::create_dir_all(profile_dir)?;
        }
        let fp = fs::File::create(&ctx.profile.path)?;
        let remote = self.make_remote_profile(&ctx)?;
        serde_yaml::to_writer(&fp, &remote)?;
        let local = self.make_local_profile(&ctx)?;
        serde_yaml::to_writer(&fp, &local)?;
        Ok(())
    }

    fn make_remote_profile(&self, ctx: &ClashContext) -> anyhow::Result<serde_yaml::Mapping> {
        let client = self.get_request_client()?;
        let resp = client.get(&ctx.primary.url).send()?;
        let mut block: serde_yaml::Mapping = serde_yaml::from_reader(resp)?;

        // Remove yaml unneeded keys
        let yaml_unneeded_keys = vec!["proxies", "proxy-groups", "rules"];
        remove_yaml_block_keys(&mut block, &yaml_unneeded_keys);
        Ok(block)
    }

    fn make_local_profile(&self, ctx: &ClashContext) -> anyhow::Result<serde_yaml::Mapping> {
        let engine = self.get_template_engine()?;
        let mut tpl_ctx = tera::Context::new();
        tpl_ctx.insert("clash", &CONFIG.clash);
        tpl_ctx.insert("primary", ctx.primary);
        let text = engine.render(CLASH_PROVIDER_PROFILE_TEMPLATE, &tpl_ctx)?;
        let mut block: serde_yaml::Mapping = serde_yaml::from_str(&text)?;
        let rule_providers = block.get_mut("rule-providers").unwrap();
        let rule_providers_kv: HashMap<String, ClashRuleProviderItem> =
            serde_yaml::from_value(rule_providers.clone()).unwrap();

        let cmd = self.get_cmd();
        if cmd.is_download_rules() {
            self.make_rules(rule_providers_kv)?;
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

    fn make_rules(
        &self,
        rule_providers_kv: HashMap<String, ClashRuleProviderItem>,
    ) -> anyhow::Result<()> {
        let cfg_dir = self.get_app_config_dir()?;
        let client = self.get_request_client()?;
        println!("starting download rules ...");
        for item in rule_providers_kv
            .values()
            .filter(|&item| !(item.url.is_empty() || item.path.is_empty()))
        {
            let target = std::path::absolute(cfg_dir.join(&item.path))?;
            println!("{} ==> {}", item.url, item.path);
            self.fetch_rule(&client, &item.url, &target)?;
        }
        Ok(())
    }

    fn fetch_rule<P: AsRef<Path>>(
        &self,
        client: &Client,
        url: &str,
        filepath: P,
    ) -> anyhow::Result<()> {
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
