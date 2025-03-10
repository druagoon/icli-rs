#![cfg(feature = "quantumultx")]

use std::fs;
use std::path::PathBuf;

use crate::commands::vpn::makeconfig::VpnMakeConfigCmd;
use crate::commands::vpn::VpnConfigGenerator;
use crate::config::Config;
use crate::prelude::*;

const QUANTUMULTX_CONF: &str = "QuantumultX.conf";
const QUANTUMULTX_TEMPLATE: &str = "vpn/quantumultx/QuantumultX.conf";

#[allow(dead_code)]
pub struct QuantumultXConfig<'a> {
    cmd: &'a VpnMakeConfigCmd,
}

impl<'a> QuantumultXConfig<'a> {
    #[allow(dead_code)]
    pub fn new(cmd: &'a VpnMakeConfigCmd) -> Self {
        Self { cmd }
    }
}

impl QuantumultXConfig<'_> {
    fn get_default_template(&self) -> Option<PathBuf> {
        let files = Config::locate_template_paths(QUANTUMULTX_TEMPLATE);
        files.into_iter().find(|x| x.exists())
    }

    fn get_template(&self) -> Option<PathBuf> {
        self.cmd.get_template().or(self.get_default_template())
    }

    fn get_template_engine(&self) -> anyhow::Result<tera::Tera> {
        let path = self.get_template().ok_or(anyhow::format_err!(
            "could not find the config template file for quantumultx"
        ))?;
        let mut engine = tera::Tera::default();
        engine.add_template_file(path, Some(QUANTUMULTX_CONF))?;
        Ok(engine)
    }
}

impl VpnConfigGenerator for QuantumultXConfig<'_> {
    fn make(&self) -> anyhow::Result<()> {
        let engine = self.get_template_engine()?;
        let output_dir = self.cmd.get_output_dir();
        if !output_dir.is_dir() {
            fs::create_dir_all(&output_dir)?
        }
        let output = output_dir.join(QUANTUMULTX_CONF);
        let fp = fs::File::create(&output)?;
        let mut ctx = tera::Context::new();
        ctx.insert("quantumultx", &CONFIG.quantumultx);
        engine.render_to(QUANTUMULTX_CONF, &ctx, fp)?;
        log::info!("write {:?} ... done", output);
        Ok(())
    }
}
