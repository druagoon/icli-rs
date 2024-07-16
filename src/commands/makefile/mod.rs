mod add_help;

use crate::prelude::*;

/// Makefile utilities.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum MakefileCmd {
    AddHelp(add_help::MakefileAddHelpCmd),
}
