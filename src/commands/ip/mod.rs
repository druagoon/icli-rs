mod ps1;

use crate::prelude::*;

/// Ip utilities.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum IpCmd {
    Ps1(ps1::IpPs1Cmd),
}
