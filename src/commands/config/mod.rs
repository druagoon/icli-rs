mod list;
mod show;

use crate::prelude::*;

/// Manage local and global configuration.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum ConfigCmd {
    List(list::ConfigListCmd),
    Show(show::ConfigShowCmd),
}
