mod completion;
mod ip;
mod makefile;
mod new;
mod vpn;

use crate::prelude::*;

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Ip(ip::IpCmd),
    #[command(subcommand)]
    Makefile(makefile::MakefileCmd),
    #[cfg(feature = "vpn")]
    #[command(subcommand)]
    Vpn(vpn::VpnCmd),
    Completion(completion::CompletionCmd),
    New(new::NewCmd),
}
