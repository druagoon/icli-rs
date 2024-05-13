use clap;
use icli_derive;

use crate::prelude::*;

mod ps1;

/// Ip utilities.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum IpCmd {
    Ps1(ps1::IpPs1Cmd),
}
