mod genhelp;

use crate::prelude::*;

/// Makefile utilities.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum MakefileCmd {
    #[command(name = "genhelp")]
    GenHelp(genhelp::MakefileGenHelpCmd),
}
