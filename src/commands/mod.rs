mod completion;
mod ip;
mod new;

use crate::prelude::*;

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Ip(ip::IpCmd),
    Completion(completion::CompletionCmd),
    New(new::NewCmd),
}
