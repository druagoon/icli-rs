mod completion;
mod config;
mod ip;
mod makefile;
mod new;
mod shinc;
mod vpn;

use crate::prelude::*;

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Config(config::ConfigCmd),
    #[command(subcommand)]
    Ip(ip::IpCmd),
    #[command(subcommand)]
    Makefile(makefile::MakefileCmd),
    #[command(subcommand)]
    Shinc(shinc::ShincCmd),
    #[cfg(feature = "vpn")]
    #[command(subcommand)]
    Vpn(vpn::VpnCmd),
    Completion(completion::CompletionCmd),
    New(new::NewCmd),
}
