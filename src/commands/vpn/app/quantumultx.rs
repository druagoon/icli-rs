#![cfg(feature = "quantumultx")]

use std::fs;

use crate::commands::vpn::makeconfig::VpnMakeConfigCmd;
use crate::commands::vpn::VpnConfigGenerator;
use crate::prelude::*;

const QUANTUMULTX_CONFIG_NAME: &str = "QuantumultX.conf";
const QUANTUMULTX_TEMPLATES: [(&str, &str); 1] =
    [(QUANTUMULTX_CONFIG_NAME, include_template!("vpn/quantumultx/QuantumultX.conf"))];

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

impl<'a> VpnConfigGenerator for QuantumultXConfig<'a> {
    fn make(&self) -> anyhow::Result<()> {
        let engine = get_template_engine()?;
        let output_dir = self.cmd.get_output_dir().unwrap_or(std::env::current_dir()?);
        if !output_dir.is_dir() {
            fs::create_dir_all(&output_dir)?
        }
        let output = output_dir.join(QUANTUMULTX_CONFIG_NAME);
        let fp = fs::File::create(&output)?;
        let mut ctx = tera::Context::new();
        ctx.insert("quantumultx", &CONFIG.quantumultx);
        engine.render_to(QUANTUMULTX_CONFIG_NAME, &ctx, fp)?;
        log::info!("write {:?} ... done", output);
        Ok(())
    }
}

fn get_template_engine() -> tera::Result<tera::Tera> {
    let mut engine = tera::Tera::default();
    engine.add_raw_templates(QUANTUMULTX_TEMPLATES)?;
    Ok(engine)
}
