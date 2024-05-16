#![cfg(feature = "vpn")]

mod app;
mod makeconfig;

use crate::prelude::*;

#[allow(dead_code)]
pub trait VpnConfigGenerator {
    fn make(&self) -> anyhow::Result<()>;
}

/// VPN utilities.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum VpnCmd {
    #[command(name = "makeconfig")]
    MakeConfig(makeconfig::VpnMakeConfigCmd),
}
